use crate::{config_store, storage};
use serde::{Deserialize, Serialize};
use std::{fs, path::{Path, PathBuf}, time::{SystemTime, UNIX_EPOCH}};

const MANIFEST_VERSION: u32 = 1;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct StateManifest {
    version: u32,
    sqlite_schema_version: i32,
    catalog_version: u32,
    updated_at: u64,
}

fn error(message: &str) -> String { message.to_owned() }
fn timestamp() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() }

pub fn path(app_data: &Path) -> PathBuf { app_data.join("state-manifest.json") }

fn read(app_data: &Path) -> Result<Option<StateManifest>, String> {
    let manifest_path = path(app_data);
    if !manifest_path.exists() { return Ok(None); }
    let bytes = fs::read(manifest_path).map_err(|_| error("Не удалось прочитать manifest локального состояния."))?;
    serde_json::from_slice(&bytes)
        .map(Some)
        .map_err(|_| error("Локальное состояние требует восстановления: manifest повреждён. Данные не изменены."))
}

fn write(app_data: &Path, sqlite_schema_version: i32) -> Result<(), String> {
    fs::create_dir_all(app_data).map_err(|_| error("Не удалось подготовить каталог локальных данных."))?;
    let manifest = StateManifest {
        version: MANIFEST_VERSION,
        sqlite_schema_version,
        catalog_version: config_store::CATALOG_VERSION,
        updated_at: timestamp(),
    };
    let destination = path(app_data);
    let temporary = app_data.join("state-manifest.json.tmp");
    fs::write(&temporary, serde_json::to_vec_pretty(&manifest).map_err(|_| error("Не удалось подготовить manifest локального состояния."))?)
        .map_err(|_| error("Не удалось сохранить manifest локального состояния."))?;
    fs::rename(temporary, destination).map_err(|_| error("Не удалось завершить сохранение manifest локального состояния."))
}

/// Проверяет состояние до открытия БД в режиме записи. Отсутствующий manifest
/// допустим для установки v1.0.0 и будет создан после успешного открытия.
pub fn validate_before_open(app_data: &Path) -> Result<(), String> {
    let database_version = storage::installed_schema_version(app_data)?;
    let catalog_version = config_store::installed_catalog_version(app_data)?;
    if database_version.is_some_and(|version| version > storage::SCHEMA_VERSION) {
        return Err(error("Локальные данные созданы более новой версией приложения. Обновите приложение или восстановите совместимую резервную копию."));
    }
    let Some(manifest) = read(app_data)? else { return Ok(()); };
    if manifest.version > MANIFEST_VERSION || manifest.sqlite_schema_version > storage::SCHEMA_VERSION || manifest.catalog_version > config_store::CATALOG_VERSION {
        return Err(error("Локальное состояние создано более новой версией приложения. Данные не изменены."));
    }
    if catalog_version.is_some_and(|version| version > config_store::CATALOG_VERSION) {
        return Err(error("Локальный каталог создан более новой версией приложения. Данные не изменены."));
    }
    if let Some(version) = database_version {
        if manifest.sqlite_schema_version != version {
            return Err(error("Локальное состояние рассогласовано. Данные не изменены; восстановите последнюю резервную копию."));
        }
    }
    Ok(())
}

/// Фиксирует только уже успешно открытое и проверенное состояние.
pub fn synchronize(app_data: &Path, sqlite_schema_version: i32) -> Result<(), String> {
    if sqlite_schema_version > storage::SCHEMA_VERSION { return Err(error("Локальные данные созданы более новой версией приложения.")); }
    write(app_data, sqlite_schema_version)
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn temporary_path() -> PathBuf {
        std::env::temp_dir().join(format!("ai-works-state-manifest-{}", Uuid::new_v4()))
    }

    #[test]
    fn creates_manifest_atomically_after_successful_open() {
        let directory = temporary_path();
        validate_before_open(&directory).unwrap();
        synchronize(&directory, storage::SCHEMA_VERSION).unwrap();
        let value: serde_json::Value = serde_json::from_slice(&fs::read(path(&directory)).unwrap()).unwrap();
        assert_eq!(value["sqliteSchemaVersion"], storage::SCHEMA_VERSION);
        assert!(!directory.join("state-manifest.json.tmp").exists());
        let _ = fs::remove_dir_all(directory);
    }

    #[test]
    fn blocks_corrupted_manifest_without_creating_database() {
        let directory = temporary_path();
        fs::create_dir_all(&directory).unwrap();
        fs::write(path(&directory), b"not-json").unwrap();
        assert!(validate_before_open(&directory).unwrap_err().contains("повреждён"));
        assert!(!storage::database_path(&directory).exists());
        let _ = fs::remove_dir_all(directory);
    }

    #[test]
    fn blocks_manifest_from_newer_application() {
        let directory = temporary_path();
        fs::create_dir_all(&directory).unwrap();
        fs::write(path(&directory), br#"{"version":1,"sqliteSchemaVersion":99,"catalogVersion":2,"updatedAt":0}"#).unwrap();
        assert!(validate_before_open(&directory).unwrap_err().contains("более новой"));
        let _ = fs::remove_dir_all(directory);
    }

    #[test]
    fn blocks_corrupted_catalog_manifest() {
        let directory = temporary_path();
        let catalog = config_store::root(&directory);
        fs::create_dir_all(&catalog).unwrap();
        fs::write(catalog.join("catalog-manifest.json"), b"not-json").unwrap();
        assert!(validate_before_open(&directory).unwrap_err().contains("каталога повреждён"));
        assert!(!storage::database_path(&directory).exists());
        let _ = fs::remove_dir_all(directory);
    }

    #[test]
    fn blocks_manifest_that_does_not_match_existing_database() {
        let directory = temporary_path();
        let connection = storage::open(&directory).unwrap();
        drop(connection);
        fs::write(path(&directory), br#"{"version":1,"sqliteSchemaVersion":1,"catalogVersion":2,"updatedAt":0}"#).unwrap();
        assert!(validate_before_open(&directory).unwrap_err().contains("рассогласовано"));
        let _ = fs::remove_dir_all(directory);
    }
}
