## Контекст

Schema `0` — fresh database, schema `1` — Task/Template/TaskBlock без workspace,
schema `2` — текущий формат с Workspace и Folder. Миграция запускается в одной
transaction с pre-migration backup.

## Rollback и regression

При ошибке transaction откатывается, а backup остаётся в app-data. Regression
fixture schema `1` проверяет UUID, nullable description, content,
sourceTemplateId, blockSnapshot, timestamps, revision и default Workspace.
