---
name: agent-config-frontend
---

# Frontend-реализация

До кода прочитайте frontend/contracts/testing rules, active change и
утверждённый Figma node. Определите модуль и допустимое направление зависимостей;
не начинайте UI при отсутствии design gate.

Реализуйте сценарий целиком: данные, loading, empty, error, retry, disabled,
focus и responsive states. Используйте существующие UI-компоненты, токены и
public API. Компоненты не содержат скрытого I/O, страницы не получают
бизнес-логику, контрактные типы не пишутся вручную.

Перед handoff запустите lint/typecheck, targeted tests и visual smoke в
утверждённых viewport. Сообщите о каждой не проверенной границе, а не скрывайте
её за успешной сборкой.
