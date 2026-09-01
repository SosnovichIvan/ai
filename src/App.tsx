import { useCallback, useEffect, useRef, useState, type ReactNode } from "react";
import { analyseExternalConfiguration, applyExternalImport, applyInstallation, chooseConfigurationFolder, chooseTargetFolder, clearAllLocalData, clearEntities, copyCatalogEntry, copyEntity, deleteEntity, deleteFolder, deleteWorkspace, deleteWorkspaceCatalogEntry, listenInstallationProgress, previewExternalImport, previewInstallation, saveEntity, saveTaskAsTemplate, listFolders, listWorkspaceCatalogEntries, listWorkspaceEntities, listWorkspaces, saveFolder, saveWorkspace, saveWorkspaceCatalogEntry, clearWorkspaceCatalogEntries } from "./native";
import type { CatalogEntry, Entity, EntityKind, EnvironmentTab, ExternalAnalysis, Folder, ImportMode, ImportPreview, InstallationPreview, InstallStage, Progress, Workspace, WorkspaceRecord } from "./types";

const stageLabel: Record<Progress["stage"], string> = {
  backup: "Создаём резервные копии",
  rules: "Устанавливаем правила",
  skills: "Устанавливаем skills",
  manifest: "Обновляем manifest"
};

export function App() {
  const [workspace, setWorkspace] = useState<Workspace>("environment");
  const [tab, setTab] = useState<EnvironmentTab>("installation");
  const [targetPath, setTargetPath] = useState<string | null>(null);
  const [preview, setPreview] = useState<InstallationPreview | null>(null);
  const [state, setState] = useState<InstallStage>("idle");
  const [progress, setProgress] = useState<Progress>({ stage: "backup", percent: 0 });
  const [error, setError] = useState<string | null>(null);
  const clearEverything = async () => { if (!window.confirm("Полностью очистить локальные задачи, шаблоны, блоки, rules и skills? Файлы выбранного проекта не будут затронуты.")) return; try { const count = await clearAllLocalData(); setError(`Очищено локальных элементов: ${count}.`); } catch (cause) { setError(safeError(cause)); } };

  useEffect(() => {
    let stop: () => void = () => {};
    void listenInstallationProgress((next) => setProgress(next)).then((unlisten) => { stop = unlisten; });
    return () => stop();
  }, []);

  async function selectFolder() {
    setError(null);
    const path = await chooseTargetFolder();
    if (!path) return;
    setTargetPath(path);
    setPreview(null);
    setState("preview");
    try {
      const next = await previewInstallation(path);
      setPreview(next);
      setState(next.canApply ? "idle" : "conflict");
    } catch (cause) {
      setState("failed");
      setError(safeError(cause));
    }
  }

  async function applyProfile() {
    if (!targetPath || !preview?.canApply) return;
    setError(null);
    setProgress({ stage: "backup", percent: 0 });
    setState("applying");
    try {
      await applyInstallation(targetPath);
      setProgress({ stage: "manifest", percent: 100 });
      setState("completed");
    } catch (cause) {
      setState("failed");
      setError(safeError(cause));
    }
  }

  return <main className="app-shell">
    <aside className="workspace-nav" aria-label="Рабочие области">
      <p className="eyebrow">Agent Foundry</p>
      <button className={workspace === "environment" ? "nav-item active" : "nav-item"} onClick={() => setWorkspace("environment")}>Установка окружения</button>
      <button className={workspace === "tasks" ? "nav-item active" : "nav-item"} onClick={() => setWorkspace("tasks")}>Задачи и шаблоны</button>
      <button className="link-button danger" onClick={() => void clearEverything()}>Полностью очистить</button>
    </aside>
    {workspace === "environment" ? <section className="workspace"><header className="workspace-header"><h1>Установка окружения</h1><div className="header-actions"><button className="button secondary" onClick={() => void selectFolder()}>Выбрать папку</button><button className="button primary" disabled={!preview?.canApply || state === "applying"} onClick={() => void applyProfile()}>Применить профиль</button></div></header>
      <div className="tabs" role="tablist"><Tab label="Установка" selected={tab === "installation"} onClick={() => setTab("installation")} /><Tab label="Skills" selected={tab === "skills"} onClick={() => setTab("skills")} /><Tab label="Rules" selected={tab === "rules"} onClick={() => setTab("rules")} /></div>
      {tab === "installation" ? <><Installation targetPath={targetPath} preview={preview} state={state} progress={progress} error={error} /><ImportConfigurator /></> : <SourceEditor key={tab} kind={tab} />}
    </section> : <section className="workspace"><header className="workspace-header"><h1>Задачи и шаблоны</h1></header><TaskWorkspace /></section>}
  </main>;
}

