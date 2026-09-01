## Why

Landing зависит от `releases.json`, но до сих пор этот файл не проверяется как
контракт. Ошибка в ссылке, checksum или platform metadata может показать
пользователю недостоверную кнопку скачивания.

## What Changes

- Добавляется проверяемая schema release catalog.
- CI валидирует каталог до публикации.
- Разделяются published artifact и честно unavailable artifact.

## Scope

- `landing/releases.json`, генератор release metadata и CI verification.
- Node-валидатор и fixtures без сетевых запросов.

## Non-Goals

- Публикация релиза, signing key management, auto-update и изменение landing UI.

## Version

- **Previous:** v1.0.0.
- **Target:** v1.0.0.
- **Level:** deferred PATCH.
- **Rationale:** проверяется существующий release contract без изменения
  публичной функциональности.
- **Compatibility:** valid v1.0.0 catalog остаётся валидным; неполные
  артефакты явно допускаются только как unavailable.

## Capabilities

### Modified Capabilities

- `application-versioning`: machine-readable проверка release metadata.
