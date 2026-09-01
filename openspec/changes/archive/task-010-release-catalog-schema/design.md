## Контекст

Каталог должен быть валидирован до того, как он попадёт в landing или release
artifact. Валидатор не открывает URL и не читает секреты: он проверяет только
JSON shape, SemVer, platform keys, URL и checksum format.

## Правила

Published artifact обязан иметь непустой URL, 64-символьный lowercase SHA-256
и непустую signature. Unavailable artifact обязан иметь пустой URL и marker
`PUBLISH_SHA256`; landing оставляет CTA disabled. Stable release должен
содержать macOS, Windows и Linux descriptors, а latest должен быть ровно один.

## Rollback и regression

Rollback сводится к публикации ранее валидного immutable JSON. Regression-тесты
покрывают baseline catalog и invalid fixtures: неверный checksum, лишний latest
и неполный published artifact. Ошибка validation останавливает CI до release.
