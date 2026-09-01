# Baseline release v1.0.0

## Идентичность релиза

- Public name: `Agent Foundry`.
- Technical distribution ID: `ai_rules_version`.
- Baseline SemVer: `v1.0.0`.

`ai_rules_version` применяется в именах поставляемых артефактов, release
catalog и update metadata. Это технический идентификатор, поэтому он не
заменяет отображаемое пользователю название продукта `Agent Foundry`.

## Зафиксированный функционал

В baseline входят следующие пользовательские сценарии:

1. Выбор директории проекта, проверка пути, preview/diff и безопасное
   применение installation profile с резервными копиями.
2. Просмотр и редактирование rules и skills, включая безопасный анализ внешней
   папки, классификацию Markdown-файлов и режимы merge/replace.
3. Workspaces, вложенные folders, изоляция задач, templates, blocks, rules и
   skills, а также независимое копирование между пространствами.
4. Создание, просмотр, редактирование и удаление задач, шаблонов и blocks.
5. Вставка текста task block в позицию курсора редактора задачи.
6. Локальное хранение и миграция данных без потери существующих Task,
   Template и TaskBlock.

## Обязательный regression suite

Перед PATCH, MINOR и MAJOR релизом выполняются:

```sh
npm run typecheck
npm run lint
npm test
npm run verify:sdd
cargo test --manifest-path src-tauri/Cargo.toml
npm run build:web
node scripts/verify-release-version.mjs vX.Y.Z
```

MAJOR дополнительно требует проверку migration, rollback и upgrade с последней
поддерживаемой версией. GitHub Actions запускает этот набор до создания
артефактов для macOS, Windows и Linux.

## Правило следующих релизов

Каждое изменение получает PATCH, MINOR или MAJOR SemVer-оценку и описывает
compatibility impact в OpenSpec. Процесс проверки отклоняет change без
`Previous`, `Target`, `Level`, `Rationale`, `Compatibility`, rollback и
regression plan.
