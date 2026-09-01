## MODIFIED Requirements

### Requirement: Reproducible multi-platform release pipeline

Система SHALL собирать каждый tag release на нативных CI runners для macOS,
Windows и Linux из lockfile и запускать regression gates до публикации. Linux
regression job MUST устанавливать GTK3, WebKitGTK и связанные development
dependencies, требуемые Tauri bindings.

#### Scenario: Проверка Tauri на чистом Ubuntu runner

- **WHEN** release workflow запускает `cargo test` на новом Ubuntu runner
- **THEN** `pkg-config` находит `gdk-3.0`, `libsoup-3.0` и
  `javascriptcoregtk-4.1`, а regression gate продолжает выполнение
