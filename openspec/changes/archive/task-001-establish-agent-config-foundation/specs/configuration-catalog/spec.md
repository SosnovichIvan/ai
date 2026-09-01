## ADDED Requirements

### Requirement: Canonical configuration catalog

Система SHALL хранить устанавливаемые правила, skills и профили в `ai/`, а
`ai/catalog.yaml` SHALL задавать состав профиля и целевые относительные пути.

#### Scenario: Подготовка установки standard profile

- **WHEN** пользователь выбирает профиль `standard`
- **THEN** установщик получает его состав только из `ai/catalog.yaml` и файлов `ai/`

### Requirement: Generated target isolation

Система SHALL считать файлы в выбранном проекте производными копиями и SHALL
не использовать их как источник для последующих установок.

#### Scenario: Локальное изменение target file

- **WHEN** файл в целевом проекте отличается от зафиксированного manifest
- **THEN** установщик помечает операцию как конфликт и не перезаписывает файл по умолчанию
