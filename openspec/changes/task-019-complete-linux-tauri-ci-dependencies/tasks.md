## 1. Исправление CI

- [x] 1.1 Установить полный набор Tauri/WebKit dependencies в regression job. — Проверка: `pkg-config` получает GTK3, libsoup и JavaScriptCore metadata.

## 2. Проверка и выпуск

- [x] 2.1 Выполнить локальные проверки и OpenSpec validation. — Проверка: typecheck, lint, 11 frontend-тестов, 28 Rust-тестов и validation проходят.
- [ ] 2.2 Слить change в `main` через pull request. — Проверка: прямой push не используется.
- [ ] 2.3 Перезапустить `v1.0.0`. — Проверка: GitHub Release содержит platform assets и checksums.
