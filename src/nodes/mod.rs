
pub mod data_types;
pub mod node_types;
pub mod blocks;
pub mod density_function;
pub mod surface_rule;

use egui_node_graph::{self, NodeId, Graph, NodeDataTrait, UserResponseTrait, NodeResponse};
use eframe::egui;

use self::{data_types::{DataType, ValueType}, node_types::NodeTemplate};


pub type GraphType = Graph<NodeData, DataType, ValueType>;

#[derive(Clone, Copy)]
pub struct NodeData {
    pub template: NodeTemplate
}

impl NodeDataTrait for NodeData {
    type Response = Response;
    type UserState = GraphState;
    type DataType = DataType;
    type ValueType = ValueType;

    fn bottom_ui(
        &self,
        ui: &mut egui::Ui,
        node_id: NodeId,
        _graph: &Graph<Self, Self::DataType, Self::ValueType>,
        user_state: &mut Self::UserState,
    ) -> Vec<egui_node_graph::NodeResponse<Self::Response, Self>>
    where
        Response: egui_node_graph::UserResponseTrait {
        let mut responses = vec![];
        let is_active = user_state
            .active_node
            .map(|id| id == node_id)
            .unwrap_or(false);
        if !is_active {
            if ui.button("set active").clicked() {
                responses.push(NodeResponse::User(Response::SetActiveNode(node_id)));
            }
        } else {
            let button = egui::Button::new(
                egui::RichText::new("active").color(egui::Color32::BLACK)
            ).fill(egui::Color32::GOLD);
            if ui.add(button).clicked() {
                responses.push(NodeResponse::User(Response::ClearActiveNode));
            }
        }
        responses
    }

    fn titlebar_color(
        &self,
        _ui: &egui::Ui,
        _node_id: NodeId,
        _graph: &Graph<Self, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
    ) -> Option<egui::Color32> {
        None
    }

    fn can_delete(
        &self,
        node_id: NodeId,
        graph: &Graph<Self, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
    ) -> bool {
        if let NodeTemplate::Output(_) = self.template {
            return false
        }
        true
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Response {
    SetActiveNode(NodeId),
    ClearActiveNode,
    IncreaseInputs(NodeId),
    DecreaseInputs(NodeId)
}
impl UserResponseTrait for Response {

}
#[derive(Clone)]
pub struct GraphState {
    pub active_node: Option<NodeId>,
}
impl Default for GraphState {
    fn default() -> Self {
        Self { 
            active_node: Default::default()
        }
    }
}
