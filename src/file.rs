use std::hash::Hash;
use eframe::epaint::{Vec2, Pos2};
use egui_node_graph::NodeTemplateTrait;
use enum_ordinalize::Ordinalize;
use strum::{EnumCount, EnumIter, AsRefStr};

use crate::nodes::node_types::NodeTemplate;
use crate::ui::ComboBoxEnum;
use crate::{nodes::GraphState, app::EditorStateType};

#[derive(Clone)]
pub struct Window {
    pub window_type: WindowType,
    pub name: String,
    pub filepath: String,
    pub namespace: String,
  //  pub file: File,
    pub state: EditorStateType,
    pub user_state: GraphState,
    pub dirty: bool
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

#[derive(PartialEq, PartialOrd, Copy, Clone, EnumCount, EnumIter, AsRefStr, Hash, Eq, Ordinalize)]
pub enum WindowType {
    #[strum(serialize= "Density Function")]
    DensityFunction,
    Noise
}

impl ComboBoxEnum for WindowType{}


impl Window {
    pub fn new(filename: String, namespace: String, window_type: WindowType) -> Self {
        let mut state = EditorStateType::default();
        let mut user_state = GraphState::default();
        
        Self::add_default_node(&mut state, &mut user_state, window_type);
       // let file = File::open(filename);
        Self {
            window_type,
            name: filename.clone(),
       //     file: file.unwrap(),
            state,
            user_state,
            dirty: false,
            filepath: Self::path_from(namespace.clone(), filename, window_type),
            namespace,
        }
    }
    fn add_default_node(state: &mut EditorStateType, user_state: &mut GraphState, window_type: WindowType) {
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
    }
    fn save_to_file(&self) {
        //self.file.write(buf)
    }
    fn path_from(namespace: String, filename: String, window_type: WindowType) -> String {
        let path = match window_type {
            WindowType::DensityFunction => "/worldgen/density_function",
            WindowType::Noise => "/worldgen/noise",
        };
        format!("{}{}/{}.json", namespace, path, filename)
    } 
}