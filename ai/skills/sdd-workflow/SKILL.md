---
name: agent-config-sdd-workflow
description: Conduct substantial changes in the Agent Config Hub through the repository's OpenSpec SDD lifecycle.
---

# Agent Config Hub SDD workflow

Read `AGENTS.md`, relevant `openspec/specs/`, the active change and related
files in `ai/` before editing. OpenSpec is authoritative for current behavior;
the active change is authoritative for proposed behavior.

For every substantial behavior change:

1. Create a change with proposal, delta specs, technical design, tasks and
   `design-approval.md` where UI is involved.
2. Write observable requirements using SHALL/MUST and scenarios using
   WHEN/THEN. Do not encode internal implementation in specs.
3. Give every task a concrete verification method.
4. Do not implement UI before the user has explicitly approved a Figma URL with
   `node-id` recorded in the design gate.
5. Verify, sync current specs, and archive only after all tasks are complete.
