## 1. Исправление CI

- [x] 1.1 Добавить GTK3 development package в Linux regression job. — Проверка: `cargo test` на Ubuntu получает `gdk-3.0.pc`.
- [x] 1.2 Дополнить зависимости Linux bundle job. — Проверка: оба Linux jobs получают `libgtk-3-dev`.

## 2. Проверка и выпуск

- [x] 2.1 Выполнить локальные проверки и валидацию change. — Проверка: typecheck, lint, 11 frontend тестов, 28 Rust тестов и OpenSpec validation проходят.
- [ ] 2.2 Слить исправление через PR в `develop` и `main`. — Проверка: прямые push в защищённые ветки не используются.
- [ ] 2.3 Перезапустить tag `v1.0.0`. — Проверка: опубликован GitHub Release с assets и checksums.
