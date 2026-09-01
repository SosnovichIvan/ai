## ADDED Requirements

### Requirement: Native self-contained application

Система SHALL распространяться как нативное desktop-приложение для
поддерживаемой платформы и SHALL запускаться без установленного Bun, Node.js,
Rust или терминала у конечного пользователя.

#### Scenario: Первый запуск установленного приложения

- **WHEN** пользователь устанавливает и открывает release package
- **THEN** он видит `AI Works` и может начать настройку окружения без developer toolchain

### Requirement: AI Works navigation

Система SHALL отображать единственный блок `AI Works` с рабочими областями
`Установка окружения` и `Задачи и шаблоны`, явно обозначая активную область.

#### Scenario: Переход к задачам

- **WHEN** пользователь выбирает `Задачи и шаблоны` в `AI Works`
- **THEN** центральная область показывает task workspace, а navigation обозначает его активным
