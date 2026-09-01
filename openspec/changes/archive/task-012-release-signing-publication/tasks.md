## 1. Внешние release-материалы

- [ ] 1.1 Владелец repository добавляет signing certificates и passwords в GitHub Secrets. — Проверка: secrets доступны только protected release workflow и не попадают в log.
- [ ] 1.2 Владелец подтверждает target GitHub repository и право выпуска immutable tags. — Проверка: test tag может создать draft release без public publication.

## 2. Подписанный pipeline

- [ ] 2.1 Добавить platform-specific signing и verification после сборки artifact. — Проверка: изменённый artifact не проходит signature verification.
- [ ] 2.2 Публиковать release и обновлённый каталог только после успешных verification gates. — Проверка: `v1.0.0` draft содержит macOS, Windows и Linux artifacts, checksum и signature.

## 3. Handoff

- [ ] 3.1 Обновить пользовательскую инструкцию проверки подписи и выполнить controlled release smoke. — Проверка: независимая проверка checksum/signature проходит для каждого artifact.
