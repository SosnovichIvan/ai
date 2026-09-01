## Контекст

Изменение не затрагивает UI. Image name выбирается по public product name в
lowercase: `agent-foundry-landing`. SemVer передаётся build-argument и tag в
compose, чтобы runtime и OCI metadata описывали одну и ту же версию.

## Безопасность и rollback

Контейнер остаётся статическим nginx без секретов, volumes и privileged mode.
Откат выполняется повторным запуском конкретного предыдущего image tag. Переменная
`LANDING_VERSION` не принимает команды и используется только как строка тега.

## Regression и проверка

Статический тест подтверждает image name, default version, build argument и
наличие инструкции запуска. Для production registry необходим отдельный change
с credentials и политикой публикации.
