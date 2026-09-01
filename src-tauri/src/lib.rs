use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::{Component, Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::{Emitter, Manager};
use std::sync::Mutex;

mod config_store;
mod state_manifest;
mod storage;

const MANIFEST_VERSION: u32 = 1;

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct PreviewOperation {
    path: String,
    kind: String,
    reason: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct InstallationPreview {
    target_path: String,
    profile: String,
    operations: Vec<PreviewOperation>,
    can_apply: bool,
}

#[derive(Debug, Deserialize)]
struct Catalog {
    profiles: BTreeMap<String, Profile>,
    targets: Targets,
}
#[derive(Debug, Deserialize)]
struct Profile {
    rules: Vec<String>,
    skills: Vec<String>,
}
#[derive(Debug, Deserialize)]
struct Targets {
    rules: String,
    skills: String,
    manifest: String,
    backups: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Manifest {
    version: u32,
    profile: String,
    installed_at: u64,
    files: BTreeMap<String, String>,
}

#[derive(Debug, Clone)]
struct PlannedFile {
    relative: String,
    content: Vec<u8>,
    category: &'static str,
}

fn safe_error(message: &str) -> String {
    message.to_owned()
}

fn catalog() -> Result<Catalog, String> {
    let parsed: Catalog = serde_yaml::from_str(include_str!("../../ai/catalog.yaml"))
        .map_err(|_| safe_error("Не удалось прочитать каталог профиля."))?;
    validate_relative_path(&parsed.targets.rules)?;
    validate_relative_path(&parsed.targets.skills)?;
    validate_relative_path(&parsed.targets.manifest)?;
    validate_relative_path(&parsed.targets.backups)?;
    Ok(parsed)
}

fn validate_relative_path(value: &str) -> Result<(), String> {
    let path = Path::new(value);
    if value.is_empty()
        || path.is_absolute()
        || path.components().any(|part| {
            matches!(
                part,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(safe_error("Каталог профиля содержит недопустимый путь."));
    }
    Ok(())
}

fn normalize_target(target: &str) -> Result<PathBuf, String> {
    let path = Path::new(target);
    if !path.is_dir() {
        return Err(safe_error("Выбранный путь не является доступной папкой."));
    }
    fs::canonicalize(path).map_err(|_| safe_error("Не удалось нормализовать путь к папке."))
}

fn checked_destination(target: &Path, relative: &str) -> Result<PathBuf, String> {
    validate_relative_path(relative)?;
    let destination = target.join(relative);
    if !destination.starts_with(target) {
        return Err(safe_error(
            "Вычисленный путь выходит за пределы выбранной папки.",
        ));
    }
    let mut current = target.to_path_buf();
    for component in Path::new(relative).components() {
        current.push(component);
        if current.exists()
            && fs::symlink_metadata(&current)
                .map_err(|_| safe_error("Не удалось проверить путь установки."))?
                .file_type()
                .is_symlink()
        {
            return Err(safe_error(
                "Символические ссылки в пути установки не поддерживаются.",
            ));
        }
    }
    Ok(destination)
}

fn source_content(source: &str) -> Result<&'static str, String> {
    match source {
        "rules/core.md" => Ok(include_str!("../../ai/rules/core.md")),
        "rules/installation.md" => Ok(include_str!("../../ai/rules/installation.md")),
        "rules/security.md" => Ok(include_str!("../../ai/rules/security.md")),
        "rules/lifecycle.md" => Ok(include_str!("../../ai/rules/lifecycle.md")),
        "rules/frontend.md" => Ok(include_str!("../../ai/rules/frontend.md")),
        "rules/backend.md" => Ok(include_str!("../../ai/rules/backend.md")),
        "rules/database.md" => Ok(include_str!("../../ai/rules/database.md")),
        "rules/contracts.md" => Ok(include_str!("../../ai/rules/contracts.md")),
        "rules/devops.md" => Ok(include_str!("../../ai/rules/devops.md")),
        "rules/testing.md" => Ok(include_str!("../../ai/rules/testing.md")),
        "rules/observability.md" => Ok(include_str!("../../ai/rules/observability.md")),
        "rules/git.md" => Ok(include_str!("../../ai/rules/git.md")),
        "rules/documentation.md" => Ok(include_str!("../../ai/rules/documentation.md")),
        "rules/review.md" => Ok(include_str!("../../ai/rules/review.md")),
        "skills/sdd-workflow/SKILL.md" => Ok(include_str!("../../ai/skills/sdd-workflow/SKILL.md")),
        "skills/project-bootstrap/SKILL.md" => {
            Ok(include_str!("../../ai/skills/project-bootstrap/SKILL.md"))
        }
        "skills/analyst/SKILL.md" => Ok(include_str!("../../ai/skills/analyst/SKILL.md")),
        "skills/frontend/SKILL.md" => Ok(include_str!("../../ai/skills/frontend/SKILL.md")),
        "skills/backend/SKILL.md" => Ok(include_str!("../../ai/skills/backend/SKILL.md")),
        "skills/designer/SKILL.md" => Ok(include_str!("../../ai/skills/designer/SKILL.md")),
        "skills/reviewer/SKILL.md" => Ok(include_str!("../../ai/skills/reviewer/SKILL.md")),
        "skills/tester/SKILL.md" => Ok(include_str!("../../ai/skills/tester/SKILL.md")),
        "skills/quality/SKILL.md" => Ok(include_str!("../../ai/skills/quality/SKILL.md")),
        "skills/database/SKILL.md" => Ok(include_str!("../../ai/skills/database/SKILL.md")),
        "skills/infrastructure/SKILL.md" => Ok(include_str!("../../ai/skills/infrastructure/SKILL.md")),
        "skills/observability/SKILL.md" => Ok(include_str!("../../ai/skills/observability/SKILL.md")),
        "skills/messaging/SKILL.md" => Ok(include_str!("../../ai/skills/messaging/SKILL.md")),
        "skills/orchestrator/SKILL.md" => Ok(include_str!("../../ai/skills/orchestrator/SKILL.md")),
        _ => Err(safe_error(
            "Каталог ссылается на неподдерживаемый исходный файл.",
        )),
    }
}

fn plan_files(catalog: &Catalog, profile_name: &str) -> Result<Vec<PlannedFile>, String> {
    let profile = catalog
        .profiles
        .get(profile_name)
        .ok_or_else(|| safe_error("Неизвестный профиль."))?;
    let mut files = Vec::new();
    if !profile.rules.is_empty() {
        let mut merged = String::from("# Agent Foundry — управляемые правила\n\n");
        for source in &profile.rules {
            if !source.starts_with("rules/") {
                return Err(safe_error("Правило находится вне разрешённого каталога."));
            }
            merged.push_str("<!-- source: ");
            merged.push_str(source);
            merged.push_str(" -->\n\n");
            merged.push_str(source_content(source)?);
            merged.push_str("\n\n");
        }
        files.push(PlannedFile {
            relative: catalog.targets.rules.clone(),
            content: merged.into_bytes(),
            category: "rules",
        });
    }
    for source in &profile.skills {
        let parts: Vec<_> = Path::new(source).components().collect();
        if parts.len() != 3
            || source.starts_with('/')
            || !source.starts_with("skills/")
            || Path::new(source).file_name() != Some(std::ffi::OsStr::new("SKILL.md"))
        {
            return Err(safe_error(
                "Skill находится вне разрешённого формата каталога.",
            ));
        }
        let skill_name = Path::new(source)
            .parent()
            .and_then(Path::file_name)
            .and_then(std::ffi::OsStr::to_str)
            .ok_or_else(|| safe_error("Недопустимое имя skill."))?;
        let relative = format!("{}/{}/SKILL.md", catalog.targets.skills, skill_name);
        files.push(PlannedFile {
            relative,
            content: source_content(source)?.as_bytes().to_vec(),
            category: "skills",
        });
    }
    let unique: BTreeSet<_> = files.iter().map(|file| file.relative.as_str()).collect();
    if unique.len() != files.len() {
        return Err(safe_error("Профиль содержит коллизию целевых файлов."));
    }
    Ok(files)
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn read_manifest(target: &Path, catalog: &Catalog) -> Result<Option<Manifest>, String> {
    let path = checked_destination(target, &catalog.targets.manifest)?;
    if !path.exists() {
        return Ok(None);
    }
    let data =
        fs::read(path).map_err(|_| safe_error("Не удалось прочитать manifest установки."))?;
    let manifest: Manifest = serde_json::from_slice(&data)
        .map_err(|_| safe_error("Manifest установки повреждён; запись заблокирована."))?;
    if manifest.version != MANIFEST_VERSION {
        return Err(safe_error(
            "Версия manifest не поддерживается; запись заблокирована.",
        ));
    }
    Ok(Some(manifest))
}

fn build_preview(
    target: &Path,
    profile_name: &str,
) -> Result<(Catalog, Vec<PlannedFile>, InstallationPreview), String> {
    let catalog = catalog()?;
    let files = plan_files(&catalog, profile_name)?;
    let manifest = read_manifest(target, &catalog)?;
    let mut operations = Vec::with_capacity(files.len());
    for file in &files {
        let destination = checked_destination(target, &file.relative)?;
        let desired_hash = sha256(&file.content);
        let operation = if !destination.exists() {
            PreviewOperation {
                path: file.relative.clone(),
                kind: "create".into(),
                reason: "Файл будет создан".into(),
            }
        } else {
            let actual = fs::read(&destination)
                .map_err(|_| safe_error("Не удалось проверить управляемый файл."))?;
            let actual_hash = sha256(&actual);
            match manifest
                .as_ref()
                .and_then(|value| value.files.get(&file.relative))
            {
                Some(previous) if previous == &actual_hash && actual_hash == desired_hash => {
                    PreviewOperation {
                        path: file.relative.clone(),
                        kind: "unchanged".into(),
                        reason: "Файл уже соответствует профилю".into(),
                    }
                }
                Some(previous) if previous == &actual_hash => PreviewOperation {
                    path: file.relative.clone(),
                    kind: "update".into(),
                    reason: "Управляемый файл будет безопасно обновлён".into(),
                },
                _ => PreviewOperation {
                    path: file.relative.clone(),
                    kind: "conflict".into(),
                    reason: "Существующий файл не будет перезаписан без явного разрешения".into(),
                },
            }
        };
        operations.push(operation);
    }
    let can_apply = operations
        .iter()
        .all(|operation| operation.kind != "conflict");
    let preview = InstallationPreview {
        target_path: target.display().to_string(),
        profile: profile_name.into(),
        operations,
        can_apply,
    };
    Ok((catalog, files, preview))
}

fn now_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn emit_progress(window: &tauri::Window, stage: &str, percent: u8) -> Result<(), String> {
    window
        .emit(
            "installation-progress",
            serde_json::json!({"stage":stage, "percent":percent}),
        )
        .map_err(|_| safe_error("Не удалось обновить прогресс."))
}

fn atomic_write(destination: &Path, content: &[u8]) -> Result<(), String> {
    let parent = destination
        .parent()
        .ok_or_else(|| safe_error("Недопустимый путь назначения."))?;
    fs::create_dir_all(parent)
        .map_err(|_| safe_error("Не удалось подготовить каталог назначения."))?;
    let unique = format!(".ai-works-{}.tmp", now_timestamp());
    let temporary = parent.join(unique);
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|_| safe_error("Не удалось создать временный файл."))?;
    file.write_all(content)
        .and_then(|_| file.sync_all())
        .map_err(|_| safe_error("Не удалось записать временный файл."))?;
    fs::rename(&temporary, destination).map_err(|_| {
        let _ = fs::remove_file(&temporary);
        safe_error("Не удалось атомарно заменить файл.")
    })
}

fn apply_plan(
    target: &Path,
    catalog: &Catalog,
    profile: &str,
    files: &[PlannedFile],
    preview: &InstallationPreview,
    fail_after: Option<usize>,
    progress: &mut dyn FnMut(&str) -> Result<(), String>,
) -> Result<(), String> {
    if !preview.can_apply {
        return Err(safe_error(
            "Есть неразрешённые конфликты. Запись не выполнялась.",
        ));
    }
    let backup_root = checked_destination(
        target,
        &format!("{}/{}", catalog.targets.backups, now_timestamp()),
    )?;
    let mut restored: Vec<(PathBuf, Option<Vec<u8>>)> = Vec::new();
    let result = (|| -> Result<(), String> {
        for (index, file) in files.iter().enumerate() {
            progress(file.category)?;
            let destination = checked_destination(target, &file.relative)?;
            let old = if destination.exists() {
                Some(
                    fs::read(&destination)
                        .map_err(|_| safe_error("Не удалось создать резервную копию."))?,
                )
            } else {
                None
            };
            if let Some(ref bytes) = old {
                let backup = backup_root.join(&file.relative);
                atomic_write(&backup, bytes)?;
            }
            if fail_after == Some(index) {
                return Err(safe_error("Тестовая ошибка записи."));
            }
            atomic_write(&destination, &file.content)?;
            restored.push((destination, old));
        }
        let mut hashes = BTreeMap::new();
        for file in files {
            hashes.insert(file.relative.clone(), sha256(&file.content));
        }
        let manifest = Manifest {
            version: MANIFEST_VERSION,
            profile: profile.into(),
            installed_at: now_timestamp(),
            files: hashes,
        };
        let manifest_path = checked_destination(target, &catalog.targets.manifest)?;
        progress("manifest")?;
        atomic_write(
            &manifest_path,
            &serde_json::to_vec_pretty(&manifest)
                .map_err(|_| safe_error("Не удалось подготовить manifest."))?,
        )?;
        Ok(())
    })();
    if result.is_err() {
        for (destination, old) in restored.into_iter().rev() {
            match old {
                Some(bytes) => {
                    let _ = atomic_write(&destination, &bytes);
                }
                None => {
                    let _ = fs::remove_file(&destination);
                }
            }
        }
    }
    result
}

#[tauri::command]
fn preview_profile(target_path: String, profile: String) -> Result<InstallationPreview, String> {
    let target = normalize_target(&target_path)?;
    let (_, _, preview) = build_preview(&target, &profile)?;
    Ok(preview)
}

#[tauri::command]
fn apply_profile(
    window: tauri::Window,
    target_path: String,
    profile: String,
) -> Result<(), String> {
    let target = normalize_target(&target_path)?;
    let (catalog, files, preview) = build_preview(&target, &profile)?;
    emit_progress(&window, "backup", 5)?;
    let mut emit_stage = |stage: &str| match stage {
        "rules" => emit_progress(&window, "rules", 35),
        "skills" => emit_progress(&window, "skills", 70),
        "manifest" => emit_progress(&window, "manifest", 100),
        _ => Ok(()),
    };
    apply_plan(
        &target,
        &catalog,
        &profile,
        &files,
        &preview,
        None,
        &mut emit_stage,
    )
}

fn task_connection(app: &tauri::AppHandle) -> Result<rusqlite::Connection, String> {
    let data_directory = app
        .path()
        .app_data_dir()
        .map_err(|_| safe_error("Не удалось определить каталог данных приложения."))?;
    state_manifest::validate_before_open(&data_directory)?;
    let connection = storage::open(&data_directory)?;
    state_manifest::synchronize(&data_directory, storage::SCHEMA_VERSION)?;
    Ok(connection)
}

fn app_data_directory(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let directory = app.path().app_data_dir().map_err(|_| safe_error("Не удалось определить каталог данных приложения."))?;
    state_manifest::validate_before_open(&directory)?;
    Ok(directory)
}

#[tauri::command]
fn list_catalog_entries(app: tauri::AppHandle, kind: String) -> Result<Vec<config_store::CatalogEntry>, String> {
    config_store::list(&app_data_directory(&app)?, &kind)
}

#[tauri::command]
fn list_workspace_catalog_entries(app: tauri::AppHandle, workspace_id: String, kind: String) -> Result<Vec<config_store::CatalogEntry>, String> {
    storage::list_workspaces(&task_connection(&app)?)?.iter().find(|workspace| workspace.id == workspace_id).ok_or_else(|| safe_error("Рабочее пространство не найдено."))?;
    config_store::list_in_workspace(&app_data_directory(&app)?, &workspace_id, &kind)
}

#[tauri::command]
fn save_catalog_entry(app: tauri::AppHandle, kind: String, path: String, content: String) -> Result<(), String> {
    config_store::save(&app_data_directory(&app)?, &kind, &path, &content)
}

#[tauri::command]
fn save_workspace_catalog_entry(app: tauri::AppHandle, workspace_id: String, kind: String, path: String, content: String) -> Result<(), String> {
    storage::list_workspaces(&task_connection(&app)?)?.iter().find(|workspace| workspace.id == workspace_id).ok_or_else(|| safe_error("Рабочее пространство не найдено."))?;
    config_store::save_in_workspace(&app_data_directory(&app)?, &workspace_id, &kind, &path, &content)
}

#[tauri::command]
fn clear_catalog_entries(app: tauri::AppHandle, kind: String, confirmed: bool) -> Result<usize, String> {
    if !confirmed { return Err(safe_error("Подтвердите очистку категории.")); }
    config_store::clear_kind(&app_data_directory(&app)?, &kind)
}

#[tauri::command]
fn clear_workspace_catalog_entries(app: tauri::AppHandle, workspace_id: String, kind: String, confirmed: bool) -> Result<usize, String> {
    if !confirmed { return Err(safe_error("Подтвердите очистку категории.")); }
    storage::list_workspaces(&task_connection(&app)?)?.iter().find(|workspace| workspace.id == workspace_id).ok_or_else(|| safe_error("Рабочее пространство не найдено."))?;
    config_store::clear_kind_in_workspace(&app_data_directory(&app)?, &workspace_id, &kind)
}

#[tauri::command]
fn delete_catalog_entry(app: tauri::AppHandle, kind: String, path: String, confirmed: bool) -> Result<(), String> {
    if !confirmed { return Err(safe_error("Подтвердите удаление элемента.")); }
    config_store::delete(&app_data_directory(&app)?, &kind, &path)
}

#[tauri::command]
fn delete_workspace_catalog_entry(app: tauri::AppHandle, workspace_id: String, kind: String, path: String, confirmed: bool) -> Result<(), String> {
    if !confirmed { return Err(safe_error("Подтвердите удаление элемента.")); }
    storage::list_workspaces(&task_connection(&app)?)?.iter().find(|workspace| workspace.id == workspace_id).ok_or_else(|| safe_error("Рабочее пространство не найдено."))?;
    config_store::delete_in_workspace(&app_data_directory(&app)?, &workspace_id, &kind, &path)
}

#[tauri::command]
fn copy_catalog_entry(app: tauri::AppHandle, source_workspace_id: String, workspace_id: String, kind: String, path: String) -> Result<(), String> {
    let spaces = storage::list_workspaces(&task_connection(&app)?)?;
    if !spaces.iter().any(|item| item.id == source_workspace_id) || !spaces.iter().any(|item| item.id == workspace_id) { return Err(safe_error("Рабочее пространство не найдено.")); }
    config_store::copy_entry(&app_data_directory(&app)?, &source_workspace_id, &workspace_id, &kind, &path)
}

#[tauri::command]
fn analyse_external_configuration(
    app: tauri::AppHandle,
    sessions: tauri::State<'_, Mutex<config_store::ImportSessions>>,
    source_path: String,
) -> Result<config_store::Analysis, String> {
    let analysis = config_store::analyse(&app_data_directory(&app)?, &source_path)?;
    sessions.lock().map_err(|_| safe_error("Сеанс анализа недоступен."))?.analyses.insert(analysis.id.clone(), analysis.clone());
    Ok(analysis)
}

#[tauri::command]
fn preview_external_import(
    app: tauri::AppHandle,
    sessions: tauri::State<'_, Mutex<config_store::ImportSessions>>,
    analysis_id: String,
    mode: String,
) -> Result<config_store::ImportPreview, String> {
    let mut store = sessions.lock().map_err(|_| safe_error("Сеанс анализа недоступен."))?;
    let analysis = store.analyses.get(&analysis_id).cloned().ok_or_else(|| safe_error("Результат анализа устарел; выполните анализ заново."))?;
    let preview = config_store::preview(&app_data_directory(&app)?, &analysis, &mode)?;
    store.previews.insert(preview.id.clone(), preview.clone());
    Ok(preview)
}

#[tauri::command]
fn apply_external_import(
    app: tauri::AppHandle,
    sessions: tauri::State<'_, Mutex<config_store::ImportSessions>>,
    analysis_id: String,
    preview_id: String,
    confirmed_replace: bool,
) -> Result<(), String> {
    let store = sessions.lock().map_err(|_| safe_error("Сеанс анализа недоступен."))?;
    let analysis = store.analyses.get(&analysis_id).ok_or_else(|| safe_error("Результат анализа устарел; выполните анализ заново."))?;
    let preview = store.previews.get(&preview_id).ok_or_else(|| safe_error("Preview импорта устарел; сформируйте его заново."))?;
    config_store::apply(&app_data_directory(&app)?, analysis, preview, confirmed_replace)
}

#[tauri::command]
fn list_entities(
    app: tauri::AppHandle,
    kind: storage::EntityKind,
) -> Result<Vec<storage::Entity>, String> {
    storage::list(&task_connection(&app)?, kind)
}

#[tauri::command]
fn list_workspace_entities(
    app: tauri::AppHandle,
    kind: storage::EntityKind,
    workspace_id: String,
    folder_id: Option<String>,
) -> Result<Vec<storage::Entity>, String> {
    storage::list_scoped(&task_connection(&app)?, kind, &workspace_id, folder_id.as_deref())
}

#[tauri::command]
fn list_workspaces(app: tauri::AppHandle) -> Result<Vec<storage::Workspace>, String> {
    storage::list_workspaces(&task_connection(&app)?)
}

#[tauri::command]
fn save_workspace(app: tauri::AppHandle, input: storage::WorkspaceInput) -> Result<storage::Workspace, String> {
    storage::save_workspace(&task_connection(&app)?, input)
}

#[tauri::command]
fn delete_workspace(app: tauri::AppHandle, workspace_id: String, confirmed: bool) -> Result<usize, String> {
    if !confirmed { return Err(safe_error("Подтвердите удаление рабочего пространства.")); }
    storage::delete_workspace(&task_connection(&app)?, &workspace_id)
}

#[tauri::command]
fn list_folders(app: tauri::AppHandle, workspace_id: String) -> Result<Vec<storage::Folder>, String> {
    storage::list_folders(&task_connection(&app)?, &workspace_id)
}

#[tauri::command]
fn save_folder(app: tauri::AppHandle, input: storage::FolderInput) -> Result<storage::Folder, String> {
    storage::save_folder(&task_connection(&app)?, input)
}

#[tauri::command]
fn delete_folder(app: tauri::AppHandle, folder_id: String, confirmed: bool) -> Result<usize, String> {
    if !confirmed { return Err(safe_error("Подтвердите удаление папки.")); }
    storage::delete_folder(&task_connection(&app)?, &folder_id)
}

#[tauri::command]
fn copy_folder(app: tauri::AppHandle, folder_id: String, workspace_id: String, parent_id: Option<String>) -> Result<storage::Folder, String> {
    storage::copy_folder(&task_connection(&app)?, &folder_id, &workspace_id, parent_id.as_deref())
}

#[tauri::command]
fn save_entity(
    app: tauri::AppHandle,
    kind: storage::EntityKind,
    input: storage::EntityInput,
) -> Result<storage::Entity, String> {
    storage::save(&task_connection(&app)?, kind, input)
}

#[tauri::command]
fn copy_entity(
    app: tauri::AppHandle,
    kind: storage::EntityKind,
    id: String,
    workspace_id: String,
    folder_id: Option<String>,
) -> Result<storage::Entity, String> {
    storage::copy_entity(&task_connection(&app)?, kind, &id, &workspace_id, folder_id.as_deref())
}

#[tauri::command]
fn delete_entity(
    app: tauri::AppHandle,
    kind: storage::EntityKind,
    id: String,
) -> Result<(), String> {
    storage::delete(&task_connection(&app)?, kind, &id)
}

#[tauri::command]
fn clear_entities(
    app: tauri::AppHandle,
    kind: storage::EntityKind,
    confirmed: bool,
) -> Result<usize, String> {
    if !confirmed { return Err(safe_error("Подтвердите очистку категории.")); }
    storage::clear(&task_connection(&app)?, kind)
}

#[tauri::command]
fn clear_all_local_data(app: tauri::AppHandle, confirmed: bool) -> Result<usize, String> {
    if !confirmed { return Err(safe_error("Подтвердите полное очищение.")); }
    let data = app_data_directory(&app)?;
    let count = storage::clear_all(&task_connection(&app)?)?;
    let rules = config_store::clear_kind(&data, "rules")?;
    let skills = config_store::clear_kind(&data, "skills")?;
    Ok(count + rules + skills)
}

#[tauri::command]
fn save_task_as_template(
    app: tauri::AppHandle,
    task_id: String,
    name: String,
    description: Option<String>,
) -> Result<storage::Entity, String> {
    storage::save_task_as_template(&task_connection(&app)?, &task_id, name, description)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(Mutex::new(config_store::ImportSessions::default()))
        .invoke_handler(tauri::generate_handler![
            preview_profile,
            apply_profile,
            list_entities,
            list_workspace_entities,
            save_entity,
            copy_entity,
            delete_entity,
            clear_entities,
            clear_all_local_data,
            save_task_as_template,
            list_workspaces,
            save_workspace,
            delete_workspace,
            list_folders,
            save_folder,
            delete_folder,
            copy_folder,
            list_catalog_entries,
            list_workspace_catalog_entries,
            save_catalog_entry,
            save_workspace_catalog_entry,
            clear_catalog_entries,
            clear_workspace_catalog_entries,
            delete_catalog_entry,
            delete_workspace_catalog_entry,
            copy_catalog_entry,
            analyse_external_configuration,
            preview_external_import,
            apply_external_import
        ])
        .run(tauri::generate_context!())
        .expect("Ошибка запуска Agent Foundry");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temporary_target(label: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!("ai-works-{label}-{}", now_timestamp()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).unwrap();
        path
    }

    #[test]
    fn catalog_produces_distinct_destinations() {
        let catalog = catalog().unwrap();
        let files = plan_files(&catalog, "standard").unwrap();
        assert_eq!(files.len(), 15);
        assert!(files.iter().any(|file| file.relative == "AGENTS.md"));
        assert!(files
            .iter()
            .any(|file| file.relative == ".agents/skills/sdd-workflow/SKILL.md"));
        assert!(files
            .iter()
            .any(|file| file.relative == ".agents/skills/reviewer/SKILL.md"));
    }

    #[test]
    fn preview_is_read_only_and_detects_conflict() {
        let target = temporary_target("preview");
        fs::write(target.join("AGENTS.md"), "пользовательское правило").unwrap();
        let (_, _, preview) = build_preview(&target, "standard").unwrap();
        assert!(!preview.can_apply);
        assert_eq!(preview.operations[0].kind, "conflict");
        assert_eq!(
            fs::read_to_string(target.join("AGENTS.md")).unwrap(),
            "пользовательское правило"
        );
        let _ = fs::remove_dir_all(target);
    }

    #[test]
    fn validator_rejects_traversal_and_unknown_sources() {
        assert!(validate_relative_path("../outside").is_err());
        assert!(validate_relative_path("/absolute").is_err());
        assert!(source_content("rules/missing.md").is_err());
        assert!(plan_files(&catalog().unwrap(), "missing-profile").is_err());
    }

    #[test]
    fn preview_recognizes_a_managed_file_as_update() {
        let target = temporary_target("update");
        let old = b"previous managed content";
        fs::write(target.join("AGENTS.md"), old).unwrap();
        fs::create_dir_all(target.join(".agent-config")).unwrap();
        let mut files = BTreeMap::new();
        files.insert("AGENTS.md".into(), sha256(old));
        let manifest = Manifest {
            version: MANIFEST_VERSION,
            profile: "standard".into(),
            installed_at: now_timestamp(),
            files,
        };
        fs::write(
            target.join(".agent-config/manifest.json"),
            serde_json::to_vec(&manifest).unwrap(),
        )
        .unwrap();
        let (_, _, preview) = build_preview(&target, "standard").unwrap();
        assert_eq!(preview.operations[0].kind, "update");
        let _ = fs::remove_dir_all(target);
    }

    #[test]
    fn transaction_writes_manifest_and_is_idempotent() {
        let target = temporary_target("apply");
        let catalog = catalog().unwrap();
        let files = plan_files(&catalog, "standard").unwrap();
        let (_, _, preview) = build_preview(&target, "standard").unwrap();
        apply_plan(
            &target,
            &catalog,
            "standard",
            &files,
            &preview,
            None,
            &mut |_| Ok(()),
        )
        .unwrap();
        let (_, _, second) = build_preview(&target, "standard").unwrap();
        assert!(second.can_apply);
        assert!(second
            .operations
            .iter()
            .all(|operation| operation.kind == "unchanged"));
        assert!(target.join(".agent-config/manifest.json").exists());
        let _ = fs::remove_dir_all(target);
    }

    #[test]
    fn installation_progress_reports_rules_skills_then_manifest() {
        let target = temporary_target("progress");
        let catalog = catalog().unwrap();
        let files = plan_files(&catalog, "standard").unwrap();
        let (_, _, preview) = build_preview(&target, "standard").unwrap();
        let mut stages = Vec::new();
        apply_plan(&target, &catalog, "standard", &files, &preview, None, &mut |stage| { stages.push(stage.to_owned()); Ok(()) }).unwrap();
        let first_rules = stages.iter().position(|stage| stage == "rules").unwrap();
        let first_skills = stages.iter().position(|stage| stage == "skills").unwrap();
        assert!(first_rules < first_skills);
        assert_eq!(stages.last().map(String::as_str), Some("manifest"));
        let _ = fs::remove_dir_all(target);
    }

    #[test]
    fn failed_transaction_restores_previous_files() {
        let target = temporary_target("rollback");
        let catalog = catalog().unwrap();
        let files = plan_files(&catalog, "standard").unwrap();
        let (_, _, preview) = build_preview(&target, "standard").unwrap();
        assert!(apply_plan(
            &target,
            &catalog,
            "standard",
            &files,
            &preview,
            Some(1),
            &mut |_| Ok(())
        )
        .is_err());
        assert!(!target.join("AGENTS.md").exists());
        assert!(!target.join(".agents/skills/sdd-workflow/SKILL.md").exists());
        assert!(!target.join(".agent-config/manifest.json").exists());
        let _ = fs::remove_dir_all(target);
    }
}
