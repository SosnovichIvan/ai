import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import type { CatalogEntry, Entity, EntityInput, EntityKind, ExternalAnalysis, Folder, FolderInput, ImportMode, ImportPreview, InstallationPreview, Progress, WorkspaceInput, WorkspaceRecord } from "./types";

const isTauri = "__TAURI_INTERNALS__" in window;

export async function chooseTargetFolder(): Promise<string | null> {
  if (!isTauri) return null;
  const selected = await open({ directory: true, multiple: false, title: "Выберите папку проекта" });
  return typeof selected === "string" ? selected : null;
}

export async function chooseConfigurationFolder(): Promise<string | null> {
  if (!isTauri) return null;
  const selected = await open({ directory: true, multiple: false, title: "Выберите папку с rules и skills" });
  return typeof selected === "string" ? selected : null;
}

export async function previewInstallation(targetPath: string): Promise<InstallationPreview> {
  return invoke<InstallationPreview>("preview_profile", { targetPath, profile: "standard" });
}

export async function applyInstallation(targetPath: string): Promise<void> {
  await invoke("apply_profile", { targetPath, profile: "standard" });
}

export async function listenInstallationProgress(onProgress: (progress: Progress) => void): Promise<() => void> {
  if (!isTauri) return () => undefined;
  const { listen } = await import("@tauri-apps/api/event");
  return listen<Progress>("installation-progress", (event) => onProgress(event.payload));
}

export function listEntities(kind: EntityKind) { return invoke<Entity[]>("list_entities", { kind }); }
export function listWorkspaceEntities(kind: EntityKind, workspaceId: string, folderId?: string | null) { return invoke<Entity[]>("list_workspace_entities", { kind, workspaceId, folderId: folderId ?? null }); }
export function saveEntity(kind: EntityKind, input: EntityInput) { return invoke<Entity>("save_entity", { kind, input }); }
export function copyEntity(kind: EntityKind, id: string, workspaceId: string, folderId?: string | null) { return invoke<Entity>("copy_entity", { kind, id, workspaceId, folderId: folderId ?? null }); }
export function deleteEntity(kind: EntityKind, id: string) { return invoke<void>("delete_entity", { kind, id }); }
export function clearEntities(kind: EntityKind) { return invoke<number>("clear_entities", { kind, confirmed: true }); }
export function clearAllLocalData() { return invoke<number>("clear_all_local_data", { confirmed: true }); }
export function saveTaskAsTemplate(taskId: string, name: string, description?: string | null) { return invoke<Entity>("save_task_as_template", { taskId, name, description: description ?? null }); }
export function listWorkspaces() { return invoke<WorkspaceRecord[]>("list_workspaces"); }
export function saveWorkspace(input: WorkspaceInput) { return invoke<WorkspaceRecord>("save_workspace", { input }); }
export function deleteWorkspace(workspaceId: string) { return invoke<number>("delete_workspace", { workspaceId, confirmed: true }); }
export function listFolders(workspaceId: string) { return invoke<Folder[]>("list_folders", { workspaceId }); }
export function saveFolder(input: FolderInput) { return invoke<Folder>("save_folder", { input }); }
export function deleteFolder(folderId: string) { return invoke<number>("delete_folder", { folderId, confirmed: true }); }
export function copyFolder(folderId: string, workspaceId: string, parentId?: string | null) { return invoke<Folder>("copy_folder", { folderId, workspaceId, parentId: parentId ?? null }); }
export function listCatalogEntries(kind: "skills" | "rules") { return invoke<CatalogEntry[]>("list_catalog_entries", { kind }); }
export function listWorkspaceCatalogEntries(workspaceId: string, kind: "skills" | "rules") { return invoke<CatalogEntry[]>("list_workspace_catalog_entries", { workspaceId, kind }); }
export function saveCatalogEntry(kind: "skills" | "rules", path: string, content: string) { return invoke<void>("save_catalog_entry", { kind, path, content }); }
export function saveWorkspaceCatalogEntry(workspaceId: string, kind: "skills" | "rules", path: string, content: string) { return invoke<void>("save_workspace_catalog_entry", { workspaceId, kind, path, content }); }
export function clearCatalogEntries(kind: "skills" | "rules") { return invoke<number>("clear_catalog_entries", { kind, confirmed: true }); }
export function clearWorkspaceCatalogEntries(workspaceId: string, kind: "skills" | "rules") { return invoke<number>("clear_workspace_catalog_entries", { workspaceId, kind, confirmed: true }); }
export function deleteCatalogEntry(kind: "skills" | "rules", path: string) { return invoke<void>("delete_catalog_entry", { kind, path, confirmed: true }); }
export function deleteWorkspaceCatalogEntry(workspaceId: string, kind: "skills" | "rules", path: string) { return invoke<void>("delete_workspace_catalog_entry", { workspaceId, kind, path, confirmed: true }); }
export function copyCatalogEntry(sourceWorkspaceId: string, workspaceId: string, kind: "skills" | "rules", path: string) { return invoke<void>("copy_catalog_entry", { sourceWorkspaceId, workspaceId, kind, path }); }
export function analyseExternalConfiguration(sourcePath: string) { return invoke<ExternalAnalysis>("analyse_external_configuration", { sourcePath }); }
export function previewExternalImport(analysisId: string, mode: ImportMode) { return invoke<ImportPreview>("preview_external_import", { analysisId, mode }); }
export function applyExternalImport(analysisId: string, previewId: string, confirmedReplace: boolean) { return invoke<void>("apply_external_import", { analysisId, previewId, confirmedReplace }); }
