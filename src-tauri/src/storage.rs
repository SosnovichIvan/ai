use rusqlite::{params, Connection, OpenFlags, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

pub const SCHEMA_VERSION: i32 = 2;
/// Единственный реестр поддерживаемых направлений SQLite-миграции.
const MIGRATION_REGISTRY: &[i32] = &[1, 2];

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EntityKind {
    Task,
    Template,
    TaskBlock,
}

impl EntityKind {
    fn table(self) -> &'static str {
        match self {
            Self::Task => "tasks",
            Self::Template => "templates",
            Self::TaskBlock => "task_blocks",
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub content: String,
    pub source_template_id: Option<String>,
    pub block_snapshot: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub revision: i64,
    pub workspace_id: String,
    pub folder_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityInput {
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub content: Option<String>,
    pub source_template_id: Option<String>,
    pub block_snapshot: Option<String>,
    pub workspace_id: Option<String>,
    pub folder_id: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceInput {
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub id: String,
    pub workspace_id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderInput {
    pub id: Option<String>,
    pub workspace_id: String,
    pub parent_id: Option<String>,
    pub name: String,
}

fn error(message: &str) -> String {
    message.to_owned()
}
fn timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

pub fn database_path(app_data: &Path) -> PathBuf {
    app_data.join("ai-works.sqlite3")
}

/// Возвращает версию существующей БД без её создания.
pub fn installed_schema_version(app_data: &Path) -> Result<Option<i32>, String> {
    let path = database_path(app_data);
    if !path.exists() { return Ok(None); }
    let connection = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map_err(|_| error("Не удалось проверить локальное хранилище."))?;
    connection.query_row("PRAGMA user_version", [], |row| row.get(0))
        .map(Some)
        .map_err(|_| error("Не удалось прочитать версию локального хранилища."))
}

pub fn open(app_data: &Path) -> Result<Connection, String> {
    fs::create_dir_all(app_data)
        .map_err(|_| error("Не удалось подготовить локальное хранилище."))?;
    let path = database_path(app_data);
    let existed = path.exists();
    let connection =
        Connection::open(&path).map_err(|_| error("Не удалось открыть локальное хранилище."))?;
    let version: i32 = connection
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .map_err(|_| error("Не удалось прочитать версию локального хранилища."))?;
    if version > SCHEMA_VERSION { return Err(error("Локальные данные созданы более новой версией приложения. Данные не изменены.")); }
    if version < SCHEMA_VERSION && existed {
        let backup = app_data.join(format!("ai-works.sqlite3.backup-{}", timestamp()));
        fs::copy(&path, backup)
            .map_err(|_| error("Не удалось создать резервную копию локального хранилища."))?;
    }
    migrate(&connection, version)?;
    Ok(connection)
}

fn migrate(connection: &Connection, version: i32) -> Result<(), String> {
    if version > SCHEMA_VERSION { return Err(error("Локальные данные созданы более новой версией приложения. Данные не изменены.")); }
    if version >= SCHEMA_VERSION { return Ok(()); }
    let tx = connection
        .unchecked_transaction()
        .map_err(|_| error("Не удалось начать миграцию локального хранилища."))?;
    for migration in MIGRATION_REGISTRY.iter().copied().filter(|migration| *migration > version) {
        match migration {
            1 => for table in ["tasks", "templates", "task_blocks"] {
                tx.execute_batch(&format!(
                    "CREATE TABLE IF NOT EXISTS {table} (
              id TEXT PRIMARY KEY NOT NULL,
              name TEXT NOT NULL CHECK(length(trim(name)) > 0),
              description TEXT NULL,
              content TEXT NOT NULL DEFAULT '',
              source_template_id TEXT NULL,
              block_snapshot TEXT NULL,
              created_at INTEGER NOT NULL,
              updated_at INTEGER NOT NULL,
              revision INTEGER NOT NULL DEFAULT 1
            );
            CREATE INDEX IF NOT EXISTS idx_{table}_updated_at ON {table}(updated_at DESC);"
                ))
                .map_err(|_| error("Не удалось создать таблицы локального хранилища."))?;
            },
            2 => {
                tx.execute_batch("CREATE TABLE IF NOT EXISTS workspaces (id TEXT PRIMARY KEY NOT NULL, name TEXT NOT NULL CHECK(length(trim(name)) > 0), description TEXT NULL, created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL); CREATE TABLE IF NOT EXISTS folders (id TEXT PRIMARY KEY NOT NULL, workspace_id TEXT NOT NULL, parent_id TEXT NULL, name TEXT NOT NULL CHECK(length(trim(name)) > 0), created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL); CREATE INDEX IF NOT EXISTS idx_workspaces_updated_at ON workspaces(updated_at DESC); CREATE INDEX IF NOT EXISTS idx_folders_workspace_parent ON folders(workspace_id, parent_id, updated_at DESC);")
                    .map_err(|_| error("Не удалось создать рабочие пространства."))?;
                let now = timestamp();
                tx.execute("INSERT OR IGNORE INTO workspaces (id, name, description, created_at, updated_at) VALUES ('default', 'Моё рабочее пространство', NULL, ?1, ?1)", [now]).map_err(|_| error("Не удалось создать рабочее пространство по умолчанию."))?;
                for table in ["tasks", "templates", "task_blocks"] {
                    tx.execute_batch(&format!("ALTER TABLE {table} ADD COLUMN workspace_id TEXT NOT NULL DEFAULT 'default'; ALTER TABLE {table} ADD COLUMN folder_id TEXT NULL; CREATE INDEX IF NOT EXISTS idx_{table}_workspace_folder_updated ON {table}(workspace_id, folder_id, updated_at DESC);"))
                        .map_err(|_| error("Не удалось перенести данные в рабочее пространство."))?;
                }
            }
            _ => return Err(error("Неизвестная миграция локального хранилища.")),
        }
    }
    tx.execute_batch(&format!("PRAGMA user_version = {SCHEMA_VERSION};"))
        .map_err(|_| error("Не удалось обновить версию локального хранилища."))?;
    tx.commit()
        .map_err(|_| error("Не удалось завершить миграцию локального хранилища."))
}

fn row_to_entity(row: &rusqlite::Row<'_>) -> rusqlite::Result<Entity> {
    Ok(Entity {
        id: row.get(0)?,
        name: row.get(1)?,
        description: row.get(2)?,
        content: row.get(3)?,
        source_template_id: row.get(4)?,
        block_snapshot: row.get(5)?,
        created_at: row.get(6)?,
        updated_at: row.get(7)?,
        revision: row.get(8)?,
        workspace_id: row.get(9)?,
        folder_id: row.get(10)?,
    })
}

pub fn list(connection: &Connection, kind: EntityKind) -> Result<Vec<Entity>, String> {
    list_scoped(connection, kind, "default", None)
}

pub fn list_scoped(connection: &Connection, kind: EntityKind, workspace_id: &str, folder_id: Option<&str>) -> Result<Vec<Entity>, String> {
    ensure_workspace(connection, workspace_id)?;
    let mut statement = connection.prepare(&format!("SELECT id, name, description, content, source_template_id, block_snapshot, created_at, updated_at, revision, workspace_id, folder_id FROM {} WHERE workspace_id = ?1 AND ((?2 IS NULL AND folder_id IS NULL) OR folder_id = ?2) ORDER BY updated_at DESC", kind.table())).map_err(|_| error("Не удалось подготовить список."))?;
    let rows = statement
        .query_map(params![workspace_id, folder_id], row_to_entity)
        .map_err(|_| error("Не удалось прочитать список."))?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|_| error("Не удалось прочитать запись."))
}

pub fn save(
    connection: &Connection,
    kind: EntityKind,
    input: EntityInput,
) -> Result<Entity, String> {
    let name = input.name.trim();
    if name.is_empty() {
        return Err(error("Укажите название."));
    }
    let now = timestamp();
    let id = input.id.unwrap_or_else(|| Uuid::new_v4().to_string());
    let description = input.description.filter(|value| !value.trim().is_empty());
    let content = input.content.unwrap_or_default();
    let exists: Option<i64> = connection
        .query_row(
            &format!("SELECT revision FROM {} WHERE id = ?1", kind.table()),
            [&id],
            |row| row.get(0),
        )
        .optional()
        .map_err(|_| error("Не удалось проверить запись."))?;
    let existing_location: Option<(String, Option<String>)> = connection.query_row(&format!("SELECT workspace_id, folder_id FROM {} WHERE id = ?1", kind.table()), [&id], |row| Ok((row.get(0)?, row.get(1)?))).optional().map_err(|_| error("Не удалось проверить расположение записи."))?;
    let workspace_id = input.workspace_id.as_deref().or(existing_location.as_ref().map(|value| value.0.as_str())).unwrap_or("default");
    let folder_id = input.folder_id.as_deref().or(existing_location.as_ref().and_then(|value| value.1.as_deref()));
    validate_location(connection, workspace_id, folder_id)?;
    match exists {
        Some(_) => connection.execute(&format!("UPDATE {} SET name = ?2, description = ?3, content = ?4, source_template_id = ?5, block_snapshot = ?6, workspace_id = ?7, folder_id = ?8, updated_at = ?9, revision = revision + 1 WHERE id = ?1", kind.table()), params![id, name, description, content, input.source_template_id, input.block_snapshot, workspace_id, folder_id, now]).map_err(|_| error("Не удалось сохранить изменения."))?,
        None => connection.execute(&format!("INSERT INTO {} (id, name, description, content, source_template_id, block_snapshot, workspace_id, folder_id, created_at, updated_at, revision) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?9, 1)", kind.table()), params![id, name, description, content, input.source_template_id, input.block_snapshot, workspace_id, folder_id, now]).map_err(|_| error("Не удалось создать запись."))?,
    };
    connection.query_row(&format!("SELECT id, name, description, content, source_template_id, block_snapshot, created_at, updated_at, revision, workspace_id, folder_id FROM {} WHERE id = ?1", kind.table()), [&id], row_to_entity).map_err(|_| error("Не удалось прочитать сохранённую запись."))
}

fn ensure_workspace(connection: &Connection, id: &str) -> Result<(), String> {
    let exists: Option<String> = connection.query_row("SELECT id FROM workspaces WHERE id = ?1", [id], |row| row.get(0)).optional().map_err(|_| error("Не удалось проверить рабочее пространство."))?;
    if exists.is_none() { return Err(error("Рабочее пространство не найдено.")); }
    Ok(())
}

fn validate_location(connection: &Connection, workspace_id: &str, folder_id: Option<&str>) -> Result<(), String> {
    ensure_workspace(connection, workspace_id)?;
    if let Some(folder_id) = folder_id {
        let folder_workspace: Option<String> = connection.query_row("SELECT workspace_id FROM folders WHERE id = ?1", [folder_id], |row| row.get(0)).optional().map_err(|_| error("Не удалось проверить папку."))?;
        if folder_workspace.as_deref() != Some(workspace_id) { return Err(error("Папка не принадлежит выбранному рабочему пространству.")); }
    }
    Ok(())
}

fn row_to_workspace(row: &rusqlite::Row<'_>) -> rusqlite::Result<Workspace> {
    Ok(Workspace { id: row.get(0)?, name: row.get(1)?, description: row.get(2)?, created_at: row.get(3)?, updated_at: row.get(4)? })
}

fn row_to_folder(row: &rusqlite::Row<'_>) -> rusqlite::Result<Folder> {
    Ok(Folder { id: row.get(0)?, workspace_id: row.get(1)?, parent_id: row.get(2)?, name: row.get(3)?, created_at: row.get(4)?, updated_at: row.get(5)? })
}

pub fn list_workspaces(connection: &Connection) -> Result<Vec<Workspace>, String> {
    let mut statement = connection.prepare("SELECT id, name, description, created_at, updated_at FROM workspaces ORDER BY CASE WHEN id = 'default' THEN 0 ELSE 1 END, updated_at DESC").map_err(|_| error("Не удалось подготовить список рабочих пространств."))?;
    let result = statement.query_map([], row_to_workspace).map_err(|_| error("Не удалось прочитать рабочие пространства."))?.collect::<Result<Vec<_>, _>>().map_err(|_| error("Не удалось прочитать рабочее пространство."));
    result
}

pub fn save_workspace(connection: &Connection, input: WorkspaceInput) -> Result<Workspace, String> {
    let name = input.name.trim(); if name.is_empty() { return Err(error("Укажите название рабочего пространства.")); }
    let id = input.id.unwrap_or_else(|| Uuid::new_v4().to_string()); let now = timestamp();
    let description = input.description.filter(|value| !value.trim().is_empty());
    let exists: Option<String> = connection.query_row("SELECT id FROM workspaces WHERE id = ?1", [&id], |row| row.get(0)).optional().map_err(|_| error("Не удалось проверить рабочее пространство."))?;
    if exists.is_some() { connection.execute("UPDATE workspaces SET name = ?2, description = ?3, updated_at = ?4 WHERE id = ?1", params![id, name, description, now]).map_err(|_| error("Не удалось сохранить рабочее пространство."))?; }
    else { connection.execute("INSERT INTO workspaces (id, name, description, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?4)", params![id, name, description, now]).map_err(|_| error("Не удалось создать рабочее пространство."))?; }
    connection.query_row("SELECT id, name, description, created_at, updated_at FROM workspaces WHERE id = ?1", [&id], row_to_workspace).map_err(|_| error("Не удалось прочитать рабочее пространство."))
}

pub fn list_folders(connection: &Connection, workspace_id: &str) -> Result<Vec<Folder>, String> {
    ensure_workspace(connection, workspace_id)?;
    let mut statement = connection.prepare("SELECT id, workspace_id, parent_id, name, created_at, updated_at FROM folders WHERE workspace_id = ?1 ORDER BY name COLLATE NOCASE").map_err(|_| error("Не удалось подготовить список папок."))?;
    let result = statement.query_map([workspace_id], row_to_folder).map_err(|_| error("Не удалось прочитать папки."))?.collect::<Result<Vec<_>, _>>().map_err(|_| error("Не удалось прочитать папку."));
    result
}

pub fn save_folder(connection: &Connection, input: FolderInput) -> Result<Folder, String> {
    let name = input.name.trim(); if name.is_empty() { return Err(error("Укажите название папки.")); }
    validate_location(connection, &input.workspace_id, input.parent_id.as_deref())?;
    let id = input.id.unwrap_or_else(|| Uuid::new_v4().to_string()); let now = timestamp();
    if input.parent_id.as_deref() == Some(&id) { return Err(error("Папка не может быть родительской для самой себя.")); }
    let exists: Option<String> = connection.query_row("SELECT id FROM folders WHERE id = ?1", [&id], |row| row.get(0)).optional().map_err(|_| error("Не удалось проверить папку."))?;
    if exists.is_some() { connection.execute("UPDATE folders SET workspace_id = ?2, parent_id = ?3, name = ?4, updated_at = ?5 WHERE id = ?1", params![id, input.workspace_id, input.parent_id, name, now]).map_err(|_| error("Не удалось сохранить папку."))?; }
    else { connection.execute("INSERT INTO folders (id, workspace_id, parent_id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?5)", params![id, input.workspace_id, input.parent_id, name, now]).map_err(|_| error("Не удалось создать папку."))?; }
    connection.query_row("SELECT id, workspace_id, parent_id, name, created_at, updated_at FROM folders WHERE id = ?1", [&id], row_to_folder).map_err(|_| error("Не удалось прочитать папку."))
}

pub fn delete(connection: &Connection, kind: EntityKind, id: &str) -> Result<(), String> {
    let count = connection
        .execute(&format!("DELETE FROM {} WHERE id = ?1", kind.table()), [id])
        .map_err(|_| error("Не удалось удалить запись."))?;
    if count == 0 {
        return Err(error("Запись не найдена."));
    }
    Ok(())
}

pub fn clear(connection: &Connection, kind: EntityKind) -> Result<usize, String> {
    connection
        .execute(&format!("DELETE FROM {}", kind.table()), [])
        .map_err(|_| error("Не удалось очистить локальные записи."))
}

pub fn clear_all(connection: &Connection) -> Result<usize, String> {
    let transaction = connection.unchecked_transaction().map_err(|_| error("Не удалось начать очистку локальных данных."))?;
    let mut count = 0;
    for table in ["tasks", "templates", "task_blocks"] { count += transaction.execute(&format!("DELETE FROM {table}"), []).map_err(|_| error("Не удалось очистить локальные записи."))?; }
    transaction.commit().map_err(|_| error("Не удалось завершить очистку локальных данных."))?;
    Ok(count)
}

pub fn delete_folder(connection: &Connection, folder_id: &str) -> Result<usize, String> {
    let folder: Option<Folder> = connection.query_row("SELECT id, workspace_id, parent_id, name, created_at, updated_at FROM folders WHERE id = ?1", [folder_id], row_to_folder).optional().map_err(|_| error("Не удалось прочитать папку."))?;
    let folder = folder.ok_or_else(|| error("Папка не найдена."))?;
    let transaction = connection.unchecked_transaction().map_err(|_| error("Не удалось начать удаление папки."))?;
    let mut statement = transaction.prepare("WITH RECURSIVE subtree(id) AS (SELECT id FROM folders WHERE id = ?1 UNION ALL SELECT folders.id FROM folders JOIN subtree ON folders.parent_id = subtree.id) SELECT id FROM subtree").map_err(|_| error("Не удалось подготовить удаление папки."))?;
    let ids = statement.query_map([folder_id], |row| row.get::<_, String>(0)).map_err(|_| error("Не удалось прочитать вложенные папки."))?.collect::<Result<Vec<_>, _>>().map_err(|_| error("Не удалось прочитать вложенные папки."))?;
    drop(statement);
    let mut count = 0;
    for id in &ids { for table in ["tasks", "templates", "task_blocks"] { count += transaction.execute(&format!("DELETE FROM {table} WHERE workspace_id = ?1 AND folder_id = ?2"), params![folder.workspace_id, id]).map_err(|_| error("Не удалось удалить содержимое папки."))?; } }
    for id in ids { count += transaction.execute("DELETE FROM folders WHERE id = ?1", [id]).map_err(|_| error("Не удалось удалить папку."))?; }
    transaction.commit().map_err(|_| error("Не удалось завершить удаление папки."))?;
    Ok(count)
}

pub fn delete_workspace(connection: &Connection, workspace_id: &str) -> Result<usize, String> {
    if workspace_id == "default" { return Err(error("Рабочее пространство по умолчанию удалить нельзя.")); }
    ensure_workspace(connection, workspace_id)?;
    let transaction = connection.unchecked_transaction().map_err(|_| error("Не удалось начать удаление рабочего пространства."))?;
    let mut count = 0;
    for table in ["tasks", "templates", "task_blocks"] { count += transaction.execute(&format!("DELETE FROM {table} WHERE workspace_id = ?1"), [workspace_id]).map_err(|_| error("Не удалось удалить содержимое рабочего пространства."))?; }
    transaction.execute("DELETE FROM folders WHERE workspace_id = ?1", [workspace_id]).map_err(|_| error("Не удалось удалить папки рабочего пространства."))?;
    transaction.execute("DELETE FROM workspaces WHERE id = ?1", [workspace_id]).map_err(|_| error("Не удалось удалить рабочее пространство."))?;
    transaction.commit().map_err(|_| error("Не удалось завершить удаление рабочего пространства."))?;
    Ok(count)
}

pub fn copy_entity(connection: &Connection, kind: EntityKind, id: &str, workspace_id: &str, folder_id: Option<&str>) -> Result<Entity, String> {
    let source = connection.query_row(&format!("SELECT id, name, description, content, source_template_id, block_snapshot, created_at, updated_at, revision, workspace_id, folder_id FROM {} WHERE id = ?1", kind.table()), [id], row_to_entity).optional().map_err(|_| error("Не удалось прочитать исходную запись."))?.ok_or_else(|| error("Исходная запись не найдена."))?;
    validate_location(connection, workspace_id, folder_id)?;
    save(connection, kind, EntityInput { id: None, name: source.name, description: source.description, content: Some(source.content), source_template_id: source.source_template_id, block_snapshot: source.block_snapshot, workspace_id: Some(workspace_id.to_owned()), folder_id: folder_id.map(str::to_owned) })
}

pub fn copy_folder(connection: &Connection, source_folder_id: &str, workspace_id: &str, parent_id: Option<&str>) -> Result<Folder, String> {
    validate_location(connection, workspace_id, parent_id)?;
    let source: Folder = connection.query_row("SELECT id, workspace_id, parent_id, name, created_at, updated_at FROM folders WHERE id = ?1", [source_folder_id], row_to_folder).optional().map_err(|_| error("Не удалось прочитать исходную папку."))?.ok_or_else(|| error("Исходная папка не найдена."))?;
    let transaction = connection.unchecked_transaction().map_err(|_| error("Не удалось начать копирование папки."))?;
    let mut mapping = std::collections::BTreeMap::new();
    let mut queue = vec![(source.id.clone(), parent_id.map(str::to_owned))];
    let mut root_copy = None;
    while let Some((old_id, new_parent_id)) = queue.pop() {
        let folder: Folder = transaction.query_row("SELECT id, workspace_id, parent_id, name, created_at, updated_at FROM folders WHERE id = ?1", [&old_id], row_to_folder).map_err(|_| error("Не удалось прочитать папку для копирования."))?;
        let new_id = Uuid::new_v4().to_string(); let now = timestamp();
        transaction.execute("INSERT INTO folders (id, workspace_id, parent_id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?5)", params![new_id, workspace_id, new_parent_id, folder.name, now]).map_err(|_| error("Не удалось создать копию папки."))?;
        if root_copy.is_none() { root_copy = Some(new_id.clone()); }
        mapping.insert(old_id.clone(), new_id.clone());
        let mut statement = transaction.prepare("SELECT id FROM folders WHERE parent_id = ?1 ORDER BY created_at ASC").map_err(|_| error("Не удалось прочитать вложенные папки."))?;
        let children = statement.query_map([&old_id], |row| row.get::<_, String>(0)).map_err(|_| error("Не удалось прочитать вложенные папки."))?.collect::<Result<Vec<_>, _>>().map_err(|_| error("Не удалось прочитать вложенные папки."))?;
        drop(statement);
        for child in children.into_iter().rev() { queue.push((child, Some(new_id.clone()))); }
        for table in ["tasks", "templates", "task_blocks"] {
            let mut entities = transaction.prepare(&format!("SELECT id, name, description, content, source_template_id, block_snapshot FROM {table} WHERE folder_id = ?1")) .map_err(|_| error("Не удалось прочитать содержимое папки."))?;
            let rows = entities.query_map([&old_id], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, Option<String>>(2)?, row.get::<_, String>(3)?, row.get::<_, Option<String>>(4)?, row.get::<_, Option<String>>(5)?))).map_err(|_| error("Не удалось прочитать содержимое папки."))?.collect::<Result<Vec<_>, _>>().map_err(|_| error("Не удалось прочитать содержимое папки."))?;
            drop(entities);
            for (_, name, description, content, source_template_id, block_snapshot) in rows { transaction.execute(&format!("INSERT INTO {table} (id, name, description, content, source_template_id, block_snapshot, workspace_id, folder_id, created_at, updated_at, revision) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?9, 1)"), params![Uuid::new_v4().to_string(), name, description, content, source_template_id, block_snapshot, workspace_id, new_id, now]).map_err(|_| error("Не удалось скопировать содержимое папки."))?; }
        }
    }
    let copied_id = root_copy.ok_or_else(|| error("Не удалось создать копию папки."))?;
    let copied = transaction.query_row("SELECT id, workspace_id, parent_id, name, created_at, updated_at FROM folders WHERE id = ?1", [&copied_id], row_to_folder).map_err(|_| error("Не удалось прочитать копию папки."))?;
    transaction.commit().map_err(|_| error("Не удалось завершить копирование папки."))?;
    Ok(copied)
}

pub fn save_task_as_template(
    connection: &Connection,
    task_id: &str,
    name: String,
    description: Option<String>,
) -> Result<Entity, String> {
    let task = connection.query_row("SELECT id, name, description, content, source_template_id, block_snapshot, created_at, updated_at, revision, workspace_id, folder_id FROM tasks WHERE id = ?1", [task_id], row_to_entity).optional().map_err(|_| error("Не удалось прочитать задачу."))?.ok_or_else(|| error("Задача не найдена."))?;
    save(
        connection,
        EntityKind::Template,
        EntityInput {
            id: None,
            name,
            description,
            content: Some(task.content),
            source_template_id: Some(task.id),
            block_snapshot: task.block_snapshot,
            workspace_id: Some(task.workspace_id),
            folder_id: task.folder_id,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temporary_path() -> PathBuf {
        let directory = std::env::temp_dir().join(format!("ai-works-storage-{}", Uuid::new_v4()));
        let _ = fs::remove_dir_all(&directory);
        directory
    }

    #[test]
    fn migration_and_crud_preserve_required_and_optional_fields() {
        let directory = temporary_path();
        let connection = open(&directory).unwrap();
        assert!(save(
            &connection,
            EntityKind::Task,
            EntityInput {
                id: None,
                name: "  ".into(),
                description: None,
                content: None,
                source_template_id: None,
                block_snapshot: None,
                workspace_id: None,
                folder_id: None,
            }
        )
        .is_err());
        let saved = save(
            &connection,
            EntityKind::Task,
            EntityInput {
                id: None,
                name: "Проверить установку".into(),
                description: None,
                content: Some("Текст".into()),
                source_template_id: None,
                block_snapshot: None,
                workspace_id: None,
                folder_id: None,
            },
        )
        .unwrap();
        assert_eq!(list(&connection, EntityKind::Task).unwrap().len(), 1);
        assert_eq!(saved.description, None);
        delete(&connection, EntityKind::Task, &saved.id).unwrap();
        assert!(list(&connection, EntityKind::Task).unwrap().is_empty());
        let _ = fs::remove_dir_all(directory);
    }

    #[test]
    fn template_is_an_independent_snapshot() {
        let directory = temporary_path();
        let connection = open(&directory).unwrap();
        let task = save(
            &connection,
            EntityKind::Task,
            EntityInput {
                id: None,
                name: "Задача".into(),
                description: None,
                content: Some("Первая версия".into()),
                source_template_id: None,
                block_snapshot: Some("[]".into()),
                workspace_id: None,
                folder_id: None,
            },
        )
        .unwrap();
        let template = save_task_as_template(&connection, &task.id, "Шаблон".into(), None).unwrap();
        save(
            &connection,
            EntityKind::Task,
            EntityInput {
                id: Some(task.id),
                name: "Задача".into(),
                description: None,
                content: Some("Новая версия".into()),
                source_template_id: None,
                block_snapshot: Some("[]".into()),
                workspace_id: None,
                folder_id: None,
            },
        )
        .unwrap();
        assert_eq!(
            list(&connection, EntityKind::Template).unwrap()[0].content,
            template.content
        );
        let _ = fs::remove_dir_all(directory);
    }

    #[test]
    fn category_and_full_clear_do_not_leave_entities() {
        let directory = temporary_path(); let connection = open(&directory).unwrap();
        for kind in [EntityKind::Task, EntityKind::Template, EntityKind::TaskBlock] { save(&connection, kind, EntityInput { id: None, name: "Запись".into(), description: None, content: None, source_template_id: None, block_snapshot: None, workspace_id: None, folder_id: None }).unwrap(); }
        assert_eq!(clear(&connection, EntityKind::Task).unwrap(), 1);
        assert!(list(&connection, EntityKind::Task).unwrap().is_empty());
        assert_eq!(clear_all(&connection).unwrap(), 2);
        assert!(list(&connection, EntityKind::Template).unwrap().is_empty());
        assert!(list(&connection, EntityKind::TaskBlock).unwrap().is_empty());
        let _ = fs::remove_dir_all(directory);
    }

    #[test]
    fn migration_places_existing_records_in_default_workspace() {
        let directory = temporary_path(); fs::create_dir_all(&directory).unwrap();
        let path = database_path(&directory); let connection = Connection::open(&path).unwrap();
        connection.execute_batch("CREATE TABLE tasks (id TEXT PRIMARY KEY NOT NULL, name TEXT NOT NULL, description TEXT NULL, content TEXT NOT NULL DEFAULT '', source_template_id TEXT NULL, block_snapshot TEXT NULL, created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL, revision INTEGER NOT NULL DEFAULT 1); CREATE TABLE templates (id TEXT PRIMARY KEY NOT NULL, name TEXT NOT NULL, description TEXT NULL, content TEXT NOT NULL DEFAULT '', source_template_id TEXT NULL, block_snapshot TEXT NULL, created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL, revision INTEGER NOT NULL DEFAULT 1); CREATE TABLE task_blocks (id TEXT PRIMARY KEY NOT NULL, name TEXT NOT NULL, description TEXT NULL, content TEXT NOT NULL DEFAULT '', source_template_id TEXT NULL, block_snapshot TEXT NULL, created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL, revision INTEGER NOT NULL DEFAULT 1); INSERT INTO tasks VALUES ('legacy', 'Старая задача', 'Описание', 'Содержимое', 'template-id', '[\"block-id\"]', 10, 20, 3); PRAGMA user_version = 1;").unwrap(); drop(connection);
        let migrated = open(&directory).unwrap();
        assert_eq!(list_workspaces(&migrated).unwrap()[0].id, "default");
        let task = &list(&migrated, EntityKind::Task).unwrap()[0];
        assert_eq!(task.id, "legacy");
        assert_eq!(task.workspace_id, "default");
        assert_eq!(task.description.as_deref(), Some("Описание"));
        assert_eq!(task.content, "Содержимое");
        assert_eq!(task.source_template_id.as_deref(), Some("template-id"));
        assert_eq!(task.block_snapshot.as_deref(), Some("[\"block-id\"]"));
        assert_eq!((task.created_at, task.updated_at, task.revision), (10, 20, 3));
        assert!(directory.read_dir().unwrap().any(|entry| entry.unwrap().file_name().to_string_lossy().starts_with("ai-works.sqlite3.backup-")));
        let _ = fs::remove_dir_all(directory);
    }

    #[test]
    fn failed_migration_rolls_back_and_keeps_backup() {
        let directory = temporary_path();
        fs::create_dir_all(&directory).unwrap();
        let path = database_path(&directory);
        let connection = Connection::open(&path).unwrap();
        connection.execute_batch("PRAGMA user_version = 1;").unwrap();
        drop(connection);

        assert!(open(&directory).is_err());
        assert_eq!(installed_schema_version(&directory).unwrap(), Some(1));
        let checked = Connection::open(&path).unwrap();
        assert_eq!(checked.query_row("SELECT count(*) FROM sqlite_master WHERE type = 'table' AND name = 'workspaces'", [], |row| row.get::<_, i32>(0)).unwrap(), 0);
        assert!(directory.read_dir().unwrap().any(|entry| entry.unwrap().file_name().to_string_lossy().starts_with("ai-works.sqlite3.backup-")));
        let _ = fs::remove_dir_all(directory);
    }

    #[test]
    fn workspaces_and_folders_isolate_and_copy_entities() {
        let directory = temporary_path(); let connection = open(&directory).unwrap();
        let target = save_workspace(&connection, WorkspaceInput { id: None, name: "Клиентский проект".into(), description: None }).unwrap();
        let folder = save_folder(&connection, FolderInput { id: None, workspace_id: target.id.clone(), parent_id: None, name: "Релиз".into() }).unwrap();
        let source = save(&connection, EntityKind::Task, EntityInput { id: None, name: "План".into(), description: None, content: Some("Текст".into()), source_template_id: None, block_snapshot: None, workspace_id: None, folder_id: None }).unwrap();
        let copied = copy_entity(&connection, EntityKind::Task, &source.id, &target.id, Some(&folder.id)).unwrap();
        assert_ne!(source.id, copied.id);
        assert!(list_scoped(&connection, EntityKind::Task, "default", None).unwrap().iter().any(|item| item.id == source.id));
        assert!(list_scoped(&connection, EntityKind::Task, &target.id, Some(&folder.id)).unwrap().iter().any(|item| item.id == copied.id));
        assert!(list_scoped(&connection, EntityKind::Task, &target.id, None).unwrap().is_empty());
        assert!(delete_workspace(&connection, "default").is_err());
        assert_eq!(delete_workspace(&connection, &target.id).unwrap(), 1);
        assert_eq!(list_workspaces(&connection).unwrap().len(), 1);
        let _ = fs::remove_dir_all(directory);
    }

    #[test]
    fn copying_a_folder_copies_nested_content_independently() {
        let directory = temporary_path(); let connection = open(&directory).unwrap();
        let target = save_workspace(&connection, WorkspaceInput { id: None, name: "Целевое".into(), description: None }).unwrap();
        let folder = save_folder(&connection, FolderInput { id: None, workspace_id: "default".into(), parent_id: None, name: "Исходная".into() }).unwrap();
        let nested = save_folder(&connection, FolderInput { id: None, workspace_id: "default".into(), parent_id: Some(folder.id.clone()), name: "Вложенная".into() }).unwrap();
        save(&connection, EntityKind::Template, EntityInput { id: None, name: "Шаблон".into(), description: None, content: Some("Текст".into()), source_template_id: None, block_snapshot: None, workspace_id: Some("default".into()), folder_id: Some(nested.id.clone()) }).unwrap();
        let copied = copy_folder(&connection, &folder.id, &target.id, None).unwrap();
        let copied_children = list_folders(&connection, &target.id).unwrap();
        let copied_nested = copied_children.iter().find(|item| item.parent_id.as_deref() == Some(&copied.id)).unwrap();
        assert_eq!(list_scoped(&connection, EntityKind::Template, &target.id, Some(&copied_nested.id)).unwrap().len(), 1);
        let _ = fs::remove_dir_all(directory);
    }

    #[test]
    fn deleting_a_folder_removes_only_its_subtree() {
        let directory = temporary_path(); let connection = open(&directory).unwrap();
        let folder = save_folder(&connection, FolderInput { id: None, workspace_id: "default".into(), parent_id: None, name: "Удалить".into() }).unwrap();
        let nested = save_folder(&connection, FolderInput { id: None, workspace_id: "default".into(), parent_id: Some(folder.id.clone()), name: "Вложенная".into() }).unwrap();
        let keep = save_folder(&connection, FolderInput { id: None, workspace_id: "default".into(), parent_id: None, name: "Оставить".into() }).unwrap();
        save(&connection, EntityKind::Task, EntityInput { id: None, name: "Удалить".into(), description: None, content: None, source_template_id: None, block_snapshot: None, workspace_id: Some("default".into()), folder_id: Some(nested.id) }).unwrap();
        save(&connection, EntityKind::Task, EntityInput { id: None, name: "Оставить".into(), description: None, content: None, source_template_id: None, block_snapshot: None, workspace_id: Some("default".into()), folder_id: Some(keep.id.clone()) }).unwrap();
        assert!(delete_folder(&connection, &folder.id).unwrap() >= 1);
        assert_eq!(list_folders(&connection, "default").unwrap().len(), 1);
        assert_eq!(list_scoped(&connection, EntityKind::Task, "default", Some(&keep.id)).unwrap().len(), 1);
        let _ = fs::remove_dir_all(directory);
    }
}
