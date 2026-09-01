## ADDED Requirements

### Requirement: Responsive product landing

Система SHALL предоставить адаптивный landing для desktop, tablet и mobile,
объясняющий продукт, возможности, начало работы, безопасность и доступные
релизы Agent Foundry.

#### Scenario: Первый визит с мобильного устройства

- **WHEN** пользователь открывает landing на mobile viewport
- **THEN** он видит понятный hero, возможности, инструкцию и доступное действие
  скачивания без горизонтальной прокрутки

### Requirement: Глобальная навигация и инструкции

Landing и связанные страницы SHALL содержать header с прямой навигацией к
«Инструкции» и «Релизы». Пункт «Возможности» MUST NOT быть отдельным пунктом
навигации. Страница инструкций SHALL описывать скачивание, выбор проекта,
preview, применение profile, работу с задачами и безопасное обновление.

#### Scenario: Переход к инструкциям из header

- **WHEN** пользователь нажимает «Инструкция» в header
- **THEN** открывается адаптивная страница с последовательными шагами и
  safety guidance без необходимости возвращаться к landing

### Requirement: Единая адаптивная навигация

Все public pages SHALL содержать одинаковые пункты `Главная`, `Инструкция` и
`Релизы`. Текущий маршрут MUST иметь visual active state с primary underline.
На mobile пункты остаются доступны в компактной navigation row.

#### Scenario: Просмотр страницы Релизы на tablet

- **WHEN** пользователь открывает release catalog на tablet
- **THEN** он видит все три пункта global navigation, а `Релизы` подчёркнуты
  как текущая страница

### Requirement: Единый логотип для скачивания и браузера

Система SHALL использовать единый знак Agent Foundry в download CTA, favicon
browser tab и platform application icon.

#### Scenario: Скачивание приложения

- **WHEN** пользователь выбирает совместимый installer
- **THEN** CTA и скачанный application artifact используют один узнаваемый
  brand mark Agent Foundry

### Requirement: Version-aware download

Landing SHALL позволять выбрать stable/beta channel, версию и platform artifact,
показывая checksum, signature, OS support и изменения выбранного релиза.

#### Scenario: Выбор предыдущей версии

- **WHEN** пользователь выбирает опубликованную предыдущую версию
- **THEN** landing показывает её changelog и соответствующие artifact metadata,
  не подменяя их последним релизом

### Requirement: Release catalog controls and changelog

Release catalog SHALL показывать слева два независимых select controls:
систему и версию. Ниже система MUST показать download CTA только для
совместимого artifact. Справа SHALL быть прокручиваемый changelog опубликованных
версий от последней к ранним.

#### Scenario: Смена версии

- **WHEN** пользователь меняет версию в select
- **THEN** metadata и download CTA слева обновляются для этой версии, а справа
  соответствующая release card становится видимой в changelog

#### Scenario: Просмотр истории

- **WHEN** changelog содержит больше release cards, чем помещается в панель
- **THEN** пользователь прокручивает только панель changelog и видит версии в
  порядке убывания SemVer

### Requirement: Download для обнаруженной ОС

Landing SHALL определять ОС посетителя на стороне клиента и показывать primary
download CTA только для совместимого artifact. Поддерживаемые ОС: macOS,
Windows и Linux.

#### Scenario: Вход с Windows

- **WHEN** landing обнаруживает Windows
- **THEN** основной CTA предлагает только Windows artifact выбранной версии и
  не предлагает macOS или Linux installer

#### Scenario: Неподдерживаемая или неопределённая ОС

- **WHEN** ОС не поддержана либо сигнал браузера не позволяет определить её
  надёжно
- **THEN** landing не предлагает файл по умолчанию и направляет пользователя в
  release catalog с пояснением

### Requirement: Mobile screenshot carousel

Mobile landing SHALL показывать product screenshots в горизонтальном carousel
с одним видимым экраном, доступом к каждому screenshot свайпом, индикаторами и
текстовой подписью текущего слайда.

#### Scenario: Просмотр следующего экрана на mobile

- **WHEN** пользователь свайпает carousel влево
- **THEN** показывается следующий screenshot, а индикатор и подпись меняются
  без наложения на заголовок или содержимое карточки
