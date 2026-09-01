export type Workspace = "environment" | "tasks";
export type EnvironmentTab = "installation" | "skills" | "rules";
export type InstallStage = "idle" | "preview" | "applying" | "completed" | "conflict" | "failed";

export type PreviewOperation = {
  path: string;
  kind: "create" | "unchanged" | "update" | "conflict";
  reason: string;
};

export type InstallationPreview = {
  targetPath: string;
  profile: string;
  operations: PreviewOperation[];
  canApply: boolean;
};

export type Progress = { stage: "backup" | "rules" | "skills" | "manifest"; percent: number };

export type CatalogEntry = { kind: "skills" | "rules"; path: string; content: string; hash: string };
export type ImportMode = "add" | "replace";
export type ExternalAnalysis = { id: string; sourcePath: string; entries: CatalogEntry[]; excludedCount: number; unclassifiedCount: number };
export type ImportOperation = { path: string; kind: "create" | "unchanged" | "update" | "conflict"; reason: string };
export type ImportPreview = { id: string; mode: ImportMode; operations: ImportOperation[]; canApply: boolean };

export type EntityKind = "task" | "template" | "taskBlock";
export type Entity = { id: string; name: string; description: string | null; content: string; sourceTemplateId: string | null; blockSnapshot: string | null; createdAt: number; updatedAt: number; revision: number; workspaceId: string; folderId: string | null };
export type EntityInput = { id?: string; name: string; description?: string | null; content?: string; sourceTemplateId?: string | null; blockSnapshot?: string | null; workspaceId?: string | null; folderId?: string | null };
export type WorkspaceRecord = { id: string; name: string; description: string | null; createdAt: number; updatedAt: number };
export type WorkspaceInput = { id?: string; name: string; description?: string | null };
export type Folder = { id: string; workspaceId: string; parentId: string | null; name: string; createdAt: number; updatedAt: number };
export type FolderInput = { id?: string; workspaceId: string; parentId?: string | null; name: string };
