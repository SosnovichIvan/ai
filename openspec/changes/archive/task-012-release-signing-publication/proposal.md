## Why

Pipeline v1.0.0 собирает кроссплатформенные артефакты и SHA-256, но не может
выдать подпись без внешних сертификатов и разрешения на публикацию. Пользователь
должен получать проверяемый release, а секретные ключи не должны попадать в
репозиторий, логи или локальный runtime-каталог.

## What Changes

- Определяется изолированная процедура signing для macOS, Windows и Linux.
- GitHub Actions получает signing-материалы только из Secrets на время job.
- Release публикуется по immutable SemVer tag после verification и подписи.
- В release catalog попадают checksum, signature и проверяемые download URL.

## Scope

- CI/CD workflow, release artifacts, документация владельца release.
- Проверка подписи и защита от подмены собранного artifact.

## Non-Goals

- Создание, импорт или хранение приватных signing-ключей в репозитории.
- Публикация без указанного пользователем repository и прав владельца.
- Изменение функционала приложения или public версии `1.0.0`.

## Version

- **Previous:** v1.0.0.
- **Target:** v1.0.0.
- **Level:** deferred PATCH.
- **Rationale:** закрывается operational release-процесс без изменения продукта.
- **Compatibility:** формат local state и download-каталога сохраняется.

## Capabilities

### Modified Capabilities

- `application-versioning`: release pipeline обязан публиковать только
  подписанные артефакты с проверяемым metadata.
