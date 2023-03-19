use std::fmt::Display;
use std::fs::{File, DirBuilder};
use std::hash::Hash;
use std::io::{Write, self, Seek, SeekFrom};
use std::path::PathBuf;
use eframe::epaint::{Vec2, Pos2};
use egui_node_graph::{NodeTemplateTrait, NodeId, Node};
use enum_ordinalize::Ordinalize;
use log::warn;
use strum::{EnumCount, EnumIter, AsRefStr};

use crate::nodes::node_types::NodeTemplate;
use crate::ui::ComboBoxEnum;
use crate::{nodes::GraphState, app::EditorStateType};

// #[derive(Clone)]
pub struct Window {
    pub window_type: WindowType,
    pub name: String,
    pub filepath: PathBuf,
    pub namespace: String,
    /// empty if window is newly created
    pub file: Option<File>,
    pub state: EditorStateType,
    pub user_state: GraphState,
    pub dirty: bool,
    pub root_node: NodeId
}

impl PartialEq for Window {
    fn eq(&self, other: &Self) -> bool {
        self.window_type == other.window_type && self.name == other.name && self.namespace == other.namespace
    }
}
impl Eq for Window {}
impl Hash for Window {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.window_type.hash(state);
        self.name.hash(state);
        self.namespace.hash(state);
    }
}
impl PartialOrd for Window {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.window_type.partial_cmp(&other.window_type) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.namespace.partial_cmp(&other.namespace) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.name.partial_cmp(&other.name) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        Some(core::cmp::Ordering::Equal)
    }
}

#[derive(PartialEq, PartialOrd, Copy, Clone, EnumCount, EnumIter, AsRefStr, Hash, Eq, Ordinalize, Debug)]
pub enum WindowType {
    #[strum(serialize= "Density Function")]
    DensityFunction,
    Noise
}

impl ComboBoxEnum for WindowType{}

impl Display for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{} ({})", self.namespace, self.name, self.window_type.as_ref())
    }
}

impl Window {
    pub fn new(filename: String, namespace: String, window_type: WindowType, project_path: &PathBuf) -> Self {
        let mut state = EditorStateType::default();
        let mut user_state = GraphState::default();

        let root_node = Self::add_default_node(&mut state, &mut user_state, window_type);
        let mut filepath = project_path.clone();
        filepath.push(&namespace);
        filepath.push(Self::path_from(window_type));
        filepath.push(&filename);
        filepath.set_extension("json");
        Self {
            window_type,
            name: filename,
            file: None,
            state,
            user_state,
            dirty: false,
            filepath,
            namespace,
            root_node
        }
    }
    pub fn from_file() {
        todo!()
    }
    fn add_default_node(state: &mut EditorStateType, user_state: &mut GraphState, window_type: WindowType) -> NodeId{
        let node_kind = NodeTemplate::Output(window_type);        
        let new_node = state.graph.add_node(
            node_kind.node_graph_label(user_state),
            node_kind.user_data(user_state),
            |graph, node_id| node_kind.build_node(graph, user_state, node_id),
        );
        state.node_positions.insert(
            new_node,
            Pos2::ZERO,
        );
        state.node_order.push(new_node);
        new_node
    }
    pub fn save_to_file(&mut self, s: String) {
        if let Some(file) = self.file.as_mut() {
            file.set_len(0);
            file.seek(SeekFrom::Start(0));
            file.write_all(s.as_bytes());
        } else {
            let mut dir = self.filepath.clone();
            dir.pop();
            DirBuilder::new()
                .recursive(true)
                .create(dir);

            self.file = Some(File::create(&self.filepath).unwrap());
            self.file.as_mut().unwrap().write_all(s.as_bytes());
        }
    }
    fn path_from(window_type: WindowType) -> String {
        match window_type {
            WindowType::DensityFunction => "worldgen/density_function",
            WindowType::Noise => "worldgen/noise",
        }.to_string()
    }
}