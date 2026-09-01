# Аудит исходных правил и skills

Дата среза: 2026-08-29.

## Границы аудита

Проверены репозитории в `/Users/ivansosnovich/Documents/codex`, исключая
служебные и сгенерированные каталоги (`.git`, `node_modules`, `dist`, `.next`).
Материалы для агентной работы найдены в трёх источниках: `my-sport-life`,
`relayform` и `arhdesign`. В каталогах `2026-08-14`, `2026-08-26`,
`relayform-design` и `resume` таких материалов не обнаружено.

Цель аудита — извлечь переносимые принципы, а не скопировать исходные файлы.
Правило становится частью поставки только после явного включения в `ai/rules/`,
`ai/skills/` и `catalog.yaml`.

## Реестр источников и решения

| Источник | Просмотренные материалы | Переносимые выводы | Что не переносится автоматически |
| --- | --- | --- | --- |
| `my-sport-life` | `AGENTS.md`; 11 разделов `ai-agent/rules` (lifecycle, frontend, backend, database, contracts, devops, security, testing, observability, git, documentation); 11 ролевых skills; OpenSpec skills в `.agents/skills` | Порядок OpenSpec «спецификация → change artifacts → проверка → archive»; роли и границы автоматизации; проверяемые критерии готовности; безопасность, документация и тестирование как часть DoD | Go, gRPC, Kubernetes, NATS, конкретные БД, FSD-пути, branch-модель, сообщения коммитов, внутренние пакеты и Figma-конвенции продукта |
| `relayform` | `.ai/project-rules.md`; `context/`; SDD-шаблон; skills `designer`, `frontend`, `review`; stack/review reference | Разделение контекста проекта, правил и skills; явные UI-состояния; дизайн-токены и доступность; ревью с конкретными замечаниями; шаблон сценариев и плана проверки | Next.js, Tailwind, FSD-нейминг и файловая структура, выбранные библиотеки и продуктовые токены |
| `arhdesign` | `.ai/README.md`; `context/` (state, decisions, feedback); skills `arhdesign-designer`, `arhdesign-frontend`, `arhdesign-review` | Журнал решений и обратной связи; согласование дизайна до реализации UI; источник истины для текущего состояния; разделение проектных ограничений и универсальных практик | Архитектура и контент сайта, SEO-подход, UI-детали, стек и naming конкретного продукта |

## Итоговая единая точка правды

После повторного детального аудита 2026-08-29 из исходников сформирована
переносимая, но предметная поставка. Она сохраняет работающие принципы, не
перенося жёсткие зависимости конкретного продукта:

- rules: `core`, `installation`, `security`, `lifecycle`, `frontend`, `backend`,
  `database`, `contracts`, `devops`, `testing`, `observability`, `git`,
  `documentation`, `review`;
- skills: `sdd-workflow`, `project-bootstrap`, `analyst`, `frontend`, `backend`,
  `designer`, `reviewer`, `tester`, `quality`, `database`, `infrastructure`,
  `observability`, `messaging`, `orchestrator`;
- `catalog.yaml` — декларация состава и целевых путей профиля.

В rules сохранены конкретные правила написания кода: слои и public API во
frontend, handler/service/client и boundary validation во backend, contract-first
и генерация, миграции/транзакции, тестовые уровни и observability. Убраны только
продуктовые ссылки, обязательные зависимости вроде конкретного UI-kit, Go, NATS,
Kubernetes, branch-name и доменные детали — их пользователь может подключить
через импорт внешней папки отдельным набором.

## Очередь развития

Ролевые навыки (аналитик, разработчик, дизайнер, ревьюер, тестировщик) могут
появиться как отдельные, версионируемые профили после подтверждения их
универсального содержания. Их добавление требует OpenSpec change, обновления
`catalog.yaml` и показа diff перед установкой.
