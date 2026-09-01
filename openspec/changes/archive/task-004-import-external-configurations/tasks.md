## 1. Design gate and catalog foundation

- [x] 1.1 Подтвердить Figma scenario `51:62` и изменить status design gate на `APPROVED`. — Проверка: `design-approval.md` содержит явный user-confirmation и прямой URL с node-id.
- [x] 1.2 Реализовать versioned runtime catalog, initial seed из bundled `ai/` и manifest происхождения. — Проверка: чистый app-data получает ожидаемые rules/skills, повторный старт не перезаписывает пользовательские изменения.
- [x] 1.4 Расширить versioned `ai/` переносимыми rules и ролевыми skills из утверждённого аудита исходных репозиториев. — Проверка: `catalog.yaml` перечисляет каждый файл, parser профиля принимает набор, а установка создаёт все заявленные skills.
- [x] 1.3 Заменить статический список SourceEditor на данные runtime catalog и безопасно сбрасывать выбранный элемент при смене категории. — Проверка: SourceEditor получает список из runtime catalog, сбрасывает selected на первый доступный файл и проверяет границы индекса.

## 2. Safe analysis and preview

- [x] 2.1 Реализовать нативный выбор папки и typed analyse command. — Проверка: Tauri dialog возвращает только выбранную папку, а typed analyse command нормализует и проверяет путь.
- [x] 2.2 Реализовать allowlist scanner, canonical containment, no-symlink policy и secret/technical exclusions. — Проверка: Rust tests создают mixed fixture и подтверждают, что допустимые Markdown найдены, а `.env`, keys и ignored directories не прочитаны и не появились в логах.
- [x] 2.4 Классифицировать допустимые `.md` из выбранного root и подкаталогов по контексту пути и признакам содержимого. — Проверка: Rust test выбирает непосредственно `rules/` и `skills/`, корректно разделяет кандидаты и показывает счётчик нераспознанных Markdown.
- [x] 2.3 Реализовать immutable import preview с categories, relative paths, hashes и conflicts. — Проверка: изменение активного catalog после preview делает commit недействительным и блокирует замену.

## 3. Import transaction and UI

- [x] 3.1 Реализовать mode `Добавить`: новые пути добавляются, path/hash conflicts не перезаписываются. — Проверка: integration test подтверждает сохранение активного конфликтующего файла.
- [x] 3.2 Реализовать mode `Заменить`: explicit confirmation, backup, atomic commit и rollback. — Проверка: Rust test подтверждает explicit confirmation, backup и сохранность каталога при отклонённом устаревшем preview.
- [x] 3.3 Реализовать Figma UI `51:62`: анализ, counts, exclusions, switch mode и переход к preview. — Проверка: manual desktop review пользователя подтвердил сценарий; frontend вызывает native picker, показывает counts, режимы и preview до apply.
- [x] 3.4 Реализовать просмотр разрешённых найденных файлов и active runtime catalog в Skills/Rules. — Проверка: manual desktop review пользователя подтвердил Skills/Rules editor; SourceEditor показывает allowlisted runtime entries и записывает текст только по explicit save.

## 4. Verification and handoff

- [x] 4.1 Выполнить security review scanner/import transaction. — Проверка: `docs/import-security-review.md`, targeted Rust tests, lint и typecheck подтверждают no-symlink/no-secret policy.
- [x] 4.2 Обновить README: импорт, add/replace, preview, backup/recovery и расположение local data. — Проверка: README описывает импорт, preview, backup/recovery и запуск без developer toolchain.
- [x] 4.3 Выполнить OpenSpec verify, обновить current specs и архивировать change. — Проверка: current `configuration-catalog` spec обновлён, Rust/Node checks и OpenSpec verify проходят.
