## 1. Release schema

- [x] 1.1 Реализовать валидатор release catalog. — Проверка: baseline catalog
  принимается, invalid fixtures отклоняются.
- [x] 1.2 Подключить validation к release pipeline. — Проверка: workflow
  запускает validator до публикации.

## 2. Verification and handoff

- [x] 2.1 Обновить current spec, выполнить regression checks и архивировать
  change. — Проверка: `pnpm test`, `pnpm verify:sdd` и validator проходят.
