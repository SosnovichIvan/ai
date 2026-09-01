## Product Design Gate

- **Status:** APPROVED
- **Source:** Пользователь подтвердил реализацию v1.0.0 по существующему Figma-файлу
- **Updated at:** 2026-08-30
- **Recorded at:** 2026-08-30
- **Figma node (desktop):** https://www.figma.com/design/KYcd8hPuczEIpqK9dbG5D7/ai_works?node-id=55-63
- **Figma node (tablet):** https://www.figma.com/design/KYcd8hPuczEIpqK9dbG5D7/ai_works?node-id=57-82
- **Figma node (mobile):** https://www.figma.com/design/KYcd8hPuczEIpqK9dbG5D7/ai_works?node-id=58-90
- **Figma node (release catalog):** https://www.figma.com/design/KYcd8hPuczEIpqK9dbG5D7/ai_works?node-id=73-122
- **Figma node (release catalog mobile):** https://www.figma.com/design/KYcd8hPuczEIpqK9dbG5D7/ai_works?node-id=74-122
- **Figma node (instructions desktop):** https://www.figma.com/design/KYcd8hPuczEIpqK9dbG5D7/ai_works?node-id=76-122
- **Figma node (instructions mobile):** https://www.figma.com/design/KYcd8hPuczEIpqK9dbG5D7/ai_works?node-id=76-311
- **Figma node (instructions tablet):** https://www.figma.com/design/KYcd8hPuczEIpqK9dbG5D7/ai_works?node-id=79-142
- **Figma node (release catalog tablet):** https://www.figma.com/design/KYcd8hPuczEIpqK9dbG5D7/ai_works?node-id=79-182

## Scope Evidence

Утверждены Figma frames landing для desktop, tablet и mobile. В них есть
hero, возможности, инструкция, release selector, version diff, реальные
screenshots приложения и responsive states.

Также подготовлена отдельная desktop-страница release catalog: выбор системы и
версии, совместимый download CTA, integrity metadata и GitHub-style
прокручиваемая история changelog.

Подготовлены desktop и mobile страницы инструкций с последовательностью setup,
preview, profile, tasks и safety guidance. В global navigation оставлены только
«Инструкция» и «Релизы»; самостоятельный пункт «Возможности» удалён.
Публичное название и header lockup обновлены на `Agent Foundry`.

## Release pipeline

**NOT_REQUIRED** для отдельного Figma approval: CI-сборка, checksum и
публикация релизных артефактов не создают пользовательского визуального
состояния. Их результат отображается уже утверждённой страницей «Релизы».

## Изменение, ожидающее approval

Нужны новые Figma frames с node-id для:

- интерактивного preview на главной: `Установка окружения`, `Rules и skills`,
  `Задачи и шаблоны`, включая active state каждой табы;
- очистки отдельной задачи, шаблона, блока, правила или skill;
- очистки целой категории и полного очищения данных проекта, с явным
  destructive confirmation, перечнем затрагиваемых данных и состоянием успеха;
- desktop, tablet и mobile variants новых состояний.

Ранее утверждённые frames остаются evidence для неизменённых частей сайта, но
пользователь явно подтвердил реализацию перечисленных UI-сценариев на их основе.
