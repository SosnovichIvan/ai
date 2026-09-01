## Why

Первый запуск release workflow для `v1.0.0` остановился на отрицательном
тесте проверки SemVer: системная переменная GitHub Actions подменяла аргумент,
который тест намеренно передавал как некорректный tag.

## What Changes

- Явно переданный tag получает приоритет над `GITHUB_REF_NAME`.
- Переменная GitHub Actions сохраняется как fallback для запуска без аргумента.

## Scope

- Скрипт проверки версии релиза и его автоматические тесты.

## Non-Goals

- Изменение версии приложения, данных пользователя или логики desktop UI.

## Version

- **Previous:** v1.0.0.
- **Target:** v1.0.0.
- **Level:** deferred PATCH.
- **Rationale:** исправляется только воспроизводимость CI до первой публикации.
- **Compatibility:** команды и формат SemVer остаются прежними.

## Capabilities

### Modified Capabilities

- `product-delivery`: release gates корректно проверяются и локально, и в GitHub Actions.
