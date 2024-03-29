use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;

use eframe::egui::{self, Button, TextEdit};
use egui_node_graph::{GraphEditorState, NodeResponse};
use log::{error, info, warn};
use strum::EnumCount;
use walkdir::WalkDir;

use crate::ui::{ComboBoxEnum, NewWindowPrompt};
use crate::window::{Window, WindowType};
use crate::{
    nodes::{
        data_types::{decrease_node_list_length, increase_node_list_length, DataType, ValueType},
        node_types::{AllNodeTemplates, NodeTemplate},
        rebuild_node, GraphState, NodeData, Response,
    },
    ui::OpenProjectPrompt,
};

pub type EditorStateType =
    GraphEditorState<NodeData, DataType, ValueType, NodeTemplate, GraphState>;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Identifier {
    window_type: WindowType,
    namespace: String,
    path: String,
}
impl Identifier {
    pub fn new(namespace: String, path: String, window_type: WindowType) -> Self {
        Self {
            namespace,
            path,
            window_type,
        }
    }
    pub fn from_string(s: String, window_type: WindowType) -> Option<Self> {
        if let Some((namespace, path)) = s.split_once(":") {
            Some(Self {
                namespace: String::from(namespace),
                path: String::from(path),
                window_type,
            })
        } else {
            None
        }
    }
}
impl ToString for Identifier {
    fn to_string(&self) -> String {
        format!("{}:{}", self.namespace, self.path)
    }
}

impl PartialOrd for Identifier {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.window_type.partial_cmp(&other.window_type) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.namespace.partial_cmp(&other.namespace) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.path.partial_cmp(&other.path)
    }
}

pub struct App<'a> {
    active_window: Option<&'a mut Window>,
    new_window_prompt: NewWindowPrompt,
    file_structure: [HashMap<Identifier, Window>; WindowType::COUNT],
    project_path: Option<PathBuf>,
    open_project_prompt: OpenProjectPrompt,
}
impl App<'_> {
    pub fn new<'a>(_cc: &eframe::CreationContext, project_path_option: Option<PathBuf>) -> Self {
        let mut map: [HashMap<Identifier, Window>; WindowType::COUNT] =
            [HashMap::new(), HashMap::new(), HashMap::new()];
        if let Some(project_path) = &project_path_option {
            Self::load_from_fs(project_path, &mut map);
        }
        let has_path = project_path_option.is_some();
        Self {
            file_structure: map,
            active_window: None,
            new_window_prompt: NewWindowPrompt::new(false),
            project_path: project_path_option,
            open_project_prompt: OpenProjectPrompt::new(!has_path),
        }
    }
    fn load_from_fs(project_path: &PathBuf, map: &mut [HashMap<Identifier, Window>]) {
        for entry in WalkDir::new(project_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let f_name = entry.file_name().to_string_lossy();
            if f_name.ends_with(".json") {
                if let Ok(file) = OpenOptions::new().read(true).write(true).open(entry.path()) {
                    let new_window =
                        Window::from_file(file, entry.path().to_path_buf(), &project_path);
                    match new_window {
                        Ok(mut window) => {
                            let mut buf = String::new();
                            if let Err(e) = window.file.as_ref().unwrap().read_to_string(&mut buf) {
                                warn!("{}", e);
                            };
                            if let Ok(json) = json::parse(&buf) {
                                window.deserialize(&json);
                            }
                            map[window.window_type as usize].insert(
                                Identifier::new(
                                    window.namespace.clone(),
                                    window.name.clone(),
                                    window.window_type,
                                ),
                                window,
                            );
                        }
                        Err(e) => error!("{}", e.to_string()),
                    }
                } else {
                    error!("fs error: {}", entry.path().display());
                }
            }
        }
    }
    fn serialize_all(&mut self) -> Result<(), ()> {
        for filetype_map in self.file_structure.iter_mut() {
            for window in filetype_map.values_mut() {
                info!("Serializing window {}", window);
                if let Some(json) = window.serialize() {
                    if let Err(e) = window.save_to_file(json.pretty(4)) {
                        error!(
                            "failed to save to file: {} ({})",
                            e,
                            window.filepath.display()
                        )
                    }
                } else {
                    error!("Window failed to serialize {}", window)
                }
            }
        }
        Ok(())
    }
}

