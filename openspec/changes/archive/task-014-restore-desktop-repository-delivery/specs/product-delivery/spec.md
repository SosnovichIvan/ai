## MODIFIED Requirements

### Requirement: Developer repository desktop launch

Система SHALL позволять пользователю запустить локальное desktop-приложение из
чистого clone repository после установки документированных developer
prerequisites: поддерживаемых Node.js/npm и Rust toolchain.

#### Scenario: Первый запуск из repository

- **WHEN** пользователь клонирует repository, выполняет `npm install` и
  `npm run tauri dev` в окружении с указанными prerequisites
- **THEN** открывается Agent Foundry desktop UI без запуска web server,
  PostgreSQL или Docker

### Requirement: Optional native release package

Система MAY распространять готовый нативный package для поддерживаемой
платформы. Такой пользовательский package SHALL запускаться без Node.js, npm,
Rust или терминала.

#### Scenario: Запуск готового пакета

- **WHEN** пользователь получает release package для своей ОС
- **THEN** он открывает приложение без установки developer prerequisites
