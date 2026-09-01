# Матрица совместимости локального состояния

| Исходная SQLite schema | Целевая schema | Поддержка | Результат |
|---|---:|---|---|
| 0 (fresh) | 2 | Да | Создаются Task, Template, TaskBlock, default Workspace и Folder schema. |
| 1 (legacy) | 2 | Да | Существующие сущности переходят в `default` Workspace; `workspace_id` получает `default`, `folder_id` остаётся пустым. |
| >2 | 2 | Нет | Операция блокируется без изменений; требуется совместимая новая версия приложения. |

Перед upgrade с существующей базой создаётся `ai-works.sqlite3.backup-<timestamp>`.
Миграция выполняется транзакционно. При ошибке transaction откатывается, а backup
остаётся доступным для восстановления. Downgrade schema не выполняется.
