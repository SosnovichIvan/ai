## 1. Compatibility proof

- [x] 1.1 Зафиксировать matrix для schema 0 и 1. — Проверка: документ описывает
  supported upgrade path и rollback.
- [x] 1.2 Расширить legacy migration fixture. — Проверка: значения Task и
  default Workspace сохраняются после `1 → 2`.

## 2. Handoff

- [x] 2.1 Обновить current spec, выполнить tests и архивировать. — Проверка:
  `cargo test` и `pnpm verify:sdd` проходят.
