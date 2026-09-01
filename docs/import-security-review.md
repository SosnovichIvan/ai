# Security review: импорт конфигурации

Проверено для scanner, preview и import transaction.

- Выбранный source normalizes через `canonicalize` и должен быть директорией.
- Scanner не следует symlink и исключает их до чтения.
- `.env`, token, cookie, credential, private key и технические каталоги
  исключаются по имени до чтения; в UI остаётся только счётчик.
- Destination принимает только относительные `rules/*.md` и
  `skills/*/SKILL.md`; выход за runtime root отклоняется.
- Add не перезаписывает конфликт, replace требует confirmation, backup и
  staging rename.
- Commit повторно сверяет preview с active catalog. Устаревший preview не
  изменяет каталог.
- Staging copy дополнительно пропускает symlink, чтобы локальный runtime
  каталог не мог разыменовать внешний файл.

Проверки: `cargo test`, `npm run lint`, `npm test`. Логи не содержат содержимое
проанализированных файлов или секретов.
