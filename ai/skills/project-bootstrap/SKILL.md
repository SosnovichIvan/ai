---
name: agent-config-project-bootstrap
description: Install or update Agent Config Hub rules and skills safely in a selected local project directory.
---

# Safe project bootstrap

Read `ai/catalog.yaml`, `ai/rules/installation.md` and `ai/rules/security.md`
before preparing an installation.

Validate the target directory, derive every destination from the catalog and
produce a preview that labels each operation as create, unchanged, update or
conflict. A conflict is never overwritten by default. Use the target manifest
to determine which files are managed and whether a file was modified locally.

Create backups before replacing a managed file, write atomically, and update
the manifest only after successful completion. Report the exact results without
exposing file contents that may contain sensitive data.
