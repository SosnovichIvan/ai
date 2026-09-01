## 1. Контракт состояния

- [x] 1.1 Добавить versioned state manifest и атомарную запись. — Проверка:
  fresh и существующее v1.0.0 состояние получают валидный manifest.
- [x] 1.2 Добавить migration registry и сверку фактической SQLite-схемы с
  manifest. — Проверка: поддерживаемые состояния открываются, новые и
  рассогласованные блокируются до мутации.

## 2. Безопасность обновления

- [x] 2.1 Гарантировать backup перед миграцией и rollback при ошибке. —
  Проверка: injected failure не меняет исходную базу.
- [x] 2.2 Добавить safe recovery для повреждённого manifest. — Проверка:
  corruption fixture блокирует mutating operation и сообщает безопасное
  действие без раскрытия содержимого данных.

## 3. Regression и handoff

- [x] 3.1 Добавить fixtures/тесты сохранности Task, Template, TaskBlock,
  Workspace и Folder. — Проверка: upgrade fixture сохраняет идентификаторы,
  связи и контент.
- [x] 3.2 Обновить current OpenSpec и инструкцию восстановления. — Проверка:
  `pnpm verify:sdd`, `cargo test`, web/landing проверки проходят.
