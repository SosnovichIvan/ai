## 1. Release version contract

- [x] 1.1 Сделать root `package.json` единственным version source и добавить CI-check синхронизации Tauri/Cargo. — Проверка: `verify-release-version` принимает `v1.0.0` и отклоняет не-SemVer tag; CI запускает check до bundle.
- [x] 1.2 Добавить immutable tag/release checklist и structured changelog template. — Проверка: `docs/release-checklist.md` задаёт immutable tag и JSON template changelog; catalog validator проверяет release metadata.

## 2. Data preservation

- [x] 2.1 Добавить `state-manifest.json`, schema compatibility check и migration registry. — Выполнено в `task-008-state-compatibility`; приложение определяет fresh, compatible, migration-required и newer-than-app state до UI.
- [x] 2.2 Реализовать transactional SQLite migration с pre-migration backup и failure recovery для Task, Template и TaskBlock. — Выполнено в `task-008-state-compatibility`; failure fixture восстанавливает исходное состояние.
- [x] 2.2a Добавить entity-compatibility fixtures для Task, Template и TaskBlock, включая snapshots и source-template связи. — Выполнено в `task-011-schema-compatibility-matrix`; legacy fixture сохраняет UUID, revision, timestamps, content, nullable description, `sourceTemplateId` и `blockSnapshot`.
- [x] 2.3 Реализовать catalog migration через staging/atomic rename с сохранением user edits/imports. — Выполнено в `task-004-import-external-configurations`; runtime-каталог применяет import через staging и backup.
- [x] 2.4 Реализовать recovery state для повреждённого manifest/backup. — Выполнено в `task-008-state-compatibility`; corruption fixture блокирует mutating operations и возвращает safe recovery information.

## 3. Release catalog and landing contract

- [x] 3.1 Определить JSON Schema для release-каталога. — Выполнено в `task-010-release-catalog-schema`; valid/invalid fixtures проходят validation.
- [x] 3.2 Генерировать checksums и signatures артефактов в release pipeline. — Проверка checksum выполнена; подписывание и фактическая публикация вынесены в `task-012-release-signing-publication`, так как требуют внешних сертификатов и прав на release repository.
- [x] 3.3 Реализовать landing/страницу релизов по отдельному утверждённому Figma change. — Выполнено в `task-006-design-landing-and-baseline` и `task-009-versioned-landing-container`; пользователь выбирает версию и видит grouped changes.

## 4. Verification and handoff

- [x] 4.1 Построить compatibility matrix как минимум для двух предыдущих schema versions. — Выполнено в `task-011-schema-compatibility-matrix`; documented upgrade и recovery smoke покрывают schema 0 и 1.
- [x] 4.2 Обновить README и release documentation. — Проверка: пользовательская инструкция `docs/website-update-guide.md` описывает update, preservation, checksum, recovery и безопасный downgrade без developer toolchain.
