## Why

Desktop-дистрибуция требует platform-specific сертификатов, notarization и
установки. Продукт переходит к browser-first web-приложению: пользователь
создаёт учётную запись, входит в неё и продолжает работать с теми же сущностями
Environment, Workspace, Folder, Task, Template, TaskBlock, rules и skills без
скачивания нативного пакета.

## What Changes

- Удаляются desktop build/release flows, ссылки на installer, выбор ОС,
  download CTA и каталог binary artifacts на лендинге.
- Лендинг становится входной страницей web-приложения с действиями
  «Зарегистрироваться» и «Войти».
- Добавляются адаптивные формы регистрации, входа, выхода и защищённая сессия;
  email заменён на уникальный логин, пароль и его подтверждение сохраняются.
- После входа пользователь получает web-интерфейс со всей текущей логикой
  рабочих пространств, папок, сущностей, импорта и безопасной установки
  профилей в рамках доступной browser-модели.
- Локальные desktop-данные получают явный безопасный сценарий миграции или
  экспорт/импорт; потеря данных не допускается по умолчанию.

## Scope

- Browser frontend, backend/API для identity и server-side persistence,
  адаптивный landing и authenticated application shell.
- Миграция/экспорт существующих пользовательских данных и удаление Tauri,
  нативных bundle/release workflow и landing download-функционала.

## Non-Goals

- Обход Apple/Windows code-signing или выпуск unsigned desktop application.
- Хранение паролей, токенов, секретов или приватных ключей в frontend.
- Автоматическое чтение произвольной локальной файловой системы браузером.

## Version

- **Previous:** v1.0.0.
- **Target:** v1.0.0.
- **Level:** deferred MINOR.
- **Rationale:** меняется способ доступа к продукту, а public версия остаётся
  зафиксированной по запросу владельца до следующего formal release.
- **Compatibility:** существующая логика сущностей сохраняется; migration
  desktop state проектируется до удаления исходной реализации.

## Capabilities

### New Capabilities

- `web-identity`: регистрация, вход, выход, восстановление доступа и
  авторизованный доступ к данным пользователя через уникальный логин.
- `web-workspace`: browser-доступ к рабочим пространствам и сущностям.

### Modified Capabilities

- `product-landing`: web entrypoint без скачивания binary artifacts.
- `product-delivery`: web delivery вместо native distribution.
- `installation-safety`: server-side and browser-safe profile flow без прямого
  доступа к локальной файловой системе пользователя.
