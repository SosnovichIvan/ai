## 1. Анализ и SDD-основа

- [x] 1.1 Проанализировать правила, skills и OpenSpec-конфигурации проектов в `/Users/ivansosnovich/Documents/codex`. — Проверка: реестр источников, переносимых и исключённых практик находится в `ai/source-analysis.md`.
- [x] 1.2 Создать OpenSpec config, templates и current capability specs. — Проверка: все файлы `openspec/config.yaml`, `openspec/schemas/` и `openspec/specs/` существуют.

## 2. Единый каталог конфигурации

- [x] 2.1 Создать source-of-truth `ai/`, каталог профилей и переносимые rules/skills. — Проверка: `ai/catalog.yaml` ссылается только на существующие файлы.
- [x] 2.2 Зафиксировать безопасные инварианты установки. — Проверка: требования `installation-safety` покрывают validation, preview, conflict и recovery.
- [x] 2.3 Зафиксировать self-contained поставку и границы будущих task templates. — Проверка: current specs `product-delivery` и `task-templates` содержат проверяемые requirements.

## 3. Проверка и следующая фаза

- [x] 3.1 Валидировать структуру OpenSpec доступным проектным инструментом. — Проверка: `openspec validate --all --strict` завершилась с `3 passed, 0 failed`.
- [x] 3.2 Получить Figma URL с `node-id` для desktop UI и создать отдельный UI change. — Проверка: successor `task-003-implement-agent-config-desktop` содержит `APPROVED` Figma URL с node-id.
- [x] 3.3 Создать отдельный change для формата, хранилища и UX task templates. — Проверка: successor `task-003-implement-agent-config-desktop` содержит утверждённые Figma nodes и task-template scope.