function Tab({ label, selected, onClick }: { label: string; selected: boolean; onClick: () => void }) { return <button role="tab" aria-selected={selected} className={selected ? "tab active" : "tab"} onClick={onClick}>{label}</button>; }

function Installation({ targetPath, preview, state, progress, error }: { targetPath: string | null; preview: InstallationPreview | null; state: InstallStage; progress: Progress; error: string | null }) {
  return <><p className="intro">Выберите проект, просмотрите изменения и примените профиль безопасно.</p><article className="surface environment-card"><span className="field-label">Целевая папка</span><strong>{targetPath ?? "Выберите папку проекта"}</strong>{!targetPath && <p className="muted">Нажмите «Выбрать папку» — откроется системное окно выбора директории.</p>}<span className="field-label">Профиль</span><strong>standard <span className="muted">· правила и skills</span></strong><Preview preview={preview} state={state} progress={progress} error={error} /></article></>;
}

function Preview({ preview, state, progress, error }: { preview: InstallationPreview | null; state: InstallStage; progress: Progress; error: string | null }) {
  if (state === "applying") return <div className="preview"><strong>{stageLabel[progress.stage]}</strong><progress max="100" value={progress.percent} /><span>{progress.percent}%</span></div>;
  if (state === "completed") return <div className="preview success"><strong>Профиль применён</strong><p>Правила, skills и manifest успешно установлены.</p></div>;
  if (state === "failed") return <div className="preview error"><strong>Установка не выполнена</strong><p>{error ?? "Безопасная операция была отменена."}</p></div>;
  if (preview) return <div className={state === "conflict" ? "preview error" : "preview"}><strong>Preview установки</strong>{preview.operations.map((operation) => <p key={operation.path}><b>{operation.kind}</b> · {operation.path} <span className="muted">— {operation.reason}</span></p>)}{state === "conflict" && <p>Конфликты нужно разрешить до применения профиля.</p>}</div>;
  return <div className="preview"><strong>Preview установки</strong><p>Сначала выберите папку, чтобы увидеть create / update / conflict.</p></div>;
}

