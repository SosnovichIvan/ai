## MODIFIED Requirements

### Requirement: Reproducible multi-platform release pipeline

Система SHALL собирать каждый tag release на нативных CI runners для macOS,
Windows и Linux из lockfile и запускать regression gates до публикации. Проверка
версии MUST отдавать приоритет явно переданному tag над значением среды CI.

#### Scenario: Проверка некорректного tag в CI

- **WHEN** тест передаёт в скрипт проверки явный некорректный tag при наличии
  `GITHUB_REF_NAME`
- **THEN** скрипт отклоняет этот явный tag, а не использует значение среды
