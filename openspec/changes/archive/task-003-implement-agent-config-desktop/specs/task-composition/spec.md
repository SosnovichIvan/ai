## ADDED Requirements

### Requirement: Local task entities

Система SHALL хранить локальные Task, Template и TaskBlock с обязательным
названием и опциональным описанием и SHALL поддерживать create, edit и delete.

#### Scenario: Создание template

- **WHEN** пользователь заполняет обязательное название в форме template
- **THEN** save становится доступным и после сохранения template появляется в списке compact cards

### Requirement: Block insertion at cursor

Система SHALL показывать TaskBlock как compact cards с названием и описанием и
SHALL вставлять текст выбранного блока в текущую позицию курсора editor.

#### Scenario: Вставка блока в середину задачи

- **WHEN** пользователь ставит курсор между двумя фрагментами текста и нажимает карточку блока
- **THEN** текст блока появляется между фрагментами, а другие символы остаются неизменными

### Requirement: Independent template snapshot

Система SHALL сохранять текущую задачу как независимый Template snapshot.

#### Scenario: Изменение задачи после сохранения template

- **WHEN** пользователь редактирует задачу после `Сохранить как шаблон`
- **THEN** сохранённый template сохраняет прежнее содержимое до отдельного edit template
