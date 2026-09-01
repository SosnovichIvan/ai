## MODIFIED Requirements

### Requirement: Repository presentation and governed changes

Repository SHALL содержать README с возможностями desktop-продукта,
воспроизводимой npm-инструкцией и локальными скриншотами интерфейса. Изменения
веток `main` и `develop` SHALL попадать в них только через pull request,
одобренный code owner. Первая публикация локального состояния MUST
использовать отдельную bootstrap-ветку и PR в `develop`.

#### Scenario: Первая публикация repository

- **WHEN** maintainer публикует первоначальное проверенное состояние
- **THEN** оно отправляется в отдельную ветку и ожидает owner-approved PR в
  `develop` без автоматического merge
