## Why

Desktop-продукту нужен понятный README с реальными изображениями интерфейса и
коротким описанием возможностей. Одновременно repository должен выпускать
проверяемые GitHub Releases и защищать `main` и `develop` от прямых push:
изменения принимаются только через PR, одобренный владельцем.

## What Changes

- README дополняется разделом возможностей, скриншотами desktop-интерфейса и
  понятным путём установки через npm.
- Добавляются хранимые в repository изображения интерфейса и `CODEOWNERS`,
  назначающий владельца обязательным reviewer.
- Release workflow публикует immutable GitHub Release по SemVer tag только
  после desktop regression gates и сборок для macOS, Windows и Linux.
- На GitHub включается branch protection / ruleset для `main` и `develop`:
  PR обязателен, требуется один approval и code-owner approval; прямой push и
  force push запрещены.

## Scope

- README, документация, GitHub Actions, CODEOWNERS и GitHub branch rules.
- Установка приложения из repository через npm и существующая desktop release
  pipeline.

## Non-Goals

- Изменение логики desktop-приложения или его версии.
- Публикация release без созданного SemVer tag.
- Передача или хранение GitHub token в repository, документации или логах.

## Version

- **Previous:** v1.0.0.
- **Target:** v1.0.0.
- **Level:** deferred PATCH.
- **Rationale:** меняются документация и repository governance без изменения
  пользовательского runtime.
- **Compatibility:** существующие команды npm и native bundle остаются
  совместимыми; правила GitHub ограничивают только способ изменения веток.

## Capabilities

### Modified Capabilities

- `product-delivery`: repository presentation, release workflow и branch
  governance становятся воспроизводимыми и проверяемыми.
