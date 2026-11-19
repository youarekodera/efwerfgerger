use std::{collections::HashSet, path::PathBuf};

use gpui::{
    App, AsyncApp, ClickEvent, Context, Div, FocusHandle, Focusable, PathPromptOptions, Render,
    SharedString, Stateful, WeakEntity, Window, actions, div, prelude::*, px,
};

use crate::{
    editor::TextDocument,
    filesystem::{EntryKind, FileRow, ProjectTree},
    ui::theme::Theme,
};

actions!(workspace, [OpenFolder, RefreshExplorer, ToggleBottomPanel]);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BottomPanel {
    Terminal,
    Build,
    Errors,
    Logs,
}

impl BottomPanel {
    fn label(self) -> &'static str {
        match self {
            Self::Terminal => "Terminal",
            Self::Build => "Compilación",
            Self::Errors => "Errores",
            Self::Logs => "Registros",
        }
    }
}

pub struct Workspace {
    focus_handle: FocusHandle,
    project: Option<ProjectTree>,
    expanded_dirs: HashSet<PathBuf>,
    explorer_loading: bool,
    explorer_error: Option<String>,
    open_tabs: Vec<TextDocument>,
    loading_files: HashSet<PathBuf>,
    active_tab: Option<usize>,
    bottom_panel: BottomPanel,
    bottom_panel_visible: bool,
    logs: Vec<String>,
}

impl Workspace {
    pub(crate) fn new(focus_handle: FocusHandle) -> Self {
        Self {
            focus_handle,
            project: None,
            expanded_dirs: HashSet::new(),
            explorer_loading: false,
            explorer_error: None,
            open_tabs: Vec::new(),
            loading_files: HashSet::new(),
            active_tab: None,
            bottom_panel: BottomPanel::Logs,
            bottom_panel_visible: false,
            logs: Vec::new(),
        }
    }

    fn prompt_for_project(&mut self, cx: &mut Context<Self>) {
        let receiver = cx.prompt_for_paths(PathPromptOptions {
            files: false,
            directories: true,
            multiple: false,
            prompt: Some("Selecciona una carpeta".into()),
        });

        cx.spawn(
            async move |this: WeakEntity<Self>, cx: &mut AsyncApp| match receiver.await {
                Ok(Ok(Some(paths))) => {
                    if let Some(path) = paths.into_iter().next() {
                        let _ = this.update(cx, |this, cx| this.open_project(path, cx));
                    }
                }
                Ok(Ok(None)) => {}
                Ok(Err(error)) => {
                    let _ = this.update(cx, |this, cx| {
                        this.push_log(format!("No se pudo seleccionar la carpeta: {error}"));
                        cx.notify();
                    });
                }
                Err(_) => {}
            },
        )
        .detach();
    }

    fn open_project(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        self.load_project(path, cx, true);
    }

