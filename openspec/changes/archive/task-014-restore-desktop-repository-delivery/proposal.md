## Why

Web-delivery, landing, server-side identity и Docker-стек больше не входят в
целевую модель продукта. Пользователь возвращает Agent Foundry к локальному
desktop-приложению и хочет получать его из репозитория: скачать исходный код,
установить зависимости через npm и запустить нативное приложение.

## What Changes

- Удаляются landing, web API, PostgreSQL/Docker delivery и связанные
  browser-first scripts и зависимости.
- Tauri снова становится единственным способом запуска продукта из
  репозитория; работа с папками, rules, skills, задачами, шаблонами и блоками
  остаётся локальной в desktop runtime.
- README описывает проверяемую установку из `git clone`, требования к Node.js,
  npm и Rust, запуск в development и сборку локального пакета.
- Незавершённый change web migration архивируется как отменённый решением
  владельца; его не следует применять или публиковать.

## Scope

- Репозиторий, package scripts, development documentation и OpenSpec.
- Удаление только web-delivery артефактов: `landing/`, `server/`,
  `docker-compose.web.yml` и browser-specific dependencies.
- Сохранение Tauri source, локального SQLite state и native release workflow.

## Non-Goals

- Выпуск подписанных production installers или обход требований сертификатов.
- Миграция browser/PostgreSQL accounts обратно в локальную SQLite без
  отдельного экспортного формата и согласованного сценария.
- Изменение функциональности локальной установки профилей.

## Version

- **Previous:** v1.0.0.
- **Target:** v1.0.0.
- **Level:** deferred MINOR.
- **Rationale:** способ delivery меняется по явному решению владельца, но
  публичная версия остаётся зафиксированной до отдельного release.
- **Compatibility:** существующие локальные desktop state и Tauri schema
  сохраняются; web runtime удаляется без автоматического удаления Docker
  volume или данных вне репозитория.

## Capabilities

### Modified Capabilities

- `product-delivery`: desktop запуск из cloned repository вместо web delivery.
- `product-landing`: capability удаляется из текущего продукта.
