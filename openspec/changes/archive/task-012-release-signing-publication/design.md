## Контекст

Сборка запускается исключительно для immutable тега `vX.Y.Z`. До публикации
workflow выполняет regression gates, собирает artifact на целевой ОС, считает
SHA-256 и создаёт detached signature. Закрытые материалы передаются как GitHub
Secrets только в соответствующий job и не записываются в рабочее дерево,
release-каталог или логи.

## Платформы

| Платформа | Требуемый механизм | Результат |
| --- | --- | --- |
| macOS | Apple Developer ID / notarization credentials | signed и notarized DMG |
| Windows | code-signing certificate из GitHub Secret | signed MSI и NSIS installer |
| Linux | GPG detached signature | `.sig` рядом с AppImage и DEB |

## Rollback и regression

Если signing или verification завершается ошибкой, job останавливается до
создания GitHub Release; уже скачанные временные artifacts не публикуются.
Rollback — удалить не опубликованные workflow artifacts и исправить секреты,
не меняя immutable тег. Regression проверки остаются обязательными до каждого
signing job: typecheck, lint, unit, e2e, Rust tests, release catalog validation
и version check.

## Безопасность

- У workflow минимальные permissions; публикация получает только `contents: write`.
- Secrets доступны только trusted repository context, не pull request workflow.
- Логи показывают fingerprint и статус проверки, но не ключ, пароль или base64
  содержимое сертификата.
