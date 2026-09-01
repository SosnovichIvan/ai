## Решение

Публикация выполняется из ветки `codex/bootstrap-repository` в защищённую
ветку `develop`. Ветка создаётся от текущего локального состояния после
проверки состава staging area. PR не сливается автоматически: его review и
merge остаются действием владельца `@SosnovichIvan` в GitHub.

В репозиторий не попадают `node_modules`, build output, `.env`, результаты
тестов и иные локальные служебные данные. Перед push выполняется установленный
набор regression-проверок.

## Безопасность

В коммит не добавляются секреты, токены, cookies, приватные ключи или `.env`.
Авторизация Git выполняется через уже настроенный SSH remote; учётные данные не
читаются и не выводятся.

## Rollback

До merge PR можно закрыть или удалить bootstrap-ветку. После merge изменения
откатываются обычным revert PR в соответствии с ruleset.

## Regression

Выполняются `npm run typecheck`, `npm run lint`, `npm test`, `cargo test` и
`npm run verify:sdd`. Перед push проверяется список staged файлов.
