## Why

Подготовлены и проверены Figma-сценарии для настройки окружения, управления
задачами, templates и блоками задач. Нужна реализация локального desktop-сервиса,
который конечный пользователь запускает как готовое приложение без Bun, Node.js,
Rust или терминала.

## What Changes

- Реализуется кроссплатформенное desktop-приложение с нативной поставкой.
- Реализуется безопасное installation core: preview, validation, conflict,
  backup, atomic write, manifest и rollback.
- Реализуется интерфейс установки окружения по утверждённым Figma-сценариям.
- Реализуется локальное управление задачами, templates и блоками задач.

## From / To

- **From:** репозиторий содержит source-of-truth `ai/`, OpenSpec и утверждённый Figma-дизайн, но не имеет исполняемого приложения.
- **To:** пользователь устанавливает готовый пакет, выбирает папку через нативный dialog ОС, устанавливает профиль и работает с локальными задачами через desktop UI.
- **Impact:** новая обратно совместимая capability; целевые проекты меняются только после явного preview и подтверждения apply.

## Scope

- Tauri v2 desktop shell, Rust installation core, React + TypeScript + Vite UI.
- Поставка собранных бинарников/установщиков для macOS, Windows и Linux.
- Профиль `standard` из `ai/catalog.yaml`.
- Локальное персистентное хранилище задач, templates и task blocks.
- Все Figma-состояния, перечисленные в `design-approval.md`.

## Non-Goals

- Облачная синхронизация, учётные записи, совместная работа и server backend.
- Исполнение shell-команд из skills, rules, templates или пользовательского текста.
- Автоматическое разрешение пользовательских конфликтов без явного действия.
- Мобильная версия.

## Version

- **Previous:** отсутствует
- **Target:** v1.0.0
- **Level:** MINOR
- **Rationale:** первая поставка нового самостоятельного пользовательского сервиса без изменения существующего публичного поведения.

## Capabilities

### New Capabilities

- `desktop-agent-config`: поставка и desktop UI приложения.
- `profile-installation`: безопасный preview и apply профиля в выбранную папку.
- `task-composition`: локальные задачи, templates и task blocks.

### Modified Capabilities

- `configuration-catalog`: каталог становится исполняемым источником для installation core.

## Impacted Boundaries

- **Services/code:** новый Tauri application, Rust core и frontend UI.
- **OpenAPI/protobuf:** N/A, сеть не используется.
- **Data/migrations:** локальное приложение хранит manifest, backups и task data; миграции версий обязательны.
- **Security/privileges:** файловый доступ только к явно выбранной папке; secrets не читаются и не логируются.
- **Compatibility/rollout:** нативные пакеты; rollback заменённых файлов через backups и версионированные локальные data migrations.
