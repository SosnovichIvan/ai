## ADDED Requirements

### Requirement: Scoped catalog clearance

Система SHALL позволять удалить один rule или skill либо всю соответствующую
категорию только после явного confirmation с числом и именами затрагиваемых
элементов.

#### Scenario: Очистка rules

- **WHEN** пользователь подтверждает удаление всех rules
- **THEN** система удаляет только записи rules из локального каталога и
  обновляет отображаемый список, не затрагивая skills, profiles или target project

### Requirement: Project data clearance boundary

Система SHALL выполнять полное очищение только в локальном data directory
приложения и SHALL не удалять файлы выбранного target project.

#### Scenario: Полное очищение

- **WHEN** пользователь подтверждает полное очищение после просмотра scope
- **THEN** система удаляет локальные tasks, templates, blocks, rules и skills,
  но не читает и не удаляет файлы за границей data directory
