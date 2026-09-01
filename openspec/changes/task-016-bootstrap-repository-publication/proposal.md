## Why

Локальная реализация Agent Foundry, её документация, workflow и OpenSpec
спецификации должны впервые попасть в GitHub repository. Публикация должна
сохранить уже включённые правила: защищённые ветки меняются только через PR,
который одобряет владелец.

## What Changes

- Служебные результаты тестов исключаются из Git index.
- Текущее проверенное состояние repository публикуется в отдельной ветке.
- Создаётся PR из этой ветки в `develop` без автоматического merge.
- Ruleset разрешает repository administrator завершать собственный PR через
  bypass только для pull request, не ослабляя запрет прямого push.

## Scope

- Git index, `.gitignore`, bootstrap branch, pull request и правила GitHub.

## Non-Goals

- Изменение логики приложения, версии `1.0.0` или создание release.
- Автоматическое слияние PR или bypass прямого push в защищённые ветки.

## Version

- **Previous:** v1.0.0.
- **Target:** v1.0.0.
- **Level:** deferred PATCH.
- **Rationale:** публикуется уже проверенное состояние без изменения runtime.
- **Compatibility:** пользовательские сценарии, локальные данные и команды npm
  не изменяются.

## Capabilities

### Modified Capabilities

- `product-delivery`: repository получает воспроизводимую стартовую публикацию
  через контролируемый PR.
