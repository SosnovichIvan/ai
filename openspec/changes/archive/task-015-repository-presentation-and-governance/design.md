## Решение

README использует проверяемые, versioned изображения из `docs/images/` и не
зависит от внешних сроков жизни URL. Скриншоты показывают реальные согласованные
desktop-сценарии: работа с задачей и список сущностей.

Защита веток реализуется двумя уровнями. `CODEOWNERS` назначает
`@SosnovichIvan` владельцем всего исходного кода. GitHub ruleset для `main` и
`develop` требует pull request, минимум один approval и approval code owner,
запрещая force push и deletion веток. GitHub Actions остаётся отдельным
required status check после того, как его имя стабилизируется в repository.

## Безопасность

Настройка branch rules выполняется только GitHub API с авторизацией владельца
repository. Token не записывается в файл, не передаётся в command history и не
выводится в логи. В workflow сохраняется минимальное право `contents: write`,
необходимое для публикации release.

## Rollback

README, изображения, CODEOWNERS и workflow откатываются обычным revert commit.
GitHub ruleset изменяется через GitHub UI или API владельцем; при блокирующей
ошибке его можно временно перевести в evaluate mode, не отключая журнал
проверок.

## Regression

Проверяются Markdown links, отсутствие внешних секретов, `npm run typecheck`,
`npm run lint`, `npm test`, Rust tests и сборка Tauri. Перед применением ruleset
проверяется, что он направлен ровно на `main` и `develop`, требует PR и code
owner approval, но не разрешает bypass для пользователей.
