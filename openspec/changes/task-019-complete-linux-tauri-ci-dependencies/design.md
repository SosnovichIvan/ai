## Context

Tauri Rust dependencies требуют GTK3 и WebKitGTK development metadata. После
установки только GTK3 `pkg-config` последовательно обнаружил отсутствие
`libsoup-3.0.pc` и `javascriptcoregtk-4.1.pc`.

## Decision

Regression job получает тот же набор зависимостей, который уже использует
Linux bundle job: `libgtk-3-dev`, `libwebkit2gtk-4.1-dev`,
`libappindicator3-dev`, `librsvg2-dev` и `patchelf`. Это исключает различие
между тестовым и packaging окружениями.

## Regression plan

- Выполнить typecheck, lint, frontend tests, Rust tests и OpenSpec validation;
- повторно запустить tag workflow и проверить успешные `cargo test` и bundle jobs.

## Rollback plan

Если список пакетов несовместим с Ubuntu runner, revert workflow commit через
pull request вернёт предыдущую конфигурацию без воздействия на приложение или
пользовательские данные.
