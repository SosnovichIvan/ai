# Security review: desktop IPC boundary

Проверка Tauri desktop runtime:

- Capability главного окна содержит только `core:default` и `dialog:default`.
  Plugins shell, fs, http, updater и произвольное выполнение команд не выданы.
- Frontend использует dialog только для выбора директории и typed `invoke`
  wrappers; он не получает прямой API чтения/записи файловой системы.
- Все mutating IPC commands принимают typed entity/catalog input, а операции
  удаления и replace требуют explicit confirmation.
- Installation и import нормализуют выбранные пути, проверяют containment и
  выполняют preview до записи.
- В Rust-коде отсутствует запуск shell/process. Ошибки возвращаются безопасными
  пользовательскими сообщениями, а содержимое секретов не логируется.

Проверено вместе с Rust unit tests, `npm run lint` и production Tauri build.
