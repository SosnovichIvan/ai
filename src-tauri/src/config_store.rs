use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::{Component, Path, PathBuf}, time::{SystemTime, UNIX_EPOCH}};
use uuid::Uuid;

const SEED_FILES: &[(&str, &[u8])] = &[];
pub const CATALOG_VERSION: u32 = 2;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CatalogEntry { pub kind: String, pub path: String, pub content: String, pub hash: String }

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Analysis { pub id: String, pub source_path: String, pub entries: Vec<CatalogEntry>, pub excluded_count: usize, pub unclassified_count: usize }

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ImportPreview { pub id: String, pub mode: String, pub operations: Vec<ImportOperation>, pub can_apply: bool }

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ImportOperation { pub path: String, pub kind: String, pub reason: String }

#[derive(Default)]
pub struct ImportSessions { pub analyses: BTreeMap<String, Analysis>, pub previews: BTreeMap<String, ImportPreview> }

fn error(message: &str) -> String { message.to_owned() }
fn hash(bytes: &[u8]) -> String { format!("{:x}", Sha256::digest(bytes)) }
fn stamp() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() }

pub fn root(data: &Path) -> PathBuf { data.join("catalog") }

/// Возвращает версию существующего общего каталога без его создания.
pub fn installed_catalog_version(data: &Path) -> Result<Option<u32>, String> {
    let manifest = root(data).join("catalog-manifest.json");
    if !manifest.exists() { return Ok(None); }
    let bytes = fs::read(manifest).map_err(|_| error("Не удалось прочитать manifest каталога."))?;
    let value: serde_json::Value = serde_json::from_slice(&bytes)
        .map_err(|_| error("Локальное состояние требует восстановления: manifest каталога повреждён. Данные не изменены."))?;
    value.get("version").and_then(serde_json::Value::as_u64).map(|version| version as u32)
        .map(Some)
        .ok_or_else(|| error("Локальное состояние требует восстановления: manifest каталога не содержит версию."))
}

fn valid_workspace_id(id: &str) -> bool { !id.is_empty() && id.len() <= 128 && id.chars().all(|character| character.is_ascii_alphanumeric() || character == '-') }

fn root_for_workspace(data: &Path, workspace_id: &str) -> Result<PathBuf, String> {
    if !valid_workspace_id(workspace_id) { return Err(error("Недопустимый идентификатор рабочего пространства.")); }
    if workspace_id == "default" { return Ok(root(data)); }
    Ok(root(data).join("workspaces").join(workspace_id))
}

pub fn ensure_seed(data: &Path) -> Result<PathBuf, String> {
    let root = root(data);
    fs::create_dir_all(&root).map_err(|_| error("Не удалось подготовить каталог конфигурации."))?;
    for (relative, content) in SEED_FILES {
        if (relative.starts_with("rules/") && root.join(".cleared-rules").exists()) || (relative.starts_with("skills/") && root.join(".cleared-skills").exists()) { continue; }
        let destination = root.join(relative);
        if destination.exists() { continue; }
        if let Some(parent) = destination.parent() { fs::create_dir_all(parent).map_err(|_| error("Не удалось подготовить каталог конфигурации."))?; }
        fs::write(destination, content).map_err(|_| error("Не удалось подготовить начальный каталог конфигурации."))?;
    }
    let manifest = root.join("catalog-manifest.json");
    if !manifest.exists() {
        fs::write(manifest, serde_json::to_vec_pretty(&serde_json::json!({"version": CATALOG_VERSION, "source": "bundled", "updatedAt": stamp()})).unwrap())
            .map_err(|_| error("Не удалось подготовить manifest каталога."))?;
    }
    Ok(root)
}

