// @vitest-environment jsdom
import { cleanup, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { afterEach, describe, expect, it, vi } from "vitest";

vi.mock("./native", () => ({
  listenInstallationProgress: vi.fn().mockResolvedValue(() => undefined), chooseTargetFolder: vi.fn(), previewInstallation: vi.fn(), applyInstallation: vi.fn(),
  chooseConfigurationFolder: vi.fn().mockResolvedValue("/allowed"),
  analyseExternalConfiguration: vi.fn().mockResolvedValue({ id: "analysis", sourcePath: "/allowed", entries: [{ kind: "rules", path: "rules/core.md", content: "# Rule", hash: "x" }], excludedCount: 1, unclassifiedCount: 0 }),
  previewExternalImport: vi.fn().mockResolvedValue({ id: "preview", mode: "add", canApply: true, operations: [{ kind: "create", path: "rules/core.md", reason: "Будет добавлен" }] }), applyExternalImport: vi.fn().mockResolvedValue(undefined),
  listWorkspaces: vi.fn().mockResolvedValue([{ id: "default", name: "Моё рабочее пространство" }]),
  listWorkspaceCatalogEntries: vi.fn((_: string, kind: string) => Promise.resolve(kind === "rules" ? [{ kind, path: "rules/core.md", content: "# Rule", hash: "x" }] : [])),
  saveWorkspaceCatalogEntry: vi.fn(), clearWorkspaceCatalogEntries: vi.fn(), deleteWorkspaceCatalogEntry: vi.fn(), copyCatalogEntry: vi.fn(), clearAllLocalData: vi.fn(), clearEntities: vi.fn(), copyEntity: vi.fn(), deleteEntity: vi.fn(), deleteFolder: vi.fn(), deleteWorkspace: vi.fn(), listFolders: vi.fn().mockResolvedValue([]), listWorkspaceEntities: vi.fn().mockResolvedValue([]), saveEntity: vi.fn().mockResolvedValue({ id: "task-1", name: "Новая задача", description: "", content: "", workspaceId: "default", folderId: null }), saveTaskAsTemplate: vi.fn(), saveFolder: vi.fn(), saveWorkspace: vi.fn(),
}));

import { App } from "./App";
import { applyInstallation, chooseTargetFolder, listWorkspaceEntities, previewInstallation, saveEntity, saveTaskAsTemplate, saveWorkspaceCatalogEntry } from "./native";

afterEach(() => cleanup());

describe("Environment UI", () => {
  it("безопасно переключает Skills и Rules с разным количеством файлов", async () => {
    const user = userEvent.setup(); render(<App />);
    await user.click(screen.getByRole("tab", { name: "Skills" }));
    expect((await screen.findByText("Нет разрешённых файлов.")).textContent).toBe("Нет разрешённых файлов.");
    await user.click(screen.getByRole("tab", { name: "Rules" }));
    const editor = await screen.findByRole("textbox", { name: "rules: rules/core.md" });
    expect((editor as HTMLTextAreaElement).value).toBe("# Rule");
  });

  it("показывает analyse, preview и apply импортирования", async () => {
    const user = userEvent.setup(); render(<App />);
    await user.click(screen.getByRole("button", { name: "Анализировать папку" }));
    expect((await screen.findByText("1 rules")).textContent).toBe("1 rules");
    await user.click(screen.getByRole("button", { name: "Показать preview импорта" }));
    await screen.findByText("Preview импорта");
    await user.click(screen.getByRole("button", { name: "Применить импорт" }));
    await waitFor(() => expect(screen.getByText(/Импорт применён/).textContent).toContain("Импорт применён"));
  });

  it("сохраняет редактор только по explicit save и умеет отменять правки", async () => {
    const user = userEvent.setup(); render(<App />);
    await user.click(screen.getByRole("tab", { name: "Rules" }));
    const editor = await screen.findByRole("textbox", { name: "rules: rules/core.md" });
    await user.type(editor, "\nEdited");
    expect(saveWorkspaceCatalogEntry).not.toHaveBeenCalled();
    await user.click(screen.getByRole("button", { name: "Отменить" }));
    expect((editor as HTMLTextAreaElement).value).toBe("# Rule");
    await user.type(editor, "\nSaved");
    await user.click(screen.getByRole("button", { name: "Сохранить" }));
    await waitFor(() => expect(saveWorkspaceCatalogEntry).toHaveBeenCalledWith("default", "rules", "rules/core.md", "# Rule\nSaved"));
  });

  it("активирует применение профиля только после preview выбранной папки", async () => {
    vi.mocked(chooseTargetFolder).mockResolvedValueOnce("/project");
    vi.mocked(previewInstallation).mockResolvedValueOnce({ targetPath: "/project", profile: "standard", canApply: true, operations: [{ kind: "create", path: "AGENTS.md", reason: "Будет создан" }] });
    const user = userEvent.setup(); render(<App />);
    const apply = screen.getByRole("button", { name: "Применить профиль" }) as HTMLButtonElement;
    expect(apply.disabled).toBe(true);
    await user.click(screen.getByRole("button", { name: "Выбрать папку" }));
    await screen.findByText("Preview установки");
    await waitFor(() => expect(apply.disabled).toBe(false));
    await user.click(apply);
    await waitFor(() => expect(applyInstallation).toHaveBeenCalledWith("/project"));
  });

  it("создаёт Task только с обязательным именем", async () => {
    const user = userEvent.setup(); render(<App />);
    await user.click(screen.getByRole("button", { name: "Задачи и шаблоны" }));
    await user.click(await screen.findByRole("button", { name: "Создать задачу" }));
    const save = screen.getByRole("button", { name: "Сохранить" }) as HTMLButtonElement;
    expect(save.disabled).toBe(true);
    const inputs = screen.getAllByRole("textbox");
    await user.type(inputs[0], "Новая задача");
    await user.type(inputs[1], "Опционально");
    expect(save.disabled).toBe(false);
    await user.click(save);
    await waitFor(() => expect(saveEntity).toHaveBeenCalledWith("task", expect.objectContaining({ name: "Новая задача", description: "Опционально", workspaceId: "default" })));
  });

  it("вставляет TaskBlock в позицию курсора редактора задачи", async () => {
    vi.mocked(listWorkspaceEntities).mockImplementation((kind) => Promise.resolve(kind === "taskBlock" ? [{ id: "block-1", name: "Контекст", description: "Блок", content: "[context]", sourceTemplateId: null, blockSnapshot: null, createdAt: 0, updatedAt: 0, revision: 1, workspaceId: "default", folderId: null }] : []));
    const user = userEvent.setup(); render(<App />);
    await user.click(screen.getByRole("button", { name: "Задачи и шаблоны" }));
    await user.click(await screen.findByRole("button", { name: "Создать задачу" }));
    const editor = screen.getByPlaceholderText("Опишите задачу…") as HTMLTextAreaElement;
    await user.type(editor, "Начало Конец");
    editor.setSelectionRange(7, 7);
    await user.click(await screen.findByRole("button", { name: /Контекст/ }));
    await waitFor(() => expect(editor.value).toBe("Начало [context]Конец"));
  });

  it("сохраняет сохранённую задачу как независимый шаблон", async () => {
    const user = userEvent.setup(); render(<App />);
    await user.click(screen.getByRole("button", { name: "Задачи и шаблоны" }));
    await user.click(await screen.findByRole("button", { name: "Создать задачу" }));
    await user.type(screen.getAllByRole("textbox")[0], "План релиза");
    await user.click(screen.getByRole("button", { name: "Сохранить" }));
    await user.click(await screen.findByRole("button", { name: "Сохранить как шаблон" }));
    await waitFor(() => expect(saveTaskAsTemplate).toHaveBeenCalledWith("task-1", "План релиза — шаблон", null));
  });
});
