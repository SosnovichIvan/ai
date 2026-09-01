## 0. Решения и дизайн gate

- [x] 0.1 Выбрать server-side provider: self-hosted API/PostgreSQL. — Проверка: в design зафиксированы ownership, server-side session и запрет секретов в Git; hosting, region и backup уточняются в deployment configuration до production deploy.
- [x] 0.2 Согласовать Figma URL с `node-id` для всех новых экранов и responsive состояний. — Проверка: `design-approval.md` имеет `APPROVED`, URL и node-id.
- [x] 0.3 Утвердить стратегию legacy data: export/import или controlled migration. — Проверка: выбран export/import; определено, как сохранить Workspace, Folder, Task, Template, TaskBlock и связи без автоматической отправки данных.

## 1. Удаление desktop delivery

- [ ] 1.1 Удалить Tauri source, native bundle scripts и multi-platform release workflow. — Проверка: repository не содержит desktop build/release command, а web build проходит из lockfile.
- [ ] 1.2 Удалить с landing download CTA, OS detection, installer instructions и binary release catalog. — Проверка: public pages не предлагают скачивание и не содержат platform artifacts.
- [ ] 1.3 Обновить README, deployment config и current specs для web delivery. — Проверка: документация описывает запуск и публикацию web-приложения без desktop toolchain.

## 2. Identity и данные

- [ ] 2.1 Реализовать server-side registration, login, logout, password reset и session lifecycle по уникальному логину. — Проверка: регистрация атомарно отклоняет занятый логин и несовпадающие пароли, password не попадает в logs/client storage, а logout инвалидирует сессию.
- [ ] 2.2 Реализовать data model и authorization для пользователя, Workspace, Folder, Task, Template, TaskBlock, rules и skills. — Проверка: пользователь не может прочитать или изменить данные другого пользователя.
- [ ] 2.3 Реализовать backup и verified legacy export/import. — Проверка: import fixture сохраняет все перечисленные сущности и связи, failure не создаёт частичное состояние.

## 3. Browser application

- [ ] 3.1 Реализовать approved responsive landing без скачивания продукта. — Проверка: desktop/tablet/mobile не имеют horizontal scroll и корректно ведут на auth.
- [ ] 3.2 Реализовать approved registration, login, logout и recovery screens. — Проверка: client/server validation уникальности логина, пароля и его подтверждения, loading/error states и keyboard navigation покрыты tests.
- [ ] 3.3 Перенести current Environment и Tasks functionality в authenticated web shell. — Проверка: Environment, import, Workspace, Folder, Task, Template, TaskBlock, clear и share проходят regression suite.
- [ ] 3.4 Адаптировать installation flow к browser security model. — Проверка: preview/merge/replace/backups сохраняются, а прямой unrestricted local filesystem access отсутствует.

## 4. Verification и handoff

- [ ] 4.1 Добавить unit, integration и browser e2e tests для identity, authorization, migration и baseline flows. — Проверка: полный suite проходит в clean environment.
- [ ] 4.2 Провести security review auth/session/input/import и accessibility review. — Проверка: findings устранены или документированы с accepted risk.
- [ ] 4.3 Обновить current OpenSpec specs, deployment guide и архивировать change. — Проверка: `pnpm verify:sdd`, tests и production build проходят.
