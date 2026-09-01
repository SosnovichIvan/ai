## Why

Лендинг уже может запускаться в Docker, но compose не задаёт имя образа и не
фиксирует версию. Это не позволяет однозначно развернуть или откатить конкретную
версию сайта.

## What Changes

- Вводится versioned Docker image `agent-foundry-landing:<SemVer>`.
- Compose получает параметр `LANDING_VERSION` со значением `1.0.0` по умолчанию.
- Docker image получает OCI metadata с названием продукта и версией.
- Добавляется инструкция локальной сборки и запуска.

## Scope

- `landing/Dockerfile`, `landing/docker-compose.yml` и документация запуска.
- Статические проверки формата контейнерной конфигурации.

## Non-Goals

- Публикация образа в registry, изменение desktop-бинарника или landing UI.
- Автоматическое обновление контейнера на production-хосте.

## Version

- **Previous:** v1.0.0.
- **Target:** v1.0.0.
- **Level:** deferred PATCH.
- **Rationale:** добавляется воспроизводимая operational-конфигурация без
  изменения публичного продукта.
- **Compatibility:** существующий `docker compose up --build` продолжает
  работать; отсутствие переменной использует тег `1.0.0`.

## Capabilities

### Modified Capabilities

- `product-landing`: воспроизводимое container-развёртывание лендинга.
