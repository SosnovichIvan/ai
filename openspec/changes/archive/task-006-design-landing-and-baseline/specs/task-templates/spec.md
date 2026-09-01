## ADDED Requirements

### Requirement: Scoped task entity clearance

Система SHALL позволять удалить отдельную Task, Template или TaskBlock и
очистить целую категорию только после явного confirmation.

#### Scenario: Удаление одного шаблона

- **WHEN** пользователь подтверждает удаление Template
- **THEN** удаляется только выбранный Template, а независимые Tasks и Blocks
  сохраняются
