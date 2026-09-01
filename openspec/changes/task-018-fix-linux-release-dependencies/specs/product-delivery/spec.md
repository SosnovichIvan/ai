## MODIFIED Requirements

### Requirement: Reproducible multi-platform release pipeline

Система SHALL собирать каждый tag release на нативных CI runners для macOS,
Windows и Linux из lockfile и запускать regression gates до публикации. Linux
jobs MUST устанавливать системные desktop dependencies, необходимые Tauri и
его GTK/WebKit bindings.

#### Scenario: Rust-проверка на Linux runner

- **WHEN** release workflow запускает `cargo test` на Ubuntu runner
- **THEN** runner содержит `libgtk-3-dev` и проверка не останавливается из-за
  отсутствующего `gdk-3.0.pc`