fn ensure_workspace_catalog(data: &Path, workspace_id: &str) -> Result<PathBuf, String> {
    if workspace_id == "default" { return ensure_seed(data); }
    let root = root_for_workspace(data, workspace_id)?;
    fs::create_dir_all(&root).map_err(|_| error("Не удалось подготовить каталог рабочего пространства."))?;
    let manifest = root.join("catalog-manifest.json");
    if !manifest.exists() { fs::write(manifest, serde_json::to_vec_pretty(&serde_json::json!({"version": CATALOG_VERSION, "source": "workspace", "updatedAt": stamp()})).unwrap()).map_err(|_| error("Не удалось подготовить manifest каталога."))?; }
    Ok(root)
}

pub fn clear_kind(data: &Path, kind: &str) -> Result<usize, String> {
    clear_kind_in_workspace(data, "default", kind)
}

pub fn clear_kind_in_workspace(data: &Path, workspace_id: &str, kind: &str) -> Result<usize, String> {
    let root = ensure_workspace_catalog(data, workspace_id)?;
    let folder = match kind { "rules" => root.join("rules"), "skills" => root.join("skills"), _ => return Err(error("Неизвестная категория конфигурации.")) };
    let count = if folder.exists() { fs::read_dir(&folder).map_err(|_| error("Не удалось прочитать каталог."))?.count() } else { 0 };
    if folder.exists() { fs::remove_dir_all(&folder).map_err(|_| error("Не удалось очистить каталог конфигурации."))?; }
    fs::create_dir_all(&folder).map_err(|_| error("Не удалось подготовить каталог конфигурации."))?;
    fs::write(root.join(format!(".cleared-{kind}")), b"1").map_err(|_| error("Не удалось сохранить состояние очистки."))?;
    Ok(count)
}

fn relative_valid(path: &str) -> bool {
    !path.is_empty() && !Path::new(path).is_absolute() && !Path::new(path).components().any(|p| matches!(p, Component::ParentDir | Component::RootDir | Component::Prefix(_)))
}

fn classify(relative: &Path, root_kind: Option<&str>, content: &str) -> Option<(String, String)> {
    if relative.extension().and_then(|v| v.to_str()) != Some("md") { return None; }
    let parts: Vec<_> = relative.components().filter_map(|p| match p { Component::Normal(v) => v.to_str(), _ => None }).collect();
    let name = relative.file_name().and_then(|value| value.to_str()).unwrap_or_default();
    let lower = content.to_ascii_lowercase();
    let path_has_skills = root_kind == Some("skills") || parts.iter().any(|part| *part == "skills");
    let path_has_rules = root_kind == Some("rules") || parts.iter().any(|part| *part == "rules" || *part == "instructions" || *part == ".ai");
    let skill_marked = name == "SKILL.md" || lower.starts_with("---\nname:") || lower.contains("# skill") || lower.contains("# скил") || lower.contains("# навык");
    let rule_marked = name == "AGENTS.md" || name.ends_with("-rules.md") || name == "project-rules.md" || lower.contains("# rules") || lower.contains("# правила") || lower.contains("## обязательные требования") || lower.contains("## запреты");
    let kind = if path_has_skills || skill_marked { "skills" } else if path_has_rules || rule_marked { "rules" } else { return None; };
    let destination = if kind == "skills" {
        let skill_name = if name == "SKILL.md" { relative.parent().and_then(Path::file_name).and_then(|value| value.to_str()).unwrap_or("imported-skill") }
            else { relative.parent().and_then(Path::file_name).and_then(|value| value.to_str()).filter(|value| *value != ".").unwrap_or("imported-skill") };
        format!("skills/{skill_name}/SKILL.md")
    } else {
        let filename = if name == "README.md" { relative.parent().and_then(Path::file_name).and_then(|value| value.to_str()).map(|value| format!("{value}.md")).unwrap_or_else(|| "README.md".into()) } else { name.into() };
        format!("rules/{filename}")
    };
    Some((kind.into(), destination))
}

fn excluded(name: &str) -> bool {
    let lowered = name.to_ascii_lowercase();
    lowered == ".git" || lowered == "node_modules" || lowered == "target" || lowered == "dist" || lowered == "build" || lowered == ".next" || lowered == ".env" || lowered.starts_with(".env.") || lowered.contains("token") || lowered.contains("cookie") || lowered.contains("credential") || lowered.contains("private") || lowered.ends_with(".pem") || lowered.ends_with(".key")
}

