## Purpose

Продукт хранит и применяет версионируемые шаблоны для формирования задач,
сохраняя их источник, параметры и полученный результат.

## Requirements

### Requirement: Workspace ownership

Система SHALL хранить каждую Task, Template и TaskBlock в одном Workspace.

#### Scenario: Migration existing entities

- **WHEN** пользователь открывает обновлённое приложение с существующими сущностями
- **THEN** система создаёт default Workspace и переносит в него все сущности транзакционно

### Requirement: Folder navigation and sharing

Система SHALL позволять создавать вложенные folders, раскрывать дерево или
входить в folder, а также копировать сущности в другое Workspace.

#### Scenario: Copy task to another workspace

- **WHEN** пользователь подтверждает share Task в target Workspace
- **THEN** target получает независимую копию, а исходная Task не изменяется

### Requirement: Scoped task entity clearance

Система SHALL позволять удалить отдельную Task, Template или TaskBlock и
очистить целую категорию только после явного confirmation.

#### Scenario: Удаление одного шаблона

- **WHEN** пользователь подтверждает удаление Template
- **THEN** удаляется только выбранный Template, а независимые Tasks и Blocks сохраняются

### Requirement: Canonical task templates

Система SHALL хранить task templates в каноническом каталоге `ai/` и SHALL
указывать их в каталоге конфигурации с идентификатором и версией.

#### Scenario: Выбор шаблона задачи

- **WHEN** пользователь открывает создание задачи
- **THEN** интерфейс показывает доступные templates из канонического каталога с их названием и версией

### Requirement: Reproducible task generation

Система SHALL сохранять идентификатор и версию использованного template,
переданные параметры и сформированный результат вместе с задачей.

#### Scenario: Повторное открытие сформированной задачи

- **WHEN** пользователь открывает ранее созданную задачу
- **THEN** система показывает template и версию, из которых был сформирован её результат

### Requirement: Safe template rendering

Система SHALL обрабатывать параметры template как данные и SHALL не выполнять
команды, код или инструкции, полученные из template либо пользовательского ввода.

#### Scenario: Недопустимый параметр

- **WHEN** обязательный параметр отсутствует или не проходит schema template
- **THEN** система не формирует задачу и показывает ошибку валидации