function SourceEditor({ kind }: { kind: "skills" | "rules" }) {
  const [workspaces, setWorkspaces] = useState<WorkspaceRecord[]>([]);
  const [workspaceId, setWorkspaceId] = useState("default");
  const [files, setFiles] = useState<CatalogEntry[]>([]);
  const [selected, setSelected] = useState(0);
  const [value, setValue] = useState("");
  const [saved, setSaved] = useState("");
  const [message, setMessage] = useState<string | null>(null);
  useEffect(() => { void listWorkspaces().then(setWorkspaces).catch((cause) => setMessage(safeError(cause))); }, []);
  useEffect(() => { void listWorkspaceCatalogEntries(workspaceId, kind).then((next) => { setFiles(next); setSelected(0); setValue(next[0]?.content ?? ""); setSaved(next[0]?.content ?? ""); }).catch((cause) => setMessage(safeError(cause))); }, [kind, workspaceId]);
  const dirty = value !== saved;
  const active = files[selected];
  const select = (index: number) => { const file = files[index]; if (!file) return; setSelected(index); setValue(file.content); setSaved(file.content); setMessage(null); };
  const save = async () => { if (!active) return; try { await saveWorkspaceCatalogEntry(workspaceId, kind, active.path, value); setFiles((all) => all.map((file, index) => index === selected ? { ...file, content: value } : file)); setSaved(value); setMessage("Изменения сохранены в рабочем пространстве."); } catch (cause) { setMessage(safeError(cause)); } };
  const clear = async () => { if (!files.length || !window.confirm(`Очистить все ${kind} в рабочем пространстве?`)) return; try { const count = await clearWorkspaceCatalogEntries(workspaceId, kind); setFiles([]); setSelected(0); setValue(""); setSaved(""); setMessage(`Очищено элементов: ${count}.`); } catch (cause) { setMessage(safeError(cause)); } };
  const remove = async () => { if (!active || !window.confirm(`Удалить «${active.path}»?`)) return; try { await deleteWorkspaceCatalogEntry(workspaceId, kind, active.path); const next = files.filter((file) => file.path !== active.path); setFiles(next); setSelected(0); setValue(next[0]?.content ?? ""); setSaved(next[0]?.content ?? ""); setMessage("Элемент удалён из рабочего пространства."); } catch (cause) { setMessage(safeError(cause)); } };
  const share = async () => { if (!active) return; const target = workspaces.find((item) => item.id !== workspaceId); if (!target) { setMessage("Сначала создайте ещё одно рабочее пространство."); return; } try { await copyCatalogEntry(workspaceId, target.id, kind, active.path); setMessage(`Копия передана в «${target.name}».`); } catch (cause) { setMessage(safeError(cause)); } };
  return <section className="source-editor"><header className="workspace-header"><h2>Skills и rules</h2><div className="header-actions"><button className="button secondary" disabled={!active} onClick={() => void share()}>Передать</button><button className="button secondary" disabled={!active} onClick={() => void remove()}>Удалить</button><button className="button secondary" disabled={!files.length} onClick={() => void clear()}>Очистить все</button><button className="button secondary" disabled={!dirty} onClick={() => setValue(saved)}>Отменить</button><button className="button primary" disabled={!dirty || !active} onClick={() => void save()}>Сохранить</button></div></header><p className="intro">Рабочее пространство определяет независимый набор {kind}.</p><label className="catalog-workspace">Рабочее пространство<select value={workspaceId} onChange={(event) => setWorkspaceId(event.target.value)}>{workspaces.map((workspace) => <option key={workspace.id} value={workspace.id}>{workspace.name}</option>)}</select></label><div className="source-layout"><aside className="surface source-files"><h2>{kind === "skills" ? "Skills" : "Rules"}</h2>{files.length === 0 ? <p className="muted">Нет разрешённых файлов.</p> : files.map((file, index) => <button key={file.path} className={selected === index ? "source-file active" : "source-file"} onClick={() => select(index)}>{file.path.replace(`${kind}/`, "")}</button>)}</aside><article className="surface source-text"><strong className={dirty ? "unsaved" : "saved"}>{dirty ? "● Несохранённые изменения" : "● Все изменения сохранены"}</strong>{active ? <textarea aria-label={`${kind}: ${active.path}`} value={value} onChange={(event) => setValue(event.target.value)} /> : <p className="muted">Выберите или импортируйте файл.</p>}</article></div>{message && <p className="status-message">{message}</p>}</section>;
}

