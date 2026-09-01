## Разделение исходной задачи

Исходный change объединил контракты версии, миграций, лендинга и публикации.
Его реализация завершалась отдельными меньшими change:

| Направление | Завершивший change |
| --- | --- |
| Каталог конфигураций и миграция import | `task-004-import-external-configurations` |
| Лендинг и baseline | `task-006-design-landing-and-baseline` |
| Совместимость local state | `task-008-state-compatibility` |
| Контейнер лендинга | `task-009-versioned-landing-container` |
| Схема release-каталога | `task-010-release-catalog-schema` |
| Матрица SQLite schema | `task-011-schema-compatibility-matrix` |
| Подписывание и фактическая публикация | `task-012-release-signing-publication` |

Проверка checksum уже реализована. Последняя строка остаётся отдельной активной
задачей: для неё нужны закрытые signing-материалы и права публикации, которые
не должны находиться в исходном коде.
