## Information architecture

Landing строится как одна прокручиваемая страница с якорями:

1. Header: логотип `Agent Foundry`, navigation, текущая стабильная версия из
   release metadata и CTA «Скачать vX.Y.Z». Для current stable это `v1.0.0`.
2. Hero: единая точка настройки agents/AI, screenshot desktop-приложения,
   platform chips и primary download CTA.
3. Возможности: безопасная установка profiles, rules/skills import, задачи,
   templates и task blocks.
4. Как начать: скачать → выбрать проект → preview → применить → работать с
   задачами; ссылка на полную инструкцию обновления.
5. Current release: отдельная карточка последней stable версии с обнаруженной
   ОС, SHA-256/signature и CTA «Открыть страницу версии». На макете показан
   пример для macOS; production-страница подставляет Windows или Linux при
   соответствующем входе.
6. Release download и история: Stable/Beta switch, platform/architecture
   selector, explicit version selector и требования ОС; изменения выбранного
   релиза находятся отдельно от current release card.
7. Что изменилось: added/changed/fixed/security и выбор двух версий для diff.
8. Screenshots: Environment, import analysis, task editor, templates/blocks.
9. FAQ/safety и footer с current version, documentation/release links.

Global header содержит прямые пункты `Инструкция` и `Релизы`. Пункт
`Возможности` не является отдельной страницей или навигационным пунктом:
описание возможностей остаётся информационным блоком landing.

Во всех desktop/tablet header навигация состоит из `Главная`, `Инструкция` и
`Релизы`; active item имеет primary color и 2px underline. На mobile эти же
пункты выводятся отдельной компактной строкой под brand header, а не скрываются
за иконкой menu. Это сохраняет один и тот же набор навигации на всех маршрутах.

## Brand asset

Публичный бренд — `Agent Foundry`. Знак: две скобки, обозначающие agent
configuration, и центральная искра — создание/сборка. Этот знак используется в
header lockup, download CTA, application icon и browser favicon. Для favicon и
app icon применяются одинаковая геометрия и primary `#6750A4`; контрастная
белая версия используется в filled download CTA.

## Instructions page

Инструкции доступны по прямой ссылке из header. Desktop использует левое
содержание и основную область с пятью последовательными шагами, safety note и
реальными screenshots. Mobile показывает те же шаги вертикальными карточками,
затем safety note и ссылку на инструкцию обновления.

## Release catalog page

Отдельная страница открывается из CTA current release. На desktop она делится
на две колонки:

- Слева — `System` и `Version` selects, статус обнаруженной ОС, только
  совместимый download CTA и metadata выбранного artifact.
- Справа — прокручиваемая в пределах панели история releases в убывающем
  порядке, по паттерну GitHub Releases: version, status, date и группы
  added/changed/fixed/security. Пока существует только `v1.0.0`, архивные
  карточки в макете являются шаблоном структуры, а не опубликованными данными.

На mobile те же controls располагаются до changelog в одну колонку: system,
version, CTA и integrity metadata занимают верхнюю карточку, после которой
следует вертикальный список release notes. Технический выбор версии и ОС
идентичен desktop-странице.

## Responsive behavior

- Desktop: 1440px grid, 12 columns, full navigation, two-column hero,
  three/four-card feature grid.
- Tablet: 768px, collapsible navigation и все смысловые разделы desktop —
  инструкция, current release, история изменений, gallery и footer.
- Mobile: 390px, one column, sticky compact header, screenshot carousel,
  current release, инструкция, changelog и footer; tappable controls at least
  44px.

Mobile gallery — горизонтальный carousel: viewport показывает один экран,
следующие Environment import и Tasks/Templates доступны свайпом; progress dots
и текстовый счётчик сообщают текущий слайд. Заголовок gallery находится выше
viewport в отдельной зоне.

## Visual direction and Tailwind implementation

Landing использует ту же спокойную light palette, что Agent Foundry: светлый
лилово-белый background, white surfaces, `#6750A4` primary, dark neutral text,
green success и red error. Tailwind implementation задаёт эти значения как
semantic CSS variables и theme tokens; raw utility colors за пределами tokens
не допускаются. Typography: Inter/Roboto system stack. Cards: 16px radius,
тонкая neutral outline, мягкая elevation.

Screenshots используются как реальные изображения утверждённых Figma frames,
не как декоративные placeholders. До реализации потребуется export/asset flow
и права на изображения.

### Отступы карточек

Для desktop-карточек минимальный внутренний horizontal padding — 24px, для
release card — 32px; в tablet/mobile — 24px/20px соответственно. У button
остаётся собственный внутренний padding: контент не прижимается к outline
карточки или кнопки.

## Baseline and versioning

- `ai_rules_version` — immutable technical distribution ID, используемый в
  artifact name, release catalog и update metadata.
- `Agent Foundry` — public display name на landing и в приложении;
  `AI Works` — прежнее название дизайн-концепта. Технический ID при этом не
  меняется.
- `v1.0.0` — public stable baseline: Environment install/preview, rules/skills editor and
  import, Tasks/Templates/TaskBlocks and their local storage.
- Любой change указывает SemVer impact и compatibility impact. PATCH/MINOR
  запускают полный regression baseline; MAJOR дополнительно требует migration,
  rollback и upgrade tests from latest supported version.
- Каждый release содержит artifacts для macOS, Windows и Linux. Artifact
  metadata содержит ОС, architecture, checksum, signature и supported OS range.
- Landing определяет ОС на клиенте до отображения primary CTA и показывает
  только artifact этой ОС. Если ОС не поддержана либо не определена надёжно,
  primary CTA заменяется нейтральным переходом в release catalog без попытки
  предложить неправильный файл.

## Testing strategy

- Unit: version parsing, release metadata schema, selectors and changelog diff.
- Component: CTA, platform/version controls, keyboard focus and error states.
- Visual: 1440/768/390 screenshots against approved Figma states.
- E2E: stable download path, release comparison, docs navigation, existing
  desktop flows and persisted Task/Template/TaskBlock data after update.
- Accessibility: keyboard, landmarks, contrast, headings, reduced motion.