fn walk(root: &Path, current: &Path, root_kind: Option<&str>, entries: &mut Vec<CatalogEntry>, excluded_count: &mut usize, unclassified_count: &mut usize) -> Result<(), String> {
    for item in fs::read_dir(current).map_err(|_| error("Не удалось прочитать выбранную папку."))? {
        let item = item.map_err(|_| error("Не удалось прочитать выбранную папку."))?;
        let metadata = fs::symlink_metadata(item.path()).map_err(|_| error("Не удалось проверить файл."))?;
        let name = item.file_name(); let name = name.to_string_lossy();
        if metadata.file_type().is_symlink() || excluded(&name) { *excluded_count += 1; continue; }
        if metadata.is_dir() { walk(root, &item.path(), root_kind, entries, excluded_count, unclassified_count)?; continue; }
        if !metadata.is_file() { continue; }
        let item_path = item.path();
        let relative = item_path.strip_prefix(root).map_err(|_| error("Недопустимый путь анализа."))?;
        if metadata.len() > 1_048_576 { *excluded_count += 1; continue; }
        let content = fs::read_to_string(item.path()).map_err(|_| error("Допустимый текстовый файл не удалось прочитать."))?;
        let Some((kind, destination)) = classify(relative, root_kind, &content) else { if relative.extension().and_then(|value| value.to_str()) == Some("md") { *unclassified_count += 1; } continue; };
        if !relative_valid(&destination) { *unclassified_count += 1; continue; }
        entries.push(CatalogEntry { kind, path: destination, hash: hash(content.as_bytes()), content });
    }
    Ok(())
}

pub fn analyse(data: &Path, selected: &str) -> Result<Analysis, String> {
    ensure_seed(data)?;
    let source = fs::canonicalize(selected).map_err(|_| error("Не удалось нормализовать выбранную папку."))?;
    if !source.is_dir() { return Err(error("Выбранный путь не является папкой.")); }
    let root_kind = source.file_name().and_then(|value| value.to_str()).map(|value| value.to_ascii_lowercase());
    let root_kind = match root_kind.as_deref() { Some("rules") | Some("instructions") => Some("rules"), Some("skills") => Some("skills"), _ => None };
    let mut entries = Vec::new(); let mut excluded_count = 0; let mut unclassified_count = 0;
    walk(&source, &source, root_kind, &mut entries, &mut excluded_count, &mut unclassified_count)?;
    entries.sort_by(|a, b| a.path.cmp(&b.path));
    entries.dedup_by(|a, b| a.path == b.path);
    Ok(Analysis { id: Uuid::new_v4().to_string(), source_path: source.display().to_string(), entries, excluded_count, unclassified_count })
}

pub fn list(data: &Path, kind: &str) -> Result<Vec<CatalogEntry>, String> {
    list_in_workspace(data, "default", kind)
}

pub fn list_in_workspace(data: &Path, workspace_id: &str, kind: &str) -> Result<Vec<CatalogEntry>, String> {
    let root = ensure_workspace_catalog(data, workspace_id)?;
    let folder = match kind { "skills" => root.join("skills"), "rules" => root.join("rules"), _ => return Err(error("Неизвестная категория конфигурации.")) };
    if !folder.exists() { return Ok(Vec::new()); }
    let mut result = Vec::new(); let mut ignored = 0; let mut unclassified = 0;
    walk(&root, &folder, Some(kind), &mut result, &mut ignored, &mut unclassified)?;
    result.retain(|entry| entry.kind == kind);
    Ok(result)
}

pub fn save(data: &Path, kind: &str, path: &str, content: &str) -> Result<(), String> {
    save_in_workspace(data, "default", kind, path, content)
}

