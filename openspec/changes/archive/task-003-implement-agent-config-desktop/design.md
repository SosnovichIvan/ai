## Context

Приложение работает локально и обязано быть удобным для пользователей без среды
разработки. Figma определяет интерфейс: `AI Works` с двумя рабочими областями,
установка окружения и задачи/templates. Системный выбор папки, валидация и
запись файлов должны выполняться нативным Rust core, а не frontend-кодом.

## Goals / Non-Goals

**Goals:** self-contained distribution; безопасные обратимые изменения файлов;
прозрачный progress; локальная работа с задачами; соответствие Figma.

**Non-Goals:** веб-сервис, удалённое хранение, командное исполнение и скрытая
запись в проект.

## Decisions

### Application boundaries

- **Shell:** Tauri v2. Сборка включает web assets и Rust runtime в готовый
  пакет; у конечного пользователя нет runtime-зависимости от Bun/Node/Rust.
- **UI:** React, TypeScript, Vite и дизайн-токены, соответствующие Figma.
  Node package manager используется только разработчиками и CI.
- **Core:** Rust crate без UI, который предоставляет typed commands для
  validate target, choose target result, preview, apply, rollback и state.
- **Persistence:** SQLite в app-data directory для задач, templates, task
  blocks и schema version; JSON manifest и файловые backups остаются в target.

### Installation transaction

1. UI открывает нативный folder picker; core нормализует/канонизирует путь и
   проверяет, что он существует и доступен для записи.
2. Core читает `ai/catalog.yaml`, вычисляет operations и возвращает preview:
   create, unchanged, update, conflict.
3. Apply допускается только для preview без unresolved conflict.
4. Core создаёт timestamped backups, пишет temp files в target, атомарно
   заменяет управляемые файлы, затем записывает manifest.
5. При ошибке transaction восстанавливает уже заменённые файлы и возвращает
   structured failure. Secrets и содержимое `.env` никогда не читаются.

### Task model

- `Task`, `Template` и `TaskBlock` имеют UUID, обязательный `name`, nullable
  `description`, timestamps и revision.
- `Task` хранит rich/plain text content, ссылку на source template (nullable) и
  snapshot использованных blocks.
- Нажатие card блока вызывает insert-at-cursor в editor; текст за пределами
  вставки сохраняется.
- Сохранение задачи как template создаёт независимую копию, не связанную с
  дальнейшим редактированием исходной задачи.
- Удаление требует confirmation и выполняется только локально.

### UI implementation

- Глобальная навигация — только блок `AI Works`; active area выделяется в нём.
- В Environment: native folder picker, read-only selected path, preview,
  progress (backup → rules → skills → manifest), completed и conflict.
- В Tasks: local segments `Задачи`, `Шаблоны`, `Блоки задач`; list использует
  compact cards, create/edit открываются в отдельной форме с required name и
  optional description.
- В редакторе справа расположены mini-card blocks с явно видимой подсказкой
  `Вставить в курсор`.

## Contracts / Data / Security

- IPC commands не принимают shell command, arbitrary source path или raw secret
  content; допустимы только typed request DTO.
- Rust core allowlists destinations из catalog и проверяет, что resolved path
  начинается с canonical target path.
- UI не получает содержимое конфликтующих secrets; error messages содержат
  только относительный managed path и безопасную причину.
- SQLite migration выполняется transactionally до открытия task workspace.

## Rollout / Rollback

- CI собирает signed/notarized packages там, где это требуется платформой.
- Установка профиля создаёт backups в `.agent-config/backups/<timestamp>/`.
- UI даёт rollback последней успешной операции, если manifest и backup
  согласованы; повреждённый manifest блокирует запись и предлагает recovery.
- Локальные data migrations имеют forward version и backup базы перед migration.

## Risks / Trade-offs

- Широкий файловый доступ опасен: выбранная папка должна быть единственным
  динамическим scope, предоставленным core после folder picker.
- Атомарная замена различается между платформами: core использует temp file в
  том же каталоге и platform-aware replace; recovery покрывается integration tests.
- Rich text editor сложнее plain text: MVP использует plain text/Markdown с
  сохранённой cursor position, без исполнения markup.
- Установщики требуют CI-настройки кодовой подписи, не локальные секреты в repo.
