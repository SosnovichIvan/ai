## Context

`GITHUB_REF_NAME` в tag workflow содержит `v1.0.0`. В тестовом процессе
`execFile` наследует окружение runner, поэтому неверный аргумент `latest`
подменялся значением среды.

## Decision

Скрипт читает `process.argv[2]` первым, а `GITHUB_REF_NAME` использует только
если явный аргумент не передан. Это сохраняет поведение release workflow и
делает тест независимым от окружения runner.

## Regression plan

- `npm test`;
- `npm run typecheck`;
- `npm run lint`;
- `npm run verify:sdd`.

## Rollback plan

Если проверка выявит несовместимость, revert этого единственного commit через
pull request вернёт прежний порядок источников tag без затрагивания данных или
версии приложения.
