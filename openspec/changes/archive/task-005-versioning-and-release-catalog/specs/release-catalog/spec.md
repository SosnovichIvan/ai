## ADDED Requirements

### Requirement: Immutable release catalog

Система SHALL публиковать машиночитаемый release catalog с SemVer, датой,
grouped changes, compatibility и artifact metadata для каждой доступной версии.

#### Scenario: Выбор версии на landing

- **WHEN** пользователь выбирает опубликованную версию
- **THEN** landing показывает её added/changed/fixed/security changes,
поддерживаемые платформы, known issues и соответствующие artifacts

### Requirement: Verifiable download artifact

Каждый artifact SHALL иметь URL, SHA-256 и signature в release catalog.

#### Scenario: Проверка загруженного artifact

- **WHEN** checksum или signature artifact не совпадает с release metadata
- **THEN** artifact считается недостоверным и не предлагается для установки
