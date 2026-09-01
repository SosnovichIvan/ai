## Why

Повторный запуск первого release workflow прошёл frontend и OpenSpec checks,
но остановился на `cargo test`: на Ubuntu runner отсутствовала системная
библиотека разработки GTK3 (`gdk-3.0`).

## What Changes

- Regression job устанавливает GTK3 development dependency до `cargo test`.
- Linux bundle job дополнен `libgtk-3-dev` наряду с его WebKit и packaging
  dependencies.

## Scope

- Зависимости Ubuntu jobs в `.github/workflows/release.yml`.

## Non-Goals

- Изменение runtime приложения, версии, пользовательских данных или release assets.

## Version

- **Previous:** v1.0.0.
- **Target:** v1.0.0.
- **Level:** deferred PATCH.
- **Rationale:** исправляется только окружение CI до первого опубликованного релиза.
- **Compatibility:** содержимое релизных пакетов и публичные сценарии не меняются.

## Capabilities

### Modified Capabilities

- `product-delivery`: Linux regression gates и bundle выполняются в полном desktop окружении.
