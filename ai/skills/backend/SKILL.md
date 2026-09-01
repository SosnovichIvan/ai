---
name: agent-config-backend
---

# Backend-реализация

Начинайте с контракта, модели данных и границы доверия. Определите handler,
service и client/repository; соединяйте зависимости явными конструкторами.
Валидация transport-ввода, business rules и access-control не смешиваются.

Возвращайте стабильные публичные ошибки, сохраняйте внутреннюю причину только в
безопасной диагностике. Конфигурация валидируется при старте, secrets не входят
в код/логи, а записи с несколькими шагами имеют transaction/rollback. Передайте
correlation ID, timeout и cancellation всем внешним вызовам.

Покройте service edge cases, handler contract и важные интеграции. До handoff
проверьте migration, backward compatibility, observability и graceful failure.
