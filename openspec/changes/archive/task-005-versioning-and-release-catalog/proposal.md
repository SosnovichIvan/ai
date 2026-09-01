## Why

Приложение уже хранит локальные задачи и runtime-каталог конфигурации. Обновление
бинарника не должно приводить к потере этих данных, а будущий landing должен
предоставлять пользователю понятный выбор версии, безопасную загрузку и
достоверное описание изменений.

## What Changes

- Вводится единая SemVer-политика для desktop application, data schema и
  release notes.
- Определяется transactional update/migration protocol с backup и recovery.
- Определяется статический release catalog, из которого landing показывает
  поддерживаемые версии, changelog, downloads, checksum и платформы.
- Формируется совместимый release manifest для автоматической проверки
  обновлений в будущем; автоматическая установка обновления не входит в MVP.

## Scope

- Контракт release metadata, data migration и backup/recovery.
- Нормативный формат changelog и статического каталога релизов.
- Документация release process и задачи внедрения.

## Non-Goals

- Реализация landing, auto-update UI, серверного API, аналитики или аккаунтов.
- Удалённая синхронизация пользовательских задач/каталогов.
- Downgrade данных без доказанной совместимости.

## Version

- **Previous:** v0.1.0.
- **Target:** v0.2.0.
- **Level:** MINOR.
- **Rationale:** добавляется безопасный контракт поставки и обновления данных,
  не меняющий существующее пользовательское содержимое.

## Capabilities

### New Capabilities

- `application-versioning`: SemVer, release manifest и safe data migration.
- `release-catalog`: машиночитаемая история доступных релизов для landing.

## Impacted Boundaries

- **Data:** SQLite database с задачами, шаблонами и блоками задач, runtime
  catalog, import backups, schema metadata.
- **Delivery:** Tauri version, signed packages, checksum, download artifacts.
- **Security:** проверка подписи/checksum, локальный backup, никакой отправки
  пользовательских данных при update check.
