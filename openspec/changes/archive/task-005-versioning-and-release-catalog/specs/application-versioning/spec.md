## ADDED Requirements

### Requirement: Single SemVer release identity

Система SHALL публиковать каждую desktop-версию как immutable SemVer release и
SHALL синхронизировать версию package, Tauri и Cargo до сборки artifact.

#### Scenario: Несогласованная версия сборки

- **WHEN** версия любого build manifest отличается от release version
- **THEN** release pipeline завершается ошибкой до публикации artifact

### Requirement: Recoverable local data migration

Система SHALL определить compatibility локального state до его изменения и
MUST создать backup SQLite и runtime catalog до первой миграции.

#### Scenario: Ошибка обновления schema

- **WHEN** migration завершается ошибкой после начала изменения state
- **THEN** система восстанавливает предыдущие данные, не открывает mutating UI и
  показывает безопасный путь recovery

#### Scenario: Запуск старой версии после новой schema

- **WHEN** application version обнаруживает state с более новой schema
- **THEN** система не выполняет автоматический downgrade и блокирует запись

### Requirement: Preservation of task composition data

Система SHALL мигрировать `Task`, `Template` и `TaskBlock` в одной
транзакционной SQLite migration и MUST сохранить их идентификаторы, содержание,
nullable description, timestamps, revision и snapshot-связи.

#### Scenario: Обновление с существующей задачей и template

- **WHEN** пользователь обновляет приложение, содержащее task с `blockSnapshot`
  и независимый template с `sourceTemplateId`
- **THEN** после успешной migration все сущности открываются с прежними UUID и
  содержанием, а template не меняется вслед за task

#### Scenario: Сбой migration пользовательских сущностей

- **WHEN** migration Task, Template или TaskBlock завершается ошибкой
- **THEN** система восстанавливает все три таблицы из единого pre-migration
  backup без частично преобразованных записей

### Requirement: Preservation of user catalog changes

Система SHALL добавлять missing bundled seed files при compatible update и MUST
сохранять пользовательские edits и imported files в runtime catalog.

#### Scenario: Compatible update после редактирования rule

- **WHEN** пользователь изменил active rule, а новая версия добавляет другой seed rule
- **THEN** edited rule не меняется, а новый seed rule появляется в каталоге