function ImportConfigurator() {
  const [analysis, setAnalysis] = useState<ExternalAnalysis | null>(null);
  const [mode, setMode] = useState<ImportMode>("add");
  const [preview, setPreview] = useState<ImportPreview | null>(null);
  const [message, setMessage] = useState<string | null>(null);
  const analyse = async () => { setMessage(null); setPreview(null); const path = await chooseConfigurationFolder(); if (!path) return; try { setAnalysis(await analyseExternalConfiguration(path)); } catch (cause) { setMessage(safeError(cause)); } };
  const buildPreview = async () => { if (!analysis) return; try { setPreview(await previewExternalImport(analysis.id, mode)); setMessage(null); } catch (cause) { setMessage(safeError(cause)); } };
  const apply = async () => { if (!analysis || !preview) return; if (mode === "replace" && !window.confirm("Заменить активные правила и skills? Предыдущий набор будет сохранён в резервной копии.")) return; try { await applyExternalImport(analysis.id, preview.id, mode === "replace"); setMessage("Импорт применён. Откройте Skills или Rules, чтобы увидеть активный набор."); setPreview(null); } catch (cause) { setMessage(safeError(cause)); } };
  const skillCount = analysis?.entries.filter((entry) => entry.kind === "skills").length ?? 0;
  const ruleCount = analysis?.entries.filter((entry) => entry.kind === "rules").length ?? 0;
  return <section className="surface import-panel"><header className="workspace-header"><div><h2>Анализ внешней папки</h2><p className="muted">Подключите переносимые rules и skills из другого проекта.</p></div><button className="button secondary" onClick={() => void analyse()}>Анализировать папку</button></header>{analysis && <><p className="import-path">{analysis.sourcePath}</p><div className="import-counts"><strong>{ruleCount} rules</strong><strong>{skillCount} skills</strong><span className="muted">Исключено защищённых или технических элементов: {analysis.excludedCount}</span><span className="muted">Не распознано как skill или rule: {analysis.unclassifiedCount}</span></div><div className="import-entries">{analysis.entries.map((entry) => <span key={entry.path} title={entry.kind}>{entry.kind === "skills" ? "skill" : "rule"} · {entry.path}</span>)}</div><fieldset className="import-mode"><legend>Как применить результат?</legend><label><input type="radio" checked={mode === "add"} onChange={() => { setMode("add"); setPreview(null); }} /> Добавить новые, оставить текущие при конфликтах</label><label><input type="radio" checked={mode === "replace"} onChange={() => { setMode("replace"); setPreview(null); }} /> Заменить управляемые правила и skills найденным набором</label></fieldset><button className="button primary" onClick={() => void buildPreview()}>Показать preview импорта</button></>}{preview && <div className="preview"><strong>Preview импорта</strong>{preview.operations.map((operation) => <p key={operation.path}><b>{operation.kind}</b> · {operation.path} <span className="muted">— {operation.reason}</span></p>)}<button className="button primary" disabled={!preview.canApply} onClick={() => void apply()}>Применить импорт</button></div>}{message && <p className="status-message">{message}</p>}</section>;
}