    fn load_project(&mut self, path: PathBuf, cx: &mut Context<Self>, reset_workspace: bool) {
        if reset_workspace {
            self.project = None;
            self.open_tabs.clear();
            self.active_tab = None;
            self.expanded_dirs.clear();
        }

        self.explorer_loading = true;
        self.explorer_error = None;
        self.push_log(format!("Escaneando proyecto: {}", path.display()));
        cx.notify();

        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let result = cx
                .background_spawn(async move { ProjectTree::scan(path) })
                .await;

            let _ = this.update(cx, |this, cx| {
                this.explorer_loading = false;
                match result {
                    Ok(tree) => {
                        let root = tree.root.clone();
                        this.expanded_dirs.clear();
                        this.expanded_dirs.insert(root.clone());
                        this.project = Some(tree);
                        this.explorer_error = None;
                        this.push_log(format!("Proyecto cargado: {}", root.display()));
                    }
                    Err(error) => {
                        this.explorer_error = Some(error.to_string());
                        this.push_log(format!("Error al escanear proyecto: {error}"));
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn refresh_explorer(&mut self, cx: &mut Context<Self>) {
        if let Some(root) = self.project.as_ref().map(|project| project.root.clone()) {
            self.load_project(root, cx, false);
        } else {
            self.prompt_for_project(cx);
        }
    }

    fn toggle_bottom_panel(&mut self, cx: &mut Context<Self>) {
        self.bottom_panel_visible = !self.bottom_panel_visible;
        cx.notify();
    }

    fn select_bottom_panel(&mut self, panel: BottomPanel, cx: &mut Context<Self>) {
        self.bottom_panel = panel;
        self.bottom_panel_visible = true;
        cx.notify();
    }

    fn handle_explorer_click(&mut self, path: PathBuf, kind: EntryKind, cx: &mut Context<Self>) {
        match kind {
            EntryKind::Directory => self.toggle_dir(path, cx),
            EntryKind::File | EntryKind::Symlink => self.open_file(path, cx),
            EntryKind::Other => {
                self.push_log(format!("No se puede abrir: {}", path.display()));
                cx.notify();
            }
        }
    }

    fn toggle_dir(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        if !self.expanded_dirs.remove(&path) {
            self.expanded_dirs.insert(path);
        }
        cx.notify();
    }

    fn open_file(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        if let Some(index) = self
            .open_tabs
            .iter()
            .position(|document| document.path == path)
        {
            self.active_tab = Some(index);
            cx.notify();
            return;
        }
        if !self.loading_files.insert(path.clone()) {
            return;
        }
        self.push_log(format!("Cargando: {}", path.display()));
        cx.notify();
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let load_path = path.clone();
            let result = cx
                .background_spawn(async move { TextDocument::load(&load_path) })
                .await;
            let _ = this.update(cx, |this, cx| {
                this.loading_files.remove(&path);
                match result {
                    Ok(document) => {
                        this.open_tabs.push(document);
                        this.active_tab = Some(this.open_tabs.len() - 1);
                        this.push_log(format!("Abierto: {}", path.display()));
                    }
                    Err(error) => {
                        this.push_log(format!("No se pudo abrir {}: {error}", path.display()))
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    fn select_tab(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.open_tabs.len() {
            self.active_tab = Some(index);
            cx.notify();
        }
    }

    fn close_tab(&mut self, index: usize, cx: &mut Context<Self>) {
        if index >= self.open_tabs.len() {
            return;
        }

        let closed = self.open_tabs.remove(index);
        self.push_log(format!("Pestaña cerrada: {}", closed.file_name()));

        if self.open_tabs.is_empty() {
            self.active_tab = None;
        } else {
            self.active_tab = Some(index.min(self.open_tabs.len() - 1));
        }
        cx.notify();
    }

    fn push_log(&mut self, message: String) {
        self.logs.push(message);
        if self.logs.len() > 200 {
            let excess = self.logs.len() - 200;
            self.logs.drain(..excess);
        }
    }

    fn project_label(&self) -> String {
        self.project
            .as_ref()
            .and_then(|project| {
                project
                    .root
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
            })
            .unwrap_or_else(|| "Sin proyecto".to_string())
    }

    fn open_folder_action(&mut self, _: &OpenFolder, _window: &mut Window, cx: &mut Context<Self>) {
        self.prompt_for_project(cx);
    }

    fn refresh_explorer_action(
        &mut self,
        _: &RefreshExplorer,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.refresh_explorer(cx);
    }

    fn toggle_bottom_panel_action(
        &mut self,
        _: &ToggleBottomPanel,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.toggle_bottom_panel(cx);
    }

    fn open_folder_click(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.prompt_for_project(cx);
    }

    fn refresh_click(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.refresh_explorer(cx);
    }

    fn toggle_bottom_panel_click(
        &mut self,
        _: &ClickEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.toggle_bottom_panel(cx);
    }

    fn render_top_bar(&self, theme: Theme, cx: &mut Context<Self>) -> Div {
        div()
            .h(px(36.))
            .flex_none()
            .flex()
            .items_center()
            .justify_between()
            .px_3()
            .bg(theme.surface)
            .border_b_1()
            .border_color(theme.border)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .child(
                        div()
                            .font_family("monospace")
                            .text_sm()
                            .text_color(theme.accent)
                            .child("Code Time"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.text_muted)
                            .child(self.project_label()),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        toolbar_button("open-folder", "Abrir carpeta", theme, false)
                            .on_click(cx.listener(Self::open_folder_click)),
                    )
                    .child(
                        toolbar_button("refresh-explorer", "Actualizar", theme, false)
                            .on_click(cx.listener(Self::refresh_click)),
                    )
                    .child(
                        toolbar_button(
                            "toggle-bottom-panel",
                            "Panel",
                            theme,
                            self.bottom_panel_visible,
                        )
                        .on_click(cx.listener(Self::toggle_bottom_panel_click)),
                    ),
            )
    }

    fn render_sidebar(&self, theme: Theme, cx: &mut Context<Self>) -> Div {
        let mut sidebar = div()
            .w(px(260.))
            .flex_none()
            .flex()
            .flex_col()
            .bg(theme.sidebar)
            .border_r_1()
            .border_color(theme.border)
            .child(
                div()
                    .h(px(34.))
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_2()
                    .border_b_1()
                    .border_color(theme.border)
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child("EXPLORADOR"),
                    )
                    .child(
                        div()
                            .id("refresh-explorer-sidebar")
                            .px_2()
                            .rounded_sm()
                            .cursor_pointer()
                            .text_color(theme.text_muted)
                            .hover(|this| this.bg(theme.hover).text_color(theme.text))
                            .child("↻")
                            .on_click(cx.listener(Self::refresh_click)),
                    ),
            );

        if self.explorer_loading {
            sidebar = sidebar.child(
                div()
                    .p_3()
                    .text_sm()
                    .text_color(theme.text_muted)
                    .child("Escaneando proyecto…"),
            );
        }

        if let Some(error) = &self.explorer_error {
            sidebar = sidebar.child(
                div()
                    .p_3()
                    .text_sm()
                    .text_color(theme.danger)
                    .child(error.clone()),
            );
        }

        if let Some(project) = &self.project {
            let rows = project.visible_rows(&self.expanded_dirs);
            let mut elements = Vec::with_capacity(rows.len() + 1);
            for row in rows {
                elements.push(self.render_file_row(row, theme, cx));
            }
            if project.truncated {
                elements.push(
                    div()
                        .id("explorer-truncated")
                        .px_2()
                        .py_1()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child("Vista truncada por tamaño del proyecto"),
                );
            }

            sidebar = sidebar.child(
                div()
                    .id("explorer-scroll")
                    .flex_1()
                    .min_h_0()
                    .overflow_scroll()
                    .py_1()
                    .children(elements),
            );
        } else if !self.explorer_loading {
            sidebar = sidebar.child(
                div()
                    .p_3()
                    .text_sm()
                    .text_color(theme.text_muted)
                    .child("Abre una carpeta para explorar archivos."),
            );
        }

        sidebar
    }

    fn render_file_row(
        &self,
        row: FileRow<'_>,
        theme: Theme,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let node = row.node;
        let path = node.path.clone();
        let kind = node.kind;
        let expanded = self.expanded_dirs.contains(&node.path);
        let indent = 8. + row.depth as f32 * 12.;
        let marker = if node.is_dir() {
            if expanded { "▾" } else { "▸" }
        } else {
            "·"
        };
        let text_color = match kind {
            EntryKind::Directory => theme.text,
            EntryKind::Symlink => theme.accent,
            EntryKind::File => theme.text,
            EntryKind::Other => theme.text_muted,
        };

        div()
            .id(SharedString::from(format!(
                "explorer-row-{}",
                node.path.display()
            )))
            .h(px(24.))
            .flex()
            .items_center()
            .gap_1()
            .pl(px(indent))
            .pr_2()
            .rounded_sm()
            .cursor_pointer()
            .hover(|this| this.bg(theme.hover))
            .child(
                div()
                    .w(px(14.))
                    .flex_none()
                    .text_xs()
                    .text_color(theme.text_muted)
                    .child(marker),
            )
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .overflow_hidden()
                    .whitespace_nowrap()
                    .text_sm()
                    .text_color(text_color)
                    .child(node.name.clone()),
            )
            .on_click(cx.listener(move |this, _, _, cx| {
                this.handle_explorer_click(path.clone(), kind, cx);
            }))
    }

    fn render_editor_area(&self, theme: Theme, cx: &mut Context<Self>) -> Div {
        let mut area = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .bg(theme.background);

        if self.open_tabs.is_empty() {
            return area.child(self.render_welcome(theme, cx));
        }

        let mut tab_bar = div()
            .id("tab-bar")
            .h(px(34.))
            .flex_none()
            .flex()
            .items_center()
            .overflow_scroll()
            .bg(theme.surface)
            .border_b_1()
            .border_color(theme.border);

        for (index, document) in self.open_tabs.iter().enumerate() {
            let active = self.active_tab == Some(index);
            let tab_background = if active {
                theme.tab_active
            } else {
                theme.tab_inactive
            };
            let tab_text = if active { theme.text } else { theme.text_muted };
            let close_id = SharedString::from(format!("close-tab-{}", index));
            let select_id = SharedString::from(format!("select-tab-{}", index));

            tab_bar = tab_bar.child(
                div()
                    .id(select_id)
                    .h_full()
                    .flex_none()
                    .flex()
                    .items_center()
                    .gap_1()
                    .px_2()
                    .bg(tab_background)
                    .border_r_1()
                    .border_color(theme.border)
                    .cursor_pointer()
                    .hover(|this| this.bg(theme.hover))
                    .child(
                        div()
                            .text_sm()
                            .text_color(tab_text)
                            .child(document.file_name()),
                    )
                    .child(
                        div()
                            .id(close_id)
                            .px_1()
                            .rounded_sm()
                            .text_sm()
                            .text_color(theme.text_muted)
                            .hover(|this| this.bg(theme.hover).text_color(theme.text))
                            .child("×")
                            .on_click(cx.listener(move |this, _, _, cx| {
                                cx.stop_propagation();
                                this.close_tab(index, cx);
                            })),
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.select_tab(index, cx);
                    })),
            );
        }

        area = area.child(tab_bar);

        if let Some(index) = self.active_tab {
            if let Some(document) = self.open_tabs.get(index) {
                area = area.child(
                    div()
                        .id("editor-scroll")
                        .flex_1()
                        .min_h_0()
                        .overflow_scroll()
                        .font_family("monospace")
                        .text_sm()
                        .children(Self::render_document_lines(document, theme)),
                );
            }
        }

        area
    }

    fn render_document_lines(document: &TextDocument, theme: Theme) -> Vec<Div> {
        let mut elements = Vec::new();
        const MAX_RENDERED_LINES: usize = 5_000;
        let mut rendered = 0usize;
        for (index, line) in document
            .content
            .lines()
            .take(MAX_RENDERED_LINES)
            .enumerate()
        {
            rendered = index + 1;
            elements.push(
                div()
                    .h(px(20.))
                    .flex()
                    .items_start()
                    .child(
                        div()
                            .w(px(44.))
                            .flex_none()
                            .pr_2()
                            .text_right()
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child((index + 1).to_string()),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .whitespace_nowrap()
                            .text_color(theme.text)
                            .child(line.to_string()),
                    ),
            );
        }

        if document.content.is_empty() {
            elements.push(div().h(px(20.)).child(" "));
        }
        if document.line_count() > rendered && rendered == MAX_RENDERED_LINES {
            elements.push(
                div()
                    .p_2()
                    .text_color(theme.text_muted)
                    .child("Vista limitada a 5.000 líneas para mantener el rendimiento."),
            );
        }

        elements
    }

    fn render_bottom_panel(&self, theme: Theme, cx: &mut Context<Self>) -> Div {
        let mut panel = div()
            .h(px(180.))
            .flex_none()
            .flex()
            .flex_col()
            .bg(theme.surface)
            .border_t_1()
            .border_color(theme.border);

        let mut tabs = div()
            .h(px(30.))
            .flex_none()
            .flex()
            .items_center()
            .px_2()
            .gap_1()
            .border_b_1()
            .border_color(theme.border);

        for panel_kind in [
            BottomPanel::Terminal,
            BottomPanel::Build,
            BottomPanel::Errors,
            BottomPanel::Logs,
        ] {
            let active = self.bottom_panel == panel_kind;
            tabs = tabs.child(
                div()
                    .id(SharedString::from(format!(
                        "bottom-tab-{}",
                        panel_kind.label()
                    )))
                    .px_2()
                    .py_1()
                    .rounded_sm()
                    .cursor_pointer()
                    .text_sm()
                    .text_color(if active { theme.text } else { theme.text_muted })
                    .bg(if active { theme.hover } else { theme.surface })
                    .hover(|this| this.bg(theme.hover))
                    .child(panel_kind.label())
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.select_bottom_panel(panel_kind, cx);
                    })),
            );
        }

        panel = panel.child(tabs);

        let content = match self.bottom_panel {
            BottomPanel::Terminal => div()
                .id("bottom-terminal-content")
                .flex_1()
                .min_h_0()
                .overflow_scroll()
                .p_3()
                .text_sm()
                .text_color(theme.text_muted)
                .child("La terminal integrada se implementará en la Fase 3."),
            BottomPanel::Build => div()
                .id("bottom-build-content")
                .flex_1()
                .min_h_0()
                .overflow_scroll()
                .p_3()
                .text_sm()
                .text_color(theme.text_muted)
                .child("No hay tareas de compilación configuradas todavía."),
            BottomPanel::Errors => div()
                .id("bottom-errors-content")
                .flex_1()
                .min_h_0()
                .overflow_scroll()
                .p_3()
                .text_sm()
                .text_color(theme.text_muted)
                .child("No hay diagnósticos todavía."),
            BottomPanel::Logs => {
                if self.logs.is_empty() {
                    div()
                        .id("logs-empty")
                        .flex_1()
                        .min_h_0()
                        .overflow_scroll()
                        .p_3()
                        .text_sm()
                        .text_color(theme.text_muted)
                        .child("Sin eventos registrados.")
                } else {
                    let mut log_elements = Vec::with_capacity(self.logs.len());
                    for (index, message) in self.logs.iter().enumerate() {
                        log_elements.push(
                            div()
                                .px_3()
                                .py_0p5()
                                .text_sm()
                                .text_color(theme.text)
                                .child(format!("{index}: {message}")),
                        );
                    }
                    div()
                        .id("logs-scroll")
                        .flex_1()
                        .min_h_0()
                        .overflow_scroll()
                        .children(log_elements)
                }
            }
        };

        panel.child(content)
    }

    fn render_status_bar(&self, theme: Theme) -> Div {
        let project_path = self
            .project
            .as_ref()
            .map(|project| project.root.display().to_string())
            .unwrap_or_else(|| "Sin proyecto".to_string());
        let active_file = self
            .active_tab
            .and_then(|index| self.open_tabs.get(index))
            .map(|document| document.path.display().to_string())
            .unwrap_or_else(|| "—".to_string());
        let document_info = self
            .active_tab
            .and_then(|index| self.open_tabs.get(index))
            .map(|document| {
                let mode = if document.read_only {
                    "Solo lectura"
                } else {
                    "Editable"
                };
                format!(
                    "{} · {mode} · {} líneas · Posición: —",
                    document.language_name(),
                    document.line_count()
                )
            })
            .unwrap_or_else(|| "—".to_string());

        div()
            .h(px(24.))
            .flex_none()
            .flex()
            .items_center()
            .justify_between()
            .px_3()
            .bg(theme.statusbar)
            .border_t_1()
            .border_color(theme.border)
            .text_xs()
            .text_color(theme.text_muted)
            .child(project_path)
            .child(active_file)
            .child(document_info)
    }

    fn render_welcome(&self, theme: Theme, cx: &mut Context<Self>) -> Div {
        div()
            .flex_1()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .child(div().text_2xl().text_color(theme.text).child("Code Time"))
            .child(
                div()
                    .text_sm()
                    .text_color(theme.text_muted)
                    .child("IDE nativo en Rust + GPUI"),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(theme.text_muted)
                    .child("Ctrl+O abrir carpeta · Ctrl+J panel · Ctrl+Shift+L tema"),
            )
            .child(
                toolbar_button("welcome-open-folder", "Abrir carpeta", theme, true)
                    .on_click(cx.listener(Self::open_folder_click)),
            )
    }
}

fn toolbar_button(id: &'static str, label: &str, theme: Theme, active: bool) -> Stateful<Div> {
    let background = if active { theme.hover } else { theme.surface };
    div()
        .id(id)
        .px_2()
        .py_1()
        .rounded_sm()
        .cursor_pointer()
        .text_sm()
        .text_color(theme.text)
        .bg(background)
        .border_1()
        .border_color(theme.border)
        .hover(|this| this.bg(theme.hover))
        .child(label.to_string())
}

impl Focusable for Workspace {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::dark();

        div()
            .id("workspace")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::open_folder_action))
            .on_action(cx.listener(Self::refresh_explorer_action))
            .on_action(cx.listener(Self::toggle_bottom_panel_action))
            .size_full()
            .flex()
            .flex_col()
            .bg(theme.background)
            .text_color(theme.text)
            .font_family("monospace")
            .child(self.render_top_bar(theme, cx))
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .flex()
                    .child(self.render_sidebar(theme, cx))
                    .child(self.render_editor_area(theme, cx)),
            )
            .when(self.bottom_panel_visible, |this| {
                this.child(self.render_bottom_panel(theme, cx))
            })
            .child(self.render_status_bar(theme))
    }
}
