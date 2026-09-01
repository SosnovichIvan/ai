## Why

После добавления GTK3 release workflow дошёл дальше, но `cargo test` на Ubuntu
runner не нашёл `libsoup-3.0` и `javascriptcoregtk-4.1`. Эти библиотеки
поставляет пакет `libwebkit2gtk-4.1-dev`, уже требуемый Linux bundle job.

## What Changes

- Regression job устанавливает полный набор Linux Tauri/WebKit dependencies,
  совпадающий с потребностями Linux bundle.

## Scope

- Linux dependency setup в regression job release workflow.

## Non-Goals

- Изменение runtime приложения, версии, данных пользователя или состава release assets.

## Version

- **Previous:** v1.0.0.
- **Target:** v1.0.0.
- **Level:** deferred PATCH.
- **Rationale:** завершается настройка CI до первого опубликованного релиза.
- **Compatibility:** публичные команды и содержимое приложения не меняются.

## Capabilities

### Modified Capabilities

- `product-delivery`: Linux regression gate получает все системные зависимости Tauri/WebKit.