function TaskWorkspace() {
  const [kind, setKind] = useState<EntityKind>("task");
  const [workspaces, setWorkspaces] = useState<WorkspaceRecord[]>([]);
  const [workspaceId, setWorkspaceId] = useState("default");
  const [folders, setFolders] = useState<Folder[]>([]);
  const [folderId, setFolderId] = useState<string | null>(null);
  const [items, setItems] = useState<Entity[]>([]);
  const [blocks, setBlocks] = useState<Entity[]>([]);
  const [editing, setEditing] = useState<Entity | null>(null);
  const [shareItem, setShareItem] = useState<Entity | null>(null);
  const [shareTarget, setShareTarget] = useState("");
  const [name, setName] = useState(""); const [description, setDescription] = useState(""); const [content, setContent] = useState("");
  const [message, setMessage] = useState<string | null>(null);
  const editor = useRef<HTMLTextAreaElement>(null);

  const load = useCallback(async (nextKind = kind, nextWorkspace = workspaceId, nextFolder = folderId) => { try { setItems(await listWorkspaceEntities(nextKind, nextWorkspace, nextFolder)); if (nextKind === "task") setBlocks(await listWorkspaceEntities("taskBlock", nextWorkspace, nextFolder)); } catch (cause) { setMessage(safeError(cause)); } }, [kind, workspaceId, folderId]);
  const loadNavigation = useCallback(async (nextWorkspace = workspaceId) => { try { setWorkspaces(await listWorkspaces()); setFolders(await listFolders(nextWorkspace)); } catch (cause) { setMessage(safeError(cause)); } }, [workspaceId]);
  useEffect(() => { void load(); }, [load]);
  useEffect(() => { void loadNavigation(); }, [loadNavigation]);
  const label = kind === "task" ? "Задачи" : kind === "template" ? "Шаблоны" : "Блоки задач";
  const newItem = () => { setEditing({ id: "", name: "", description: null, content: "", sourceTemplateId: null, blockSnapshot: null, createdAt: 0, updatedAt: 0, revision: 0, workspaceId, folderId }); setName(""); setDescription(""); setContent(""); setMessage(null); };
  const editItem = (item: Entity) => { setEditing(item); setName(item.name); setDescription(item.description ?? ""); setContent(item.content); setMessage(null); };
  const save = async () => { if (!name.trim()) return; try { const saved = await saveEntity(kind, { id: editing?.id || undefined, name, description, content, workspaceId, folderId }); setEditing(kind === "task" ? saved : null); await load(); setMessage("Сохранено в рабочем пространстве."); } catch (cause) { setMessage(safeError(cause)); } };
  const changeWorkspace = (nextWorkspace: string) => { setWorkspaceId(nextWorkspace); setFolderId(null); setEditing(null); void loadNavigation(nextWorkspace); void load(kind, nextWorkspace, null); };
  const createWorkspace = async () => { const name = window.prompt("Название рабочего пространства"); if (!name?.trim()) return; try { const created = await saveWorkspace({ name }); setWorkspaceId(created.id); setFolderId(null); await loadNavigation(created.id); await load(kind, created.id, null); } catch (cause) { setMessage(safeError(cause)); } };
  const createFolder = async () => { const name = window.prompt("Название папки"); if (!name?.trim()) return; try { await saveFolder({ workspaceId, parentId: folderId, name }); await loadNavigation(); } catch (cause) { setMessage(safeError(cause)); } };
  const removeWorkspace = async () => { if (workspaceId === "default") { setMessage("Рабочее пространство по умолчанию удалить нельзя."); return; } const name = workspaces.find((item) => item.id === workspaceId)?.name ?? "рабочее пространство"; if (!window.confirm(`Удалить «${name}» вместе со всеми вложенными элементами? Это действие нельзя отменить.`)) return; try { await deleteWorkspace(workspaceId); setWorkspaceId("default"); setFolderId(null); await loadNavigation("default"); await load(kind, "default", null); } catch (cause) { setMessage(safeError(cause)); } };
  const removeFolder = async () => { if (!folderId) return; const name = folders.find((item) => item.id === folderId)?.name ?? "папку"; if (!window.confirm(`Удалить папку «${name}» со всем вложенным содержимым? Это действие нельзя отменить.`)) return; try { await deleteFolder(folderId); setFolderId(null); await loadNavigation(); await load(kind, workspaceId, null); } catch (cause) { setMessage(safeError(cause)); } };
  const share = async () => { if (!shareItem || !shareTarget) return; try { await copyEntity(kind, shareItem.id, shareTarget); setShareItem(null); setShareTarget(""); setMessage("Независимая копия создана в выбранном рабочем пространстве."); } catch (cause) { setMessage(safeError(cause)); } };
  const remove = async (item: Entity) => { if (!window.confirm(`Удалить «${item.name}»?`)) return; try { await deleteEntity(kind, item.id); await load(); } catch (cause) { setMessage(safeError(cause)); } };
  const clear = async () => { if (!items.length || !window.confirm(`Очистить все: ${label}? Это действие удалит ${items.length} локальных записей и не затронет файлы проекта.`)) return; try { const count = await clearEntities(kind); await load(); setMessage(`Удалено локальных записей: ${count}.`); } catch (cause) { setMessage(safeError(cause)); } };
  const insertBlock = (block: Entity) => { const field = editor.current; const start = field?.selectionStart ?? content.length; const end = field?.selectionEnd ?? start; const next = `${content.slice(0, start)}${block.content}${content.slice(end)}`; setContent(next); requestAnimationFrame(() => { field?.focus(); field?.setSelectionRange(start + block.content.length, start + block.content.length); }); };
  const createTemplate = async () => { if (!editing?.id || !name.trim()) return; try { await saveTaskAsTemplate(editing.id, `${name} — шаблон`, description || null); setMessage("Шаблон сохранён как независимая копия."); } catch (cause) { setMessage(safeError(cause)); } };
  const workspaceNavigation = <section className="workspace-navigation surface"><div className="workspace-switcher"><label>Рабочее пространство<select value={workspaceId} onChange={(event) => changeWorkspace(event.target.value)}>{workspaces.map((workspace) => <option key={workspace.id} value={workspace.id}>{workspace.name}</option>)}</select></label><button className="button secondary" onClick={() => void createWorkspace()}>Создать пространство</button><button className="link-button danger" disabled={workspaceId === "default"} onClick={() => void removeWorkspace()}>Удалить пространство</button></div><div className="folder-bar"><span>Папка: {folderId ? folders.find((folder) => folder.id === folderId)?.name ?? "Выбранная папка" : "Корень"}</span><button className="link-button" onClick={() => { setFolderId(null); void load(kind, workspaceId, null); }}>Корень</button><button className="link-button" onClick={() => void createFolder()}>+ Папка</button>{folderId && <button className="link-button danger" onClick={() => void removeFolder()}>Удалить папку</button>}</div>{folders.length > 0 && <FolderTree folders={folders} selectedId={folderId} onOpen={(id) => { setFolderId(id); void load(kind, workspaceId, id); }} />}</section>;

  if (editing) return <>{workspaceNavigation}<div className="tabs"><Segment label="Задачи" active={kind === "task"} onClick={() => setKind("task")} /><Segment label="Шаблоны" active={kind === "template"} onClick={() => setKind("template")} /><Segment label="Блоки задач" active={kind === "taskBlock"} onClick={() => setKind("taskBlock")} /></div><header className="workspace-header task-header"><h2>{editing.id ? `Редактирование: ${label.slice(0, -1)}` : `Новый ${kind === "task" ? "задача" : kind === "template" ? "шаблон" : "блок задачи"}`}</h2><div className="header-actions">{kind === "task" && editing.id && <button className="button secondary" onClick={() => void createTemplate()}>Сохранить как шаблон</button>}<button className="button primary" disabled={!name.trim()} onClick={() => void save()}>Сохранить</button></div></header><div className={kind === "task" ? "task-layout" : "form-layout"}><article className="surface form"><label>Название *<input value={name} onChange={(event) => setName(event.target.value)} /></label><label>Описание <input value={description} onChange={(event) => setDescription(event.target.value)} /></label><label>{kind === "task" ? "Текст задачи" : "Текст"}<textarea ref={kind === "task" ? editor : undefined} value={content} onChange={(event) => setContent(event.target.value)} placeholder={kind === "task" ? "Опишите задачу…" : "Текст для вставки…"} /></label>{kind === "task" && <p className="muted">Нажмите блок справа — его текст вставится в позицию курсора.</p>}</article>{kind === "task" && <aside className="surface blocks"><h2>Вставить в курсор</h2><p className="muted">Клик по карточке вставит текст в позицию курсора.</p>{blocks.map((block) => <button key={block.id} className="mini-card" onClick={() => insertBlock(block)}><b>{block.name}</b><span>{block.description ?? "Без описания"}</span></button>)}<button className="link-button" onClick={() => { setKind("taskBlock"); newItem(); }}>+ Создать блок</button></aside>}</div>{message && <p className="status-message">{message}</p>}</>;
  return <>{workspaceNavigation}<header className="workspace-header"><h2>{label}</h2><div className="header-actions"><button className="button secondary" disabled={!items.length} onClick={() => void clear()}>Очистить все</button><button className="button primary" onClick={newItem}>Создать {kind === "task" ? "задачу" : kind === "template" ? "шаблон" : "блок"}</button></div></header><div className="tabs"><Segment label="Задачи" active={kind === "task"} onClick={() => setKind("task")} /><Segment label="Шаблоны" active={kind === "template"} onClick={() => setKind("template")} /><Segment label="Блоки задач" active={kind === "taskBlock"} onClick={() => setKind("taskBlock")} /></div><section className="entity-list">{items.length === 0 ? <p className="muted">Пока нет записей. Создайте первую.</p> : items.map((item) => <article className="surface entity-card" key={item.id}><div><b>{item.name}</b><p className="muted">{item.description ?? "Без описания"}</p></div><div className="card-actions"><button className="link-button" onClick={() => editItem(item)}>Редактировать</button><button className="link-button" onClick={() => { setShareItem(item); setShareTarget(workspaces.find((workspace) => workspace.id !== workspaceId)?.id ?? ""); }}>Передать</button><button className="link-button danger" onClick={() => void remove(item)}>Удалить</button></div></article>)}</section>{shareItem && <div className="dialog-backdrop" role="presentation"><section className="surface share-dialog" role="dialog" aria-modal="true" aria-label="Передать копию"><h2>Передать копию</h2><p className="muted">«{shareItem.name}» останется в текущем пространстве. В целевом будет создана независимая копия.</p><label>Рабочее пространство<select value={shareTarget} onChange={(event) => setShareTarget(event.target.value)}>{workspaces.filter((workspace) => workspace.id !== workspaceId).map((workspace) => <option key={workspace.id} value={workspace.id}>{workspace.name}</option>)}</select></label><div className="header-actions"><button className="button secondary" onClick={() => setShareItem(null)}>Отмена</button><button className="button primary" disabled={!shareTarget} onClick={() => void share()}>Создать копию</button></div></section></div>}{message && <p className="status-message">{message}</p>}</>;
}

