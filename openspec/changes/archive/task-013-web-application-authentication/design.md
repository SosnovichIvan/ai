## Контекст и границы

Регистрация и авторизация не могут быть реализованы только статическим
frontend: уникальность логина, пароль, session и данные сущностей должны
проверяться и храниться серверной стороной. Browser frontend никогда не
получает пароль в открытом виде после отправки, не хранит service secret и не
пишет в произвольные папки на компьютере пользователя.

Выбран self-hosted вариант: отдельный API, PostgreSQL, password hashing и
server-side session store. Frontend остаётся React/Vite, а API и PostgreSQL
разворачиваются вместе с web-приложением. Hosting, region, резервное
копирование PostgreSQL и production environment variables фиксируются в
deployment configuration до первого production deploy.

Пароли хешируются устойчивым password-hashing алгоритмом, cookies имеют
`HttpOnly`, `Secure`, `SameSite`, а доступ к каждой сущности фильтруется по
authenticated user id и Workspace membership.

## Сценарии

1. Неавторизованный посетитель лендинга видит ценность продукта и CTA входа
   или регистрации, но не download/OS selection.
2. Регистрация требует логин, пароль и подтверждение пароля; валидирует формат
   и уникальность логина, совпадение паролей на server side, затем создаёт
   account и default Workspace в одной transaction. При занятом логине account
   не создаётся и пользователь получает понятную ошибку.
3. После входа пользователь видит текущие разделы Environment и Tasks,
   собственные Workspace/Folders/Task/Template/TaskBlock/rules/skills.
4. Вход принимает существующие логин и пароль; выход инвалидирует сессию.
   Восстановление доступа использует одноразовый, ограниченный по времени
   token и не раскрывает существование логина.
5. Импорт rules/skills сохраняет preview, explicit merge/replace и backup, но
   browser не получает неограниченный доступ к локальному filesystem.

## Миграция desktop state

Выбрана стратегия **export/import**. До удаления Tauri создаётся проверяемый
локальный export legacy state. Пользователь явно подтверждает импорт в свой account;
данные не отправляются автоматически. Миграция сохраняет Workspace, Folder,
Task, Template, TaskBlock, revision, timestamps, `sourceTemplateId` и
`blockSnapshot`. Точный UX и Figma node-id обязательны до реализации.

## Figma gate

- **Status:** APPROVED
- **Figma:** https://www.figma.com/design/KYcd8hPuczEIpqK9dbG5D7/ai_works?node-id=55-64
- **Node:** `55:64`, `124:153`, `124:182`, `124:262`, `124:299`.
- **Рабочее пространство задач:** `32:62` (основной экран `33:62`; связанные
  списки `34:70`, `34:116`, `34:160`, формы `37:106`, `37:154`, `37:200` и
  состояния Workspace `103:118`).
- **Состав:** сохранённый landing с CTA «Войти», регистрация с уникальным
  логином, паролем и подтверждением пароля, вход по логину и паролю для
  desktop и mobile, а также действие выхода.
- **Подтверждено:** владельцем 31.08.2026. Существующие утверждённые
  desktop/landing макеты дополняются указанными экранами identity и session.

## Rollback и regression

До переключения production routing сохраняется обратимый staging deployment и
экспорт legacy state. При ошибке migration пользователь остаётся в исходном
local state, а server-side transaction откатывается. Regression suite обязана
проверять текущие flows Environment, import, Workspace, Folder, Task, Template,
TaskBlock, clear/share, а также register/login/logout/authorization/isolation
и responsive navigation.
