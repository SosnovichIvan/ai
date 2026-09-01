## ADDED Requirements

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