function Segment({ label, active, onClick }: { label: string; active: boolean; onClick: () => void }) { return <button className={active ? "tab active" : "tab"} onClick={onClick}>{label}</button>; }

function FolderTree({ folders, selectedId, onOpen }: { folders: Folder[]; selectedId: string | null; onOpen: (id: string) => void }) {
  const [expanded, setExpanded] = useState<string[]>([]);
  const children = (parentId: string | null) => folders.filter((folder) => folder.parentId === parentId);
  const branch = (parentId: string | null, depth = 0): ReactNode => children(parentId).map((folder) => {
    const hasChildren = children(folder.id).length > 0; const isExpanded = expanded.includes(folder.id);
    return <div className="folder-branch" style={{ marginLeft: depth * 16 }} key={folder.id}><div className="folder-row">{hasChildren ? <button className="folder-toggle" aria-label={isExpanded ? `Свернуть ${folder.name}` : `Раскрыть ${folder.name}`} onClick={() => setExpanded((all) => isExpanded ? all.filter((id) => id !== folder.id) : [...all, folder.id])}>{isExpanded ? "⌄" : "›"}</button> : <span className="folder-toggle placeholder">·</span>}<button className={folder.id === selectedId ? "folder-node active" : "folder-node"} onClick={() => onOpen(folder.id)}>{folder.name}</button></div>{isExpanded && branch(folder.id, depth + 1)}</div>;
  });
  return <div className="folder-tree" aria-label="Дерево папок">{branch(null)}</div>;
}

function safeError(cause: unknown) { return cause instanceof Error ? cause.message.replaceAll(/\.env|token|cookie|private key/gi, "защищённый файл") : "Не удалось выполнить операцию."; }
