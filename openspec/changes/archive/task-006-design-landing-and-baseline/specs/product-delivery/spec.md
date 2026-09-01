## ADDED Requirements

### Requirement: Reproducible multi-platform release pipeline

Система SHALL собирать каждый tag release на нативных CI runners для macOS,
Windows и Linux из зафиксированного lockfile. Pipeline SHALL запускать
typecheck, lint и backend regression suite до публикации артефактов.

#### Scenario: Публикация release

- **WHEN** maintainer запускает workflow для SemVer tag
- **THEN** CI публикует installable artifacts всех поддерживаемых ОС, файл
  `SHA-256SUMS` и machine-readable release metadata, не раскрывая секреты

### Requirement: Truthful download catalog

Landing SHALL не предлагать файл к скачиванию, пока у выбранного артефакта нет
публичного URL, SHA-256 и signature metadata.

#### Scenario: Артефакт ещё не опубликован

- **WHEN** release catalog содержит платформу без complete metadata
- **THEN** CTA сообщает, что сборка готовится к публикации, и не начинает
  скачивание
