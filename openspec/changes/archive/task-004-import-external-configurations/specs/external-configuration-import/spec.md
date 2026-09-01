## ADDED Requirements

### Requirement: Controlled external configuration analysis

Система SHALL позволять пользователю выбрать одну внешнюю директорию через
нативный folder picker и SHALL анализировать в ней только allowlisted rules и
skills. Система MUST исключать secrets, private keys, technical directories и
symlinks до чтения их содержимого.

#### Scenario: Анализ смешанной внешней папки

- **WHEN** пользователь выбирает папку, содержащую rules, `SKILL.md`, `.env` и
  `node_modules`
- **THEN** результат показывает только допустимые rules и skills, сообщает
  безопасное число исключений и не раскрывает secret content или имена секретов

### Requirement: Explicit import mode and preview

Система SHALL показать immutable preview до import commit и SHALL потребовать от
пользователя явный выбор режима `Добавить` либо `Заменить`.

#### Scenario: Добавление внешнего набора

- **WHEN** пользователь выбирает `Добавить` и подтверждает preview без конфликтов
- **THEN** система добавляет только новые управляемые пути в runtime catalog и
  сохраняет существующие файлы без изменений

#### Scenario: Замена активного набора

- **WHEN** пользователь выбирает `Заменить`, видит preview и подтверждает замену
- **THEN** система создаёт backup прежнего runtime catalog и атомарно заменяет
  active managed set найденным набором

### Requirement: Stable configuration viewer

Система SHALL показывать фактические files активного runtime catalog и MUST
безопасно обрабатывать смену между `Skills` и `Rules` при разном размере списков.

#### Scenario: Смена категории после выбора файла

- **WHEN** пользователь выбирает элемент в `Rules`, а затем переключается на
  `Skills` с меньшим количеством элементов
- **THEN** система выбирает первый существующий skill либо empty state и не
  завершает приложение с ошибкой
