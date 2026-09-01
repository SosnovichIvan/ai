## Why

Текущая SQLite schema равна `2`, но поддержка форматов fresh (`0`) и legacy
workspace migration (`1`) не зафиксирована как release compatibility matrix.

## What Changes

- Фиксируется matrix `0 → 2` и `1 → 2`.
- Migration fixture проверяет сохранность связанных полей legacy Task.

## Scope

- SQLite local state и release documentation.

## Non-Goals

- Downgrade, новый UI или изменение public version.

## Version

- **Previous:** v1.0.0.
- **Target:** v1.0.0.
- **Level:** deferred PATCH.
- **Rationale:** добавляется доказательство совместимости текущего формата.
- **Compatibility:** fresh и legacy schema `1` поддержаны на schema `2`.

## Capabilities

### Modified Capabilities

- `application-versioning`: documented schema compatibility matrix.
