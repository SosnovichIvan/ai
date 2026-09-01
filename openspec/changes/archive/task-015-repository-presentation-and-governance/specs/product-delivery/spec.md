## ADDED Requirements

### Requirement: Repository presentation and governed changes

Repository SHALL содержать README с возможностями desktop-продукта,
воспроизводимой npm-инструкцией и локальными скриншотами интерфейса. Изменения
веток `main` и `develop` SHALL попадать в них только через pull request,
одобренный code owner.

#### Scenario: Вклад в защищённую ветку

- **WHEN** contributor пытается напрямую отправить commit в `main` или
  `develop`
- **THEN** GitHub отклоняет push и предлагает создать pull request

#### Scenario: Проверка продукта из README

- **WHEN** пользователь открывает repository на GitHub
- **THEN** он видит возможности продукта, screenshots и команды `npm install`,
  `npm run tauri dev`
