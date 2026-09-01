## Context

`cargo test` компилирует зависимости Tauri, в том числе `gdk-sys`, который
ищет `gdk-3.0.pc` через `pkg-config`. Ubuntu hosted runner не содержит
соответствующий development package по умолчанию.

## Decision

Установить минимальный `libgtk-3-dev` в regression job и добавить его к уже
требуемым WebKit, indicator, SVG и AppImage dependencies Linux bundle job.
Каждый job получает только необходимые ему пакеты, а общая GTK-зависимость
остаётся синхронизированной.

## Regression plan

- Проверить YAML workflow локально через `npm run lint` и `npm test`;
- повторно запустить tag workflow и убедиться, что regression job проходит
  `cargo test`.

## Rollback plan

Если пакет GTK3 вызывает несовместимость runner, revert этого workflow commit
через pull request вернёт предыдущий список пакетов без влияния на приложение
или пользовательские данные.
