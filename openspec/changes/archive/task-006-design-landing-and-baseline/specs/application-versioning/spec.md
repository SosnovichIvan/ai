## ADDED Requirements

### Requirement: Baseline regression release policy

Система SHALL считать `v1.0.0` public stable baseline release и MUST выполнять regression
проверки текущего функционала при каждом следующем PATCH, MINOR или MAJOR
release.

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

### Requirement: Кроссплатформенная поставка

Release pipeline SHALL собирать и публиковать installable artifacts Agent Foundry
для macOS, Windows и Linux. Для каждого artifact metadata MUST хранить ОС,
architecture, checksum, signature и диапазон поддерживаемых версий ОС.

#### Scenario: Публикация stable версии

- **WHEN** stable release становится доступным пользователям
- **THEN** release catalog содержит проверяемый artifact для macOS, Windows и
  Linux под одним SemVer и technical ID `ai_rules_version`
