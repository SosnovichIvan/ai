## 1. Project bootstrap and delivery

- [x] 1.1 Создать Tauri v2 workspace с React/TypeScript/Vite frontend и изолированным Rust `installation-core`. — Проверка: `pnpm install`, `pnpm lint`, `pnpm typecheck` и `cargo check` завершаются успешно.
- [x] 1.2 Настроить production packaging без runtime-зависимости пользователя от Bun/Node/Rust. — Проверка: `pnpm tauri build --bundles app` создаёт `Agent Foundry.app`; конечный пакет не требует developer toolchain.
- [x] 1.3 Настроить capability/IPC boundary и logging policy. — Проверка: `docs/desktop-security-review.md` фиксирует ограниченные capabilities, typed IPC и отсутствие shell/filesystem permission.

## 2. Secure profile installation core

- [x] 2.1 Реализовать parser и validator `ai/catalog.yaml`, включая profile resolution. — Проверка: Rust unit tests покрывают valid catalog, missing file, path traversal и неизвестный profile.
- [x] 2.2 Реализовать target validation, canonical path containment и managed manifest diff. — Проверка: unit/integration tests во временных каталогах возвращают create/unchanged/update/conflict без записи в preview.
- [x] 2.3 Реализовать transaction: backups, temp write, atomic replace, manifest after success и rollback on failure. — Проверка: integration test инжектирует write failure и подтверждает восстановление всех изменённых файлов.
- [x] 2.4 Реализовать typed Tauri commands и progress events для folder selection, preview, apply и rollback. — Проверка: Rust test подтверждает порядок rules → skills → manifest; native command передаёт backup event перед apply.

## 3. Environment UI

- [x] 3.1 Реализовать `AI Works` navigation и Environment workflow по nodes `3:4`, `21:19`, `21:61`, `21:104`, `21:143`. — Проверка: `App.test.tsx` подтверждает native selection, preview и disabled/active apply state.
- [x] 3.2 Подключить нативный folder picker и read-only selected path. — Проверка: пользователь подтвердил native dialog в собранном `Agent Foundry.app`; ручное поле пути отсутствует.
- [x] 3.3 Реализовать Skills/Rules text viewer-editor с unsaved state, explicit save и cancellation. — Проверка: `App.test.tsx` проверяет Skills/Rules switch, explicit save и cancellation без неявной записи.

## 4. Local task composition

- [x] 4.1 Реализовать versioned SQLite schema и repositories для Task, Template и TaskBlock. — Проверка: migration/repository tests покрывают CRUD, required name и nullable description.
- [x] 4.2 Реализовать списки compact cards и отдельные create/edit/delete forms по Figma nodes `34:70`, `34:116`, `34:160`, `37:106`, `37:154`, `37:200`. — Проверка: `App.test.tsx` покрывает required name, optional description и typed create form; Rust tests покрывают CRUD.
- [x] 4.3 Реализовать task text editor и insert-at-cursor cards по node `33:62`. — Проверка: `App.test.tsx` вставляет block в середину текста без изменения окружающего content.
- [x] 4.4 Реализовать «Сохранить как шаблон» как независимый snapshot. — Проверка: UI test вызывает saveTaskAsTemplate, Rust test `template_is_an_independent_snapshot` подтверждает независимость snapshot.

## 5. Verification and handoff

- [x] 5.1 Добавить accessibility, responsive desktop, error and visual regression checks для утверждённых UI states. — Проверка: пользователь подтвердил visual review; React Testing Library покрывает 7 desktop scenarios, Playwright покрывает landing navigation/OS states.
- [x] 5.2 Выполнить security review installation transaction и проверку отсутствия secret leakage. — Проверка: desktop/import security reviews и targeted Rust tests подтверждают containment и no-secret policy.
- [x] 5.3 Обновить README с user install/run, local data location, backup/recovery и developer build workflow. — Проверка: README и `docs/website-update-guide.md` описывают готовый пакет, app-data, backup/recovery и developer build workflow.
- [x] 5.4 Выполнить OpenSpec verify, синхронизировать current specs и архивировать change после реализации. — Проверка: current specs синхронизированы позднейшими changes; repository checks, production build и user smoke проходят.
