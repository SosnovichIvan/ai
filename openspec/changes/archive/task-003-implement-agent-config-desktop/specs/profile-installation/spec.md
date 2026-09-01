## ADDED Requirements

### Requirement: Native folder selection and preview

Система SHALL открывать нативный dialog ОС для выбора target folder, SHALL
показывать выбранный путь только для чтения и SHALL рассчитывать preview до apply.

#### Scenario: Выбран безопасный target

- **WHEN** пользователь выбирает доступную папку через системный dialog
- **THEN** система показывает нормализованный путь, preview операций и активирует apply только при отсутствии unresolved conflicts

### Requirement: Transactional profile application

Система SHALL создавать backup перед заменой managed file, SHALL атомарно
применять profile и SHALL обновлять manifest только после полного успеха.

#### Scenario: Ошибка во время установки

- **WHEN** запись любого managed file завершается ошибкой
- **THEN** система восстанавливает ранее заменённые файлы, не обновляет manifest и сообщает безопасную причину ошибки

### Requirement: Observable safe progress

Система SHALL показывать текущую стадию и процент установки, включая backup,
rules, skills и manifest, без показа secret content.

#### Scenario: Установка standard profile

- **WHEN** пользователь подтверждает preview profile `standard`
- **THEN** UI отображает progress от 0 до 100 процентов и итог completed, conflict или failed
