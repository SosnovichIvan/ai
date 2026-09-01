---
name: agent-config-messaging
---

# Асинхронные взаимодействия

Опишите owner, schema/version, subject/topic, потребителей, retention и
ожидаемый порядок до публикации сообщения. Любой consumer идемпотентен:
повторная доставка, delayed delivery и partial failure не изменяют результат
непредсказуемо. Храните correlation/causation ID и не передавайте секреты.

Определите acknowledgement, retry/backoff, dead-letter/recovery и monitoring
lag/failure. Изменение schema совместимо с предыдущими consumer либо имеет
версию и rollout plan; producer не считает публикацию заменой транзакции данных
без явного outbox/compensation подхода.
