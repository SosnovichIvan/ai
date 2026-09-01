---
name: agent-config-observability
---

# Наблюдаемость

Для операции зафиксируйте service-level outcome: volume, success/error rate,
latency и saturation. Добавьте структурированный log с correlation ID, trace
span вокруг внешней границы и метрики с ограниченной cardinality. Ошибка должна
быть диагностируема без выдачи секретов или полного payload.

Проверьте propagation context, уровень логов через конфигурацию и безопасное
поведение при недоступном exporter. Alert должен описывать влияние на
пользователя и иметь ссылку на способ диагностики/восстановления.