pub fn save_in_workspace(data: &Path, workspace_id: &str, kind: &str, path: &str, content: &str) -> Result<(), String> {
    if !relative_valid(path) || content.len() > 1_048_576 { return Err(error("Недопустимые данные конфигурации.")); }
    let expected = if kind == "skills" { path.starts_with("skills/") && path.ends_with("/SKILL.md") } else { kind == "rules" && path.starts_with("rules/") && path.ends_with(".md") };
    if !expected { return Err(error("Путь не соответствует категории конфигурации.")); }
    let root = ensure_workspace_catalog(data, workspace_id)?; let destination = root.join(path);
    if !destination.starts_with(&root) { return Err(error("Путь выходит за пределы каталога.")); }
    if let Some(parent) = destination.parent() { fs::create_dir_all(parent).map_err(|_| error("Не удалось подготовить каталог конфигурации."))?; }
    fs::write(destination, content).map_err(|_| error("Не удалось сохранить конфигурацию."))
}

pub fn delete(data: &Path, kind: &str, path: &str) -> Result<(), String> {
    delete_in_workspace(data, "default", kind, path)
}

pub fn delete_in_workspace(data: &Path, workspace_id: &str, kind: &str, path: &str) -> Result<(), String> {
    if !relative_valid(path) { return Err(error("Недопустимый путь конфигурации.")); }
    let expected = if kind == "skills" { path.starts_with("skills/") && path.ends_with("/SKILL.md") } else { kind == "rules" && path.starts_with("rules/") && path.ends_with(".md") };
    if !expected { return Err(error("Путь не соответствует категории конфигурации.")); }
    let root = ensure_workspace_catalog(data, workspace_id)?; let destination = root.join(path);
    if !destination.starts_with(&root) || !destination.exists() { return Err(error("Элемент конфигурации не найден.")); }
    fs::remove_file(destination).map_err(|_| error("Не удалось удалить элемент конфигурации."))
}

pub fn copy_entry(data: &Path, source_workspace_id: &str, target_workspace_id: &str, kind: &str, path: &str) -> Result<(), String> {
    if source_workspace_id == target_workspace_id { return Err(error("Выберите другое рабочее пространство.")); }
    if !relative_valid(path) { return Err(error("Недопустимый путь конфигурации.")); }
    let source = ensure_workspace_catalog(data, source_workspace_id)?.join(path);
    if !source.exists() { return Err(error("Исходный элемент конфигурации не найден.")); }
    let content = fs::read_to_string(source).map_err(|_| error("Не удалось прочитать исходный элемент конфигурации."))?;
    save_in_workspace(data, target_workspace_id, kind, path, &content)
}

pub fn preview(data: &Path, analysis: &Analysis, mode: &str) -> Result<ImportPreview, String> {
    let root = ensure_seed(data)?;
    if mode != "add" && mode != "replace" { return Err(error("Неизвестный режим импорта.")); }
    let mut operations = Vec::new();
    for entry in &analysis.entries {
        let path = root.join(&entry.path);
        let operation = if !path.exists() { ImportOperation { path: entry.path.clone(), kind: "create".into(), reason: "Будет добавлен".into() } }
        else { let current = fs::read(&path).map_err(|_| error("Не удалось проверить активный каталог."))?;
            if hash(&current) == entry.hash { ImportOperation { path: entry.path.clone(), kind: "unchanged".into(), reason: "Уже соответствует".into() } }
            else if mode == "replace" { ImportOperation { path: entry.path.clone(), kind: "update".into(), reason: "Будет заменён после подтверждения".into() } }
            else { ImportOperation { path: entry.path.clone(), kind: "conflict".into(), reason: "Оставлен без изменений".into() } }
        }; operations.push(operation);
    }
    Ok(ImportPreview { id: Uuid::new_v4().to_string(), mode: mode.into(), can_apply: true, operations })
}

