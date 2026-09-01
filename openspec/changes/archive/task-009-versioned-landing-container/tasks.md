## 1. Versioned image

- [x] 1.1 Задать Docker image `agent-foundry-landing:<version>` и OCI metadata.
  — Проверка: compose без env строит тег `agent-foundry-landing:1.0.0`.
- [x] 1.2 Добавить безопасные переменные compose и документацию запуска. —
  Проверка: инструкция описывает build, run и откат конкретным тегом.

## 2. Verification and handoff

- [x] 2.1 Добавить статическую проверку container-конфигурации. — Проверка:
  `pnpm test` подтверждает версионное имя и default tag.
- [x] 2.2 Обновить current spec, выполнить проверки и архивировать change. —
  Проверка: `pnpm verify:sdd`, `pnpm test` и `docker compose config` проходят.
