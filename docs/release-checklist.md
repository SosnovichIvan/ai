# Immutable release checklist

Каждый public release создаётся новым immutable Git tag `vX.Y.Z`. Уже
опубликованный tag, artifact, checksum и changelog не изменяются: исправление
выходит следующим PATCH release.

## До создания tag

1. Выберите SemVer согласно `docs/release-process.md`.
2. Синхронизируйте версию в `package.json`, `src-tauri/tauri.conf.json` и
   `src-tauri/Cargo.toml`.
3. Запустите `node scripts/verify-release-version.mjs vX.Y.Z`.
4. Запустите `npm run lint`, `npm test`, `npm run verify:sdd` и `cargo test`
   из `src-tauri/`.
5. Подготовьте structured changelog:

```json
{
  "Добавлено": ["Новая пользовательская возможность"],
  "Изменено": ["Совместимое изменение поведения"],
  "Исправлено": ["Исправленная ошибка"],
  "Безопасность": ["Проверка или защита"],
  "Известные ограничения": ["Ограничение, если оно есть"]
}
```

## Публикация

1. Создайте и отправьте tag `vX.Y.Z`.
2. CI собирает native artifacts для macOS, Windows и Linux, генерирует
   SHA-256SUMS и metadata релиза.
3. Проверьте, что каждый published artifact имеет URL, checksum и signature.
4. Не заменяйте опубликованные файлы: при ошибке создайте новый PATCH tag.