pub fn apply(data: &Path, analysis: &Analysis, import_preview: &ImportPreview, confirmed_replace: bool) -> Result<(), String> {
    if import_preview.mode == "replace" && !confirmed_replace { return Err(error("Подтвердите замену активного набора.")); }
    if !import_preview.can_apply { return Err(error("Есть конфликты: выберите замену или измените набор.")); }
    let root = ensure_seed(data)?;
    for entry in &analysis.entries {
        let source_bytes = entry.content.as_bytes();
        if hash(source_bytes) != entry.hash { return Err(error("Результат анализа устарел; выполните анализ заново.")); }
    }
    let current_preview = preview(data, analysis, &import_preview.mode)?;
    if current_preview.operations != import_preview.operations {
        return Err(error("Preview импорта устарел: активный каталог изменился. Сформируйте preview заново."));
    }
    let staging = data.join(format!("catalog-staging-{}", Uuid::new_v4()));
    if import_preview.mode == "add" { fs::create_dir_all(&staging).map_err(|_| error("Не удалось подготовить импорт."))?; copy_tree(&root, &staging)?; }
    else { fs::create_dir_all(&staging).map_err(|_| error("Не удалось подготовить импорт."))?; }
    for entry in &analysis.entries {
        let destination = staging.join(&entry.path);
        if import_preview.mode == "add" && destination.exists() { continue; }
        if let Some(parent) = destination.parent() { fs::create_dir_all(parent).map_err(|_| error("Не удалось подготовить импорт."))?; }
        fs::write(destination, &entry.content).map_err(|_| error("Не удалось записать импортируемый файл."))?;
    }
    fs::write(staging.join("catalog-manifest.json"), serde_json::to_vec_pretty(&serde_json::json!({"version": CATALOG_VERSION, "source": "external-import", "updatedAt": stamp()})).unwrap())
        .map_err(|_| error("Не удалось записать manifest каталога."))?;
    let backup = data.join("catalog-backups").join(format!("{}-{}", stamp(), Uuid::new_v4()));
    if let Some(parent) = backup.parent() { fs::create_dir_all(parent).map_err(|_| error("Не удалось подготовить резервную копию."))?; }
    fs::rename(&root, &backup).map_err(|_| error("Не удалось создать резервную копию каталога."))?;
    if let Err(_) = fs::rename(&staging, &root) { let _ = fs::rename(&backup, &root); let _ = fs::remove_dir_all(&staging); return Err(error("Не удалось применить импорт; предыдущий каталог восстановлен.")); }
    Ok(())
}

fn copy_tree(from: &Path, to: &Path) -> Result<(), String> { for item in fs::read_dir(from).map_err(|_| error("Не удалось подготовить импорт."))? { let item = item.map_err(|_| error("Не удалось подготовить импорт."))?; let file_type = item.file_type().map_err(|_| error("Не удалось подготовить импорт."))?; if file_type.is_symlink() { continue; } let target = to.join(item.file_name()); if file_type.is_dir() { fs::create_dir_all(&target).map_err(|_| error("Не удалось подготовить импорт."))?; copy_tree(&item.path(), &target)?; } else if file_type.is_file() { fs::copy(item.path(), target).map_err(|_| error("Не удалось подготовить импорт."))?; } } Ok(()) }

#[cfg(test)]
mod tests {
    use super::*;

