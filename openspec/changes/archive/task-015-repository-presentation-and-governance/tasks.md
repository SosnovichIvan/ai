## 1. README и media

- [x] 1.1 Добавить проверяемые screenshots desktop-интерфейса в repository. — Проверка: README использует локальные изображения, а изображения корректно открываются из GitHub rendering.
- [x] 1.2 Описать возможности, npm setup, data safety и local bundle в README. — Проверка: README даёт ясный путь от clone до `npm run tauri dev`.

## 2. Release и repository governance

- [x] 2.1 Проверить и обновить GitHub Release workflow для SemVer artifacts. — Проверка: workflow запускает npm/Rust gates, multi-platform bundle и публикует GitHub Release.
- [x] 2.2 Добавить CODEOWNERS владельца. — Проверка: любые файлы repository имеют owner `@SosnovichIvan`.
- [x] 2.3 Применить GitHub ruleset к `main` и `develop`. — Проверка: прямой push/force push блокируются, а merge требует PR, один approval и approval code owner.

## 3. Verification

- [x] 3.1 Выполнить документационные и desktop regression checks. — Проверка: npm lint/test/typecheck, Rust tests, Tauri build и OpenSpec validation проходят.
- [x] 3.2 Обновить current specs и архивировать change. — Проверка: product-delivery отражает README/release/PR governance, change архивирован.
