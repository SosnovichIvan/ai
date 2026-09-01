## Purpose

Каталог конфигурации задаёт единственный версионируемый источник общих правил,
skills и установочных профилей для целевых проектов.

## Requirements

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

### Requirement: Scoped catalog clearance

Система SHALL позволять удалить один rule или skill либо всю соответствующую
категорию только после явного confirmation.

#### Scenario: Очистка rules

- **WHEN** пользователь подтверждает удаление всех rules
- **THEN** система удаляет только локальные rules и обновляет список, не затрагивая skills или target project

### Requirement: Project data clearance boundary

Система SHALL выполнять полное очищение только в локальном data directory
приложения и SHALL не удалять файлы выбранного target project.

#### Scenario: Полное очищение

- **WHEN** пользователь подтверждает полное очищение
- **THEN** система удаляет только локальные tasks, templates, blocks, rules и skills

### Requirement: Safe external configuration import

Система SHALL анализировать только Markdown rules и `SKILL.md` внутри явно
выбранной пользователем директории. До commit она MUST показать immutable
preview, а режим `replace` MUST требовать явного подтверждения и создавать
резервную копию runtime-каталога.

#### Scenario: Активный каталог изменился после preview

- **WHEN** пользовательская правка активного rules или skills появилась после
  формирования preview
- **THEN** commit отклоняется как устаревший, а пользовательская правка остаётся
  без изменений

#### Scenario: Исключение чувствительных файлов

- **WHEN** выбранная директория содержит `.env`, ключ, token, cookie, symlink
  либо технический каталог
- **THEN** scanner не читает и не показывает его содержимое, а учитывает
  исключение только счётчиком
