## Purpose

Продукт выпускается как проверяемое desktop-приложение с единым SemVer,
безопасными миграциями локальных данных и каталогом артефактов по платформам.

## Requirements

### Requirement: Baseline regression release policy

Система SHALL считать `v1.0.0` public stable baseline release и MUST выполнять
regression проверки текущего функционала при каждом следующем PATCH, MINOR или
MAJOR release.

#### Scenario: Выпуск версии с новой возможностью

- **WHEN** change добавляет новую возможность и маркирован MINOR
- **THEN** release pipeline проверяет новую возможность и baseline flows
  Environment, import, Task, Template и TaskBlock

### Requirement: Technical distribution identity

Система SHALL использовать `ai_rules_version` как технический immutable ID
release artifacts и update metadata, отдельно от public display name
`Agent Foundry`.

#### Scenario: Публикация artifact

- **WHEN** release pipeline собирает versioned artifact
- **THEN** его metadata содержит `ai_rules_version`, SemVer и platform identity

### Requirement: Compatible local state

Система SHALL до изменения локальных данных проверять `state-manifest.json`,
фактическую версию SQLite-схемы и поддерживаемый migration registry. Manifest
MUST хранить только номера форматов и timestamp; его запись MUST быть атомарной.

#### Scenario: Открытие существующей v1.0.0 установки

- **WHEN** manifest ещё отсутствует, а SQLite-схема поддерживается текущим
  приложением
- **THEN** приложение создаёт manifest только после успешного открытия базы и
  сохраняет данные Task, Template, TaskBlock, Workspace и Folder

#### Scenario: Повреждённое или более новое локальное состояние

- **WHEN** manifest повреждён, не согласован с базой либо создан более новой
  версией приложения
- **THEN** система не выполняет мутацию, не перезаписывает пользовательские
  данные и сообщает безопасный путь восстановления через совместимую версию или
  резервную копию

### Requirement: Validated release catalog

Система SHALL валидировать machine-readable release catalog до публикации.
Published artifact MUST иметь URL, SHA-256 и signature; unavailable artifact
MUST быть явно маркирован и не выдавать себя за доступный download.

#### Scenario: Ошибка metadata перед релизом

- **WHEN** artifact имеет некорректный checksum, URL или несколько stable
  `latest` releases
- **THEN** release pipeline завершается до публикации и сообщает ошибку

### Requirement: Schema compatibility matrix

Система SHALL документировать и regression-проверять поддерживаемые upgrade
paths локальной SQLite schema. Для schema `0` и `1` текущая schema `2` MUST
сохранять данные сущностей либо создавать fresh state.

#### Scenario: Upgrade legacy schema 1

- **WHEN** приложение открывает legacy Task, Template или TaskBlock из schema `1`
- **THEN** значения идентификатора, текста, revision, timestamps и связей
  сохраняются, а сущность получает default Workspace

### Requirement: Кроссплатформенная поставка

Release pipeline SHALL собирать и публиковать installable artifacts Agent Foundry
для macOS, Windows и Linux. Для каждого artifact metadata MUST хранить ОС,
architecture, checksum, signature и диапазон поддерживаемых версий ОС.

#### Scenario: Публикация stable версии

- **WHEN** stable release становится доступным пользователям
- **THEN** release catalog содержит проверяемый artifact для macOS, Windows и
  Linux под одним SemVer и technical ID `ai_rules_version`
