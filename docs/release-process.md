# Версии, обновления и каталог релизов

## Политика версий

Используем SemVer `MAJOR.MINOR.PATCH`. Release version одинакова в root
`package.json`, `src-tauri/tauri.conf.json` и `src-tauri/Cargo.toml`; релиз
закрепляется immutable tag `vX.Y.Z`.

| Тип | Когда применять |
| --- | --- |
| PATCH | Совместимое исправление без новой пользовательской возможности |
| MINOR | Новая обратимо совместимая возможность |
| MAJOR | Несовместимый flow, contract или обязательная migration данных |

Версии application и local data независимы. SQLite schema и runtime catalog
имеют собственные монотонные версии и migration history.

## Сохранность данных при update

Перед migration создаются локальные backups SQLite и runtime catalog. SQLite
меняется transactionally; catalog — через staging и atomic rename. Manifest
записывается только после успеха. При failure state восстанавливается из backup.
Старая application version не выполняет downgrade более нового state.

В SQLite backup и migration всегда входят все пользовательские сущности:
`Task`, `Template` и `TaskBlock`. Сохраняются UUID, текст, optional description,
timestamps, revision, `sourceTemplateId` и `blockSnapshot`. Template остаётся
независимой копией задачи; удалённый или изменённый блок не ломает сохранённый
snapshot задачи.

Совместимый update сохраняет локальные rules/skills в соответствующих
Workspace и не перезаписывает локальные edits или imported files. Новые
Workspace начинают с пустого каталога.

## Поставка desktop-пакета

Release pipeline собирает нативные artifacts для поддерживаемых платформ и
прикладывает checksums к immutable GitHub Release. Пользователь может либо
открыть готовый package своей ОС, либо запустить приложение из clone repository
по инструкции в корневом README. Исправления выходят новым PATCH release.