impl eframe::App for App<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                if ui.button("save all").clicked() {
                    if let Err(_) = self.serialize_all() {
                        warn!("Something went wrong when saving.");
                    }
                }
            });
        });
        egui::SidePanel::left("outline").show(ctx, |ui| {
            ui.add_enabled_ui(self.project_path.is_some(), |ui| {
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::Min), |ui| {
                    ui.vertical_centered_justified(|ui| {
                        if ui.button("[ + ]").clicked() {
                            self.new_window_prompt.show = !self.new_window_prompt.show;
                        }
                    });
                    let mut should_open = self.new_window_prompt.show;
                    let inner_resp = egui::Window::new("New file")
                        // TODO: refactor into a method of `NewWindowPrompt` (see `OpenProjectPrompt`)
                        .collapsible(false)
                        //.default_pos()
                        .open(&mut should_open)
                        .show(ctx, |ui| {
                            egui::ComboBox::from_label("File Type")
                                .selected_text(self.new_window_prompt.window_type.as_ref())
                                .show_ui(ui, |ui| {
                                    WindowType::show_ui(
                                        ui,
                                        &mut self.new_window_prompt.window_type,
                                    );
                                });
                            //ui.horizontal_centered(|ui| {
                            ui.add(
                                TextEdit::singleline(&mut self.new_window_prompt.namespace)
                                    .hint_text("namespace"),
                            );
                            ui.label(":");
                            ui.add(
                                TextEdit::singleline(&mut self.new_window_prompt.name)
                                    .hint_text("path"),
                            );
                            //});
                            let enable = self.new_window_prompt.are_strings_correct();
                            if ui.add_enabled(enable, Button::new("Add file")).clicked() {
                                let win = self
                                    .new_window_prompt
                                    .make_window(&self.project_path.as_ref().unwrap());
                                self.file_structure[win.window_type as usize].insert(
                                    Identifier::new(
                                        win.namespace.clone(),
                                        win.name.clone(),
                                        win.window_type,
                                    ),
                                    win,
                                );
                                self.new_window_prompt.reset();
                                true
                            } else {
                                false
                            }
                        });
                    if let Some(resp) = inner_resp {
                        should_open &= !resp.inner.unwrap();
                    }
                    self.new_window_prompt.show = should_open;

                    for (i, map) in &mut self.file_structure.iter_mut().enumerate() {
                        if !map.is_empty() {
                            ui.group(|ui| {
                                ui.label(WindowType::from_ordinal(i as i8).unwrap().as_ref());
                                let mut v: Vec<(&Identifier, &mut Window)> =
                                    map.iter_mut().collect();
                                v.sort_unstable_by(|(id, _), (id2, _)| {
                                    id.partial_cmp(id2).unwrap()
                                });
                                for (_id, win) in v {
                                    if ui
                                        .button(format!("{}:{}", &*win.namespace, &*win.name))
                                        .clicked()
                                    {
                                        self.active_window = Some(win);
                                    }
                                }
                            });
                        }
                    }
                });
            });
        });
        if let Some(window) = &mut self.active_window {
            let graph_response = egui::CentralPanel::default()
                .show(ctx, |ui| {
                    window
                        .state
                        .draw_graph_editor(ui, AllNodeTemplates, &mut window.user_state)
                })
                .inner;

            for node_response in graph_response.node_responses {
                if let NodeResponse::User(user_event) = node_response {
                    match user_event {
                        Response::SetActiveNode(node_id) => {
                            window.user_state.active_node = Some(node_id)
                        }
                        Response::ClearActiveNode => window.user_state.active_node = None,
                        Response::IncreaseInputs(node_id) => {
                            increase_node_list_length(&mut window.state.graph, node_id);
                        }
                        Response::DecreaseInputs(node_id) => {
                            decrease_node_list_length(&mut window.state.graph, node_id);
                        }
                        Response::ChangeNodeType(node_id, new_template) => rebuild_node(
                            node_id,
                            &mut window.state.graph,
                            &mut window.user_state,
                            new_template,
                        ),
                        Response::ChangeInputLabel(node_id, from, to) => {
                            if let Some(node) = window.state.graph.nodes.get_mut(node_id) {
                                if let Some(mut m) =
                                    node.inputs.iter_mut().find(|(s, _)| s == &*from)
                                {
                                    m.0 = to.to_string();
                                }
                            }
                        }
                    }
                }
            }
        }
        {
            let mut should_open = self.open_project_prompt.show;
            let inner_response = egui::Window::new("Open Project Folder")
                .open(&mut should_open)
                .show(ctx, |ui| {
                    if self.open_project_prompt.ui_entered(ui) {
                        self.project_path = Some(PathBuf::from(&self.open_project_prompt.path));
                        Self::load_from_fs(
                            &self.project_path.as_ref().unwrap(),
                            &mut self.file_structure,
                        );
                        true
                    } else {
                        false
                    }
                });
            if let Some(response) = inner_response {
                if let Some(done) = response.inner {
                    self.open_project_prompt.show = !done;
                }
            }
        }
    }
}
