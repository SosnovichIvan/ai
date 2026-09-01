## 1. Baseline and release policy

- [x] 1.1 Зафиксировать `ai_rules_version` как technical distribution ID и `v1.0.0` как baseline release. — Проверка: release metadata, artifact naming и документация используют один ID и version.
- [x] 1.2 Добавить SemVer impact и backward-compatibility section в каждый новый change. — Проверка: validation fixture отклоняет change без version rationale или regression plan.
- [x] 1.3 Определить baseline regression suite для Environment, rules/skills import, Tasks, Templates и TaskBlocks. — Проверка: suite запускается для PATCH/MINOR/MAJOR и публикует результат.
- [x] 1.4 Определить release artifacts для macOS, Windows и Linux. — Проверка: один SemVer release catalog содержит проверяемые metadata и installable artifact каждой поддерживаемой ОС.
- [x] 1.5 Реализовать tag-driven CI release pipeline для macOS, Windows и Linux. — Проверка: workflow запускает regression gates, публикует installable artifacts, `SHA-256SUMS` и release metadata без секретов в логах.

## 2. Landing design

- [x] 2.1 Создать Figma desktop/tablet/mobile frames и записать прямые node URL после user approval. — Проверка: design gate содержит `APPROVED`, URL с node-id и evidence всех ключевых states.
- [x] 2.2 Описать Tailwind semantic tokens, component map и screenshot asset workflow. — Проверка: implementation design handoff не содержит raw color decisions или placeholder screenshots.
- [x] 2.3 Подготовить общий brand asset для header, download CTA, favicon и application icon. — Проверка: browser и установленное приложение используют один векторный знак Agent Foundry.

## 3. Landing implementation

- [x] 3.1 Реализовать landing на Tailwind CSS согласно утверждённому Figma. — Проверка: responsive visual checks проходят на 1440/768/390.
- [x] 3.2 Реализовать release/version selector, changelog diff, OS detection и verified download metadata. — Проверка: E2E определяет macOS/Windows/Linux, показывает только совместимый artifact и выводит checksum/signature.
- [x] 3.3 Выполнить accessibility и baseline regression tests. — Проверка: keyboard/a11y suite и existing desktop flows зелёные.
- [x] 3.4 Реализовать общую навигацию, favicon и platform application icon. — Проверка: все маршруты имеют identical navigation и корректный active state; browser tab и собранные macOS/Windows/Linux artifacts используют brand icon.

## 4. Public stable v1.0.0 and data clearance

- [x] 4.1 Получить approval Figma для интерактивного preview и destructive cleanup states. — Проверка: `design-approval.md` содержит `APPROVED` и node-id desktop/tablet/mobile.
- [x] 4.2 Реализовать переключение preview в hero без изменения layout и с клавиатурной доступностью. — Проверка: каждая tab показывает соответствующий сценарий, активная tab имеет корректные ARIA-state.
- [x] 4.3 Реализовать безопасное удаление одной сущности и категории для Tasks, Templates, TaskBlocks, rules и skills. — Проверка: confirmation показывает scope, cancel не меняет данные, успешное действие обновляет список.
- [x] 4.4 Реализовать полное очищение локальных данных проекта с отдельным подтверждением и понятным перечнем удаляемого. — Проверка: очистка не выходит за data directory приложения и не затрагивает выбранный пользователем проект без отдельной install operation.
- [x] 4.5 Поднять package, Tauri и landing release metadata до `v1.0.0`; подготовить фактический download artifact и SHA-256. — Проверка: установленное приложение, release catalog и CTA согласованно показывают v1.0.0 и ведут на доступный файл.
