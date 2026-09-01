## 1. OpenSpec и scope

- [x] 1.1 Зафиксировать desktop repository delivery, данные и rollback. — Проверка: proposal, design и design-approval описывают scope, rollback и regression.
- [x] 1.2 Архивировать отменённый web-first change как superseded. — Проверка: в archive есть причина отмены, а active changes не предлагают web delivery.

## 2. Удаление web delivery

- [x] 2.1 Удалить landing, API, Docker compose и web-only release artifacts. — Проверка: repository не содержит `landing/`, `server/` и `docker-compose.web.yml`.
- [x] 2.2 Удалить browser-first scripts и зависимости без затрагивания Tauri runtime; перевести repository install на npm. — Проверка: `package.json` не содержит API/landing scripts или `pg`; `npm ci` проходит.

## 3. Документация и verification

- [x] 3.1 Переписать README как инструкцию установки и запуска из cloned repository через npm. — Проверка: README содержит prerequisites, clone, `npm install`, dev run, build, troubleshooting и границы для готового пакета.
- [x] 3.2 Выполнить desktop regression gates. — Проверка: typecheck, lint, tests, Rust tests и Tauri `.app` build проходят.
- [x] 3.3 Обновить current specs и архивировать change. — Проверка: product-delivery описывает repository desktop путь, `npm run verify:sdd` проходит, change архивирован.
