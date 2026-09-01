## Context

Версия application сейчас задаётся в Tauri и Cargo. Пользовательские данные
хранятся локально: SQLite для задач и app-data runtime catalog с imports/backups.
Обновление пакета заменяет код приложения, но не должно необратимо менять эти
данные. Landing в будущем должен быть витриной release metadata, а не отдельным
источником правды.

## Decisions

### Единая версия релиза

- Единственная release version в формате SemVer находится в root `package.json`;
  CI проверяет её равенство `src-tauri/tauri.conf.json` и `Cargo.toml`.
- `MAJOR` — несовместимый формат данных/контрактов или новый обязательный путь;
  `MINOR` — новая совместимая возможность; `PATCH` — исправление без нового
  пользовательского поведения.
- Каждый релиз имеет immutable git tag `vX.Y.Z`, дату, min supported predecessor
  и структурированный changelog: `added`, `changed`, `fixed`, `security`,
  `deprecated`, `knownIssues`.

### Разделение application и data versions

Application SemVer не равна schema version. Runtime state содержит отдельные
монотонные значения:

- SQLite `PRAGMA user_version` для задач/templates/blocks;
- `catalog-manifest.json.version` для runtime catalog;
- `state-manifest.json` с последней успешной application version, completed
  migration IDs и timestamp.

Каждая migration имеет ID, from/to schema, проверяемую функцию `up`, optional
tested `down` и описание recovery. Релиз не запускает миграцию повторно, если
ID уже отмечен completed.

### Миграция задач, шаблонов и блоков задач

Все пользовательские сущности мигрируют в одной SQLite transaction: `Task`,
`Template` и `TaskBlock`. Migration сохраняет UUID, name, nullable description,
content, created/updated timestamps, revision, `sourceTemplateId` и
`blockSnapshot`. Нельзя очистить, пересоздать или «нормализовать» эти таблицы
простым импортом из нового bundled state.

- Template остаётся независимым snapshot после upgrade: изменение task не
  меняет уже сохранённый template.
- Task с сохранённым block snapshot продолжает открываться, даже если исходный
  TaskBlock удалён либо изменён позже.
- Новое required поле добавляется сначала nullable/default-compatible; backfill
  выполняется и проверяется до введения `NOT NULL`/нового ограничения.
- Удаление/переименование колонок проходит staged migration и допускается
  только после compatibility-window и доказанной backup/recovery процедуры.

### Безопасный update protocol

1. Пользователь скачивает только опубликованный пакет своей платформы; landing
   показывает размер, checksum SHA-256, подпись и минимальную поддерживаемую ОС.
2. После запуска новой версии core определяет data versions до открытия UI.
   Если данные новее текущего приложения, запуск блокируется с recovery notice;
   downgrade автоматически не выполняется.
3. До первой изменяющей migration создаётся timestamped backup всего SQLite
   файла (включая tasks, templates и task blocks) и runtime catalog. Backup
   сохраняется локально на том же volume и не попадает в release artifact.
4. SQLite migration выполняется одной transaction; catalog migration — через
   staging directory и atomic rename. State manifest записывается последним.
5. При ошибке приложение возвращает backup, оставляет диагностический marker и
   открывает recovery screen. Исходные данные не удаляются до успешного запуска
   новой версии.

### Совместимость runtime catalog

Bundled `ai/` — seed текущей версии. При compatible MINOR/PATCH update
добавляются только отсутствующие seeded files; пользовательские edits и imports
не перезаписываются. Изменение семантики существующего seed-файла создаёт новый
versioned path либо требует явного migration preview, но не тихую замену.

### Release catalog for landing

Репозиторий публикует signed static JSON `releases/index.json` и immutable
`releases/vX.Y.Z.json`. Landing читает только этот каталог:

```json
{
  "version": "0.2.0",
  "releasedAt": "2026-08-30",
  "channel": "stable",
  "minimumSupportedVersion": "0.1.0",
  "changes": {"added": [], "changed": [], "fixed": [], "security": []},
  "artifacts": [{"platform": "macos-arm64", "url": "...", "sha256": "...", "signature": "..."}]
}
```

Каталог содержит stable/beta channel, supported platforms, deprecation и known
issues. Landing сортирует релизы по SemVer, даёт выбор конкретной версии и
показывает отличия между выбранной и текущей через структурированные change
groups. URL download ведёт на immutable artifact, checksum и signature лежат
рядом с ним. История релизов не редактируется после публикации; исправление
публикуется новым PATCH.

## Alternatives considered

- **Только application version:** нельзя безопасно определить состояние старой
  SQLite/catalog schema.
- **Перезаписывать runtime catalog новым seed:** уничтожает пользовательские
  edits/imports.
- **Хранить changelog только Markdown на landing:** затрудняет фильтрацию,
  выбор версии и автоматическую проверку артефактов.
- **Автоматический downgrade:** может потерять данные; запрещён по умолчанию.

## Risks / Rollback

- Migration может быть необратимой. До неё обязательны backup, dry-run/test
  fixture и compatibility matrix.
- Подмена artifact опасна. Release считается доступным только после checksum,
  подписи и immutable tag; landing не является доверенным источником бинарника
  без этих проверок.
- Повреждённый backup не даёт ложного успеха: recovery screen сообщает точный
  локальный путь и предлагает не стирать исходный state.
