## Purpose

Продукт должен запускаться конечным пользователем как готовое нативное
приложение без установки Bun, Node.js, Rust или других инструментов разработки.

## Requirements

### Requirement: Developer repository desktop launch

Система SHALL позволять запустить локальное desktop-приложение из чистого
clone repository после установки документированных Node.js/npm и Rust
toolchain. Такой запуск MUST не требовать web server, PostgreSQL или Docker.

#### Scenario: Первый запуск из repository

- **WHEN** пользователь клонирует repository, выполняет `npm install` и
  `npm run tauri dev` в окружении с указанными prerequisites
- **THEN** открывается Agent Foundry desktop UI с локальным хранилищем

### Requirement: Self-contained end-user distribution

Система SHALL распространяться в виде нативного бинарника или установщика для
поддерживаемой платформы, включающего пользовательский интерфейс и ядро
установки.

#### Scenario: Первый запуск конечным пользователем

- **WHEN** пользователь устанавливает и открывает релизный пакет
- **THEN** приложение запускается без предварительной установки Bun, Node.js, Rust или CLI

### Requirement: Optional automation interface

Система MAY поставлять CLI для автоматизации, но пользовательский сценарий
настройки SHALL быть полностью доступен через desktop-интерфейс.

#### Scenario: Установка без терминала

- **WHEN** пользователь выбирает папку и профиль в desktop-интерфейсе
- **THEN** он может просмотреть и применить установку без выполнения команд в терминале

### Requirement: Reproducible multi-platform release pipeline

Система SHALL собирать каждый tag release на нативных CI runners для macOS,
Windows и Linux из lockfile и запускать regression gates до публикации.

#### Scenario: Публикация release

- **WHEN** maintainer запускает workflow для SemVer tag
- **THEN** CI публикует installable artifacts, `SHA-256SUMS` и release metadata без секретов

### Requirement: Repository presentation and governed changes

Repository SHALL содержать README с возможностями desktop-продукта,
воспроизводимой npm-инструкцией и локальными скриншотами интерфейса. Изменения
веток `main` и `develop` SHALL попадать в них только через pull request,
одобренный code owner. Repository administrator MAY обойти approval только при
merge pull request; такой bypass MUST не разрешать прямой push в защищённые
ветки.

#### Scenario: Вклад в защищённую ветку

- **WHEN** contributor пытается напрямую отправить commit в `main` или
  `develop`
- **THEN** GitHub отклоняет push и предлагает создать pull request

#### Scenario: Merge собственного PR владельцем

- **WHEN** repository administrator завершает собственный pull request в
  `main` или `develop`
- **THEN** GitHub позволяет bypass только в контексте PR, сохраняя запрет на
  прямой push

#### Scenario: Проверка продукта из README

- **WHEN** пользователь открывает repository на GitHub
- **THEN** он видит возможности продукта, screenshots и команды `npm install`,
  `npm run tauri dev`
