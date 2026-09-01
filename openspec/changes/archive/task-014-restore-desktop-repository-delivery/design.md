## Решение

Единственный runtime продукта — Tauri desktop application. UI получает
нативный доступ к выбору директории и локальному app-data, а installation core
сохраняет preview, merge/replace и backup семантику.

Репозиторий остаётся developer distribution: пользователь клонирует его,
устанавливает Node.js/npm и Rust, затем запускает `npm run tauri dev`. Это не
заменяет будущий release package, но исключает необходимость landing, web API,
PostgreSQL и Docker для локальной работы.

## Альтернативы

- Сохранить landing только как статическую документацию — отклонено: владелец
  явно запросил удалить landing полностью.
- Сохранить web API как необязательный сервис — отклонено: это добавляет
  неиспользуемую инфраструктуру и второй источник состояния.
- Удалить Docker volumes автоматически — отклонено: они находятся вне
  репозитория и могут содержать пользовательские данные; удаление не входит в
  scope и не выполняется без отдельной команды владельца.

## Безопасность и данные

Удаление файлов web-delivery из repository не читает и не копирует secrets.
Локальный SQLite state desktop приложения не изменяется. README явно отделяет
developer prerequisites от запуска готового release package.

## Rollback

Rollback выполняется возвратом commit, содержащего этот change. Никакие
Docker volumes, PostgreSQL данные или local desktop state данной реализацией не
удаляются, поэтому восстановление кода не требует восстановления данных.

## Regression

До завершения проверяются `npm ci`, typecheck, lint,
unit tests, Tauri build и Rust tests. README команды должны быть воспроизводимы
из чистого clone с указанными prerequisites.
