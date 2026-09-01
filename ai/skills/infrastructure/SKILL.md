---
name: agent-config-infrastructure
---

# Инфраструктура и поставка

Собирайте минимальный воспроизводимый artefact из lockfile и явно заданной
версии. Конфигурация окружения отделена от build, secrets передаются защищённым
механизмом. Для каждого release определите health/readiness, ресурсы, network
boundaries, observability, migration order и проверяемый rollback.

Не применяйте production изменения вручную вне согласованного pipeline. Перед
поставкой проверьте artefact в близкой среде, отсутствие secrets в layers/logs и
то, что failure exporter/dependency не делает сервис небезопасным.
