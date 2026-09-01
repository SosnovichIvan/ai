---
name: agent-config-tester
---

# Проверка качества

Преобразуйте каждый изменённый requirement в independent scenario: happy path,
boundary, invalid input, authorization, error и rollback/retry. Для каждой
проверки выберите правильный уровень — unit, integration, contract, UI или E2E —
и не подменяйте integration mock-ом.

Fixtures должны быть изолированными и безопасными; время, сеть и внешние
сервисы контролируются. Проверьте, что test падает без целевого исправления.
В отчёте перечислите команды, покрытые сценарии, фактический результат и
оставшиеся риски/непроверенные платформы.