    fn temporary(label: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!("ai-works-config-{label}-{}", Uuid::new_v4()));
        fs::create_dir_all(&path).unwrap(); path
    }

    #[test]
    fn analysis_ignores_secrets_and_discovers_allowed_files() {
        let data = temporary("data"); let source = temporary("source");
        fs::create_dir_all(source.join("rules")).unwrap();
        fs::create_dir_all(source.join(".agents/skills/example")).unwrap();
        fs::create_dir_all(source.join("node_modules/pkg")).unwrap();
        fs::write(source.join("rules/new.md"), "# Rule").unwrap();
        fs::write(source.join(".agents/skills/example/SKILL.md"), "# Skill").unwrap();
        fs::write(source.join(".env"), "secret").unwrap();
        fs::write(source.join("node_modules/pkg/ignored.md"), "ignored").unwrap();
        let analysis = analyse(&data, source.to_str().unwrap()).unwrap();
        assert_eq!(analysis.entries.len(), 2);
        assert!(analysis.entries.iter().all(|entry| !entry.content.contains("secret")));
        assert!(analysis.excluded_count >= 2);
        let _ = fs::remove_dir_all(data); let _ = fs::remove_dir_all(source);
    }

    #[test]
    fn direct_rules_and_skills_roots_are_classified_by_path_and_content() {
        let data = temporary("classified-data"); let source = temporary("classified-source");
        let rules = source.join("rules"); let skills = source.join("skills");
        fs::create_dir_all(rules.join("frontend")).unwrap();
        fs::create_dir_all(skills.join("reviewer")).unwrap();
        fs::write(rules.join("frontend/README.md"), "# Правила frontend\n\n## Обязательные требования").unwrap();
        fs::write(rules.join("notes.md"), "# Заметки команды").unwrap();
        fs::write(skills.join("reviewer/README.md"), "# Skill: reviewer\n\n## Назначение").unwrap();
        let rules_analysis = analyse(&data, rules.to_str().unwrap()).unwrap();
        assert_eq!(rules_analysis.entries.len(), 2);
        assert_eq!(rules_analysis.entries[0].kind, "rules");
        assert_eq!(rules_analysis.entries[0].path, "rules/frontend.md");
        assert_eq!(rules_analysis.unclassified_count, 0);
        let skills_analysis = analyse(&data, skills.to_str().unwrap()).unwrap();
        assert_eq!(skills_analysis.entries.len(), 1);
        assert_eq!(skills_analysis.entries[0].kind, "skills");
        assert_eq!(skills_analysis.entries[0].path, "skills/reviewer/SKILL.md");
        let misc = source.join("misc"); fs::create_dir_all(&misc).unwrap();
        fs::write(misc.join("notes.md"), "# Заметки команды").unwrap();
        let misc_analysis = analyse(&data, misc.to_str().unwrap()).unwrap();
        assert!(misc_analysis.entries.is_empty());
        assert_eq!(misc_analysis.unclassified_count, 1);
        let _ = fs::remove_dir_all(data); let _ = fs::remove_dir_all(source);
    }

    #[test]
    fn seed_is_created_once_and_preserves_local_changes() {
        let data = temporary("seed"); let catalog = ensure_seed(&data).unwrap();
        let core = catalog.join("rules/core.md"); fs::create_dir_all(core.parent().unwrap()).unwrap(); fs::write(&core, "# Local edit").unwrap();
        ensure_seed(&data).unwrap();
        assert_eq!(fs::read_to_string(core).unwrap(), "# Local edit");
        assert!(catalog.join("catalog-manifest.json").exists());
        assert_eq!(list(&data, "rules").unwrap().len(), 1);
        assert!(list(&data, "skills").unwrap().is_empty());
        let _ = fs::remove_dir_all(data);
    }

    #[test]
    fn workspace_catalogs_are_isolated() {
        let data = temporary("workspace-catalog");
        save_in_workspace(&data, "workspace-123", "rules", "rules/local.md", "# Правило").unwrap();
        assert_eq!(list_in_workspace(&data, "workspace-123", "rules").unwrap().len(), 1);
        assert!(list(&data, "rules").unwrap().is_empty());
        copy_entry(&data, "workspace-123", "workspace-456", "rules", "rules/local.md").unwrap();
        assert_eq!(list_in_workspace(&data, "workspace-456", "rules").unwrap().len(), 1);
        save_in_workspace(&data, "workspace-123", "rules", "rules/local.md", "# Изменено").unwrap();
        assert_eq!(list_in_workspace(&data, "workspace-456", "rules").unwrap()[0].content, "# Правило");
        assert!(save_in_workspace(&data, "../unsafe", "rules", "rules/no.md", "# Нет").is_err());
        let _ = fs::remove_dir_all(data);
    }

    #[test]
    fn clearing_catalog_kind_is_persistent_and_scoped() {
        let data = temporary("clear");
        ensure_seed(&data).unwrap();
        save(&data, "rules", "rules/example.md", "# Rule").unwrap();
        save(&data, "skills", "skills/example/SKILL.md", "# Skill").unwrap();
        assert!(clear_kind(&data, "rules").unwrap() > 0);
        assert!(list(&data, "rules").unwrap().is_empty());
        assert!(!list(&data, "skills").unwrap().is_empty());
        let _ = fs::remove_dir_all(data);
    }

    #[test]
    fn add_preserves_conflict_and_replace_backs_up_catalog() {
        let data = temporary("data"); let source = temporary("source");
        ensure_seed(&data).unwrap(); save(&data, "rules", "rules/core.md", "# Local core").unwrap(); fs::create_dir_all(source.join("rules")).unwrap();
        fs::write(source.join("rules/core.md"), "# External core").unwrap();
        fs::write(source.join("rules/new.md"), "# New rule").unwrap();
        let analysis = analyse(&data, source.to_str().unwrap()).unwrap();
        let add = preview(&data, &analysis, "add").unwrap();
        assert!(add.can_apply); assert!(add.operations.iter().any(|entry| entry.kind == "conflict"));
        apply(&data, &analysis, &add, false).unwrap();
        assert!(root(&data).join("rules/new.md").exists());
        assert_ne!(fs::read_to_string(root(&data).join("rules/core.md")).unwrap(), "# External core");
        let replace = preview(&data, &analysis, "replace").unwrap();
        apply(&data, &analysis, &replace, true).unwrap();
        assert_eq!(fs::read_to_string(root(&data).join("rules/core.md")).unwrap(), "# External core");
        assert!(root(&data).join("rules/new.md").exists());
        assert_eq!(installed_catalog_version(&data).unwrap(), Some(CATALOG_VERSION));
        assert!(data.join("catalog-backups").read_dir().unwrap().next().is_some());
        let _ = fs::remove_dir_all(data); let _ = fs::remove_dir_all(source);
    }

    #[test]
    fn apply_rejects_preview_when_active_catalog_changes() {
        let data = temporary("stale-preview-data"); let source = temporary("stale-preview-source");
        ensure_seed(&data).unwrap();
        fs::create_dir_all(source.join("rules")).unwrap();
        fs::write(source.join("rules/core.md"), "# External core").unwrap();
        let analysis = analyse(&data, source.to_str().unwrap()).unwrap();
        let stale = preview(&data, &analysis, "replace").unwrap();
        save(&data, "rules", "rules/core.md", "# Local change after preview").unwrap();
        assert!(apply(&data, &analysis, &stale, true).unwrap_err().contains("устарел"));
        assert_eq!(fs::read_to_string(root(&data).join("rules/core.md")).unwrap(), "# Local change after preview");
        let _ = fs::remove_dir_all(data); let _ = fs::remove_dir_all(source);
    }

    #[cfg(unix)]
    #[test]
    fn add_import_does_not_follow_runtime_catalog_symlinks() {
        use std::os::unix::fs::symlink;
        let data = temporary("runtime-symlink-data"); let source = temporary("runtime-symlink-source");
        let catalog = ensure_seed(&data).unwrap();
        let outside = source.join("outside.md"); fs::write(&outside, "не копировать").unwrap();
        fs::create_dir_all(catalog.join("rules")).unwrap();
        symlink(&outside, catalog.join("rules/external-link.md")).unwrap();
        fs::create_dir_all(source.join("rules")).unwrap();
        fs::write(source.join("rules/new.md"), "# New").unwrap();
        let analysis = analyse(&data, source.to_str().unwrap()).unwrap();
        let preview = preview(&data, &analysis, "add").unwrap();
        apply(&data, &analysis, &preview, false).unwrap();
        assert!(!root(&data).join("rules/external-link.md").exists());
        assert_eq!(fs::read_to_string(root(&data).join("rules/new.md")).unwrap(), "# New");
        let _ = fs::remove_dir_all(data); let _ = fs::remove_dir_all(source);
    }
}
