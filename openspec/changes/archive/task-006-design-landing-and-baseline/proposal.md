## Why

Пользователю нужен публичный landing для знакомства с продуктом, инструкций и
безопасного выбора версии. Одновременно необходимо зафиксировать baseline
релиза и правило, по которому будущие изменения получают SemVer и проходят
regression-проверку.

## What Changes

- Создаётся адаптивный landing в Figma: продукт, возможности, инструкции,
  release downloads, changelog и screenshots приложения.
- В implementation scope фиксируется Tailwind CSS и цветовая система, созвучная
  desktop-приложению.
- Release scope фиксирует первичные desktop-платформы: macOS, Windows и Linux,
  а CI формирует их installable artifacts и checksums из одного tag release.
  Landing определяет ОС посетителя и предлагает только совместимый artifact.
- Техническое имя distribution/release identifier — `ai_rules_version`;
  публичное название продукта — `Agent Foundry`.
- Release `v1.0.0` закрепляется как первый public stable release; каждое следующее изменение
  классифицируется как PATCH/MINOR/MAJOR и проверяется вместе с legacy flows.
- Landing hero получает интерактивный preview трёх рабочих областей, а desktop
  приложение — безопасную очистку одной сущности, категории или всех локальных
  данных проекта с явным подтверждением и backup там, где это применимо.

## Scope

- Figma desktop/tablet/mobile landing screens и адаптивная спецификация.
- Release baseline, compatibility/revision policy и testing gates.
- CI release pipeline: сборка, regression gates, публикация артефактов и
  metadata, пригодных для landing release catalog.
- OpenSpec delta specs и implementation tasks.
- Interactive preview, destructive-action states и responsive layouts для них.

## Non-Goals

- Хостинг, CDN и auto-update desktop-приложения.
- Изменение названия UI desktop-приложения без отдельного approval.

## Version

- **Previous:** v0.1.0.
- **Target:** v1.0.0.
- **Level:** MAJOR.
- **Rationale:** фиксируется первый public stable distribution и добавляются
  операции необратимой очистки. Нужны explicit confirmation, ясная граница
  локальных данных и regression/migration проверка предыдущих пользовательских
  сценариев.
- **Compatibility:** данные Task, Template и TaskBlock мигрируют локально с
  резервной копией; установка не перезаписывает пользовательские инструкции
  без explicit replace; существующие сценарии Environment, rules/skills и
  задач входят в обязательный regression suite.
