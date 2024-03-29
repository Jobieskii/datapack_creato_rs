pub mod blocks;
pub mod data_types;
pub mod inner_data_types;
pub mod node_types;

use eframe::{egui, epaint::Pos2};
use egui_node_graph::{
    self, Graph, NodeDataTrait, NodeId, NodeResponse, NodeTemplateTrait, UserResponseTrait, InputId,
};

use crate::app::EditorStateType;

use self::{
    data_types::{DataType, ValueType},
    node_types::NodeTemplate,
};

pub type GraphType = Graph<NodeData, DataType, ValueType>;

#[derive(Clone, Copy)]
pub struct NodeData {
    pub template: NodeTemplate,
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
        Response: egui_node_graph::UserResponseTrait,
    {
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
            let button =
                egui::Button::new(egui::RichText::new("active").color(egui::Color32::BLACK))
                    .fill(egui::Color32::GOLD);
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
        _node_id: NodeId,
        _graph: &Graph<Self, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
    ) -> bool {
        if let NodeTemplate::Output(_) = self.template {
            return false;
        }
        true
    }
}

#[derive(Clone, Debug)]
pub enum Response {
    SetActiveNode(NodeId),
    ClearActiveNode,
    IncreaseInputs(NodeId),
    DecreaseInputs(NodeId),
    ChangeNodeType(NodeId, NodeTemplate),
    ChangeInputLabel(NodeId, Box<str>, Box<str>)
}
impl UserResponseTrait for Response {}
#[derive(Clone)]
pub struct GraphState {
    pub active_node: Option<NodeId>,
}
impl Default for GraphState {
    fn default() -> Self {
        Self {
            active_node: Default::default(),
        }
    }
}
/// rebuilds node in place. Keeps output connection.
pub fn rebuild_node(
    node_id: NodeId,
    graph: &mut GraphType,
    user_state: &mut GraphState,
    template: NodeTemplate,
) {
    let node = graph.nodes.get(node_id).unwrap().clone();
    let old_ouput = node.output_ids().next().unwrap();
    let old_input_opt = graph
        .connections
        .iter()
        .find(|(_, &o)| o == old_ouput)
        .map(|(i, _o)| i);
    node.input_ids().for_each(|x| graph.remove_input_param(x));
    node.output_ids().for_each(|x| graph.remove_output_param(x));

    template.build_node(graph, user_state, node_id);
    let mut node = graph.nodes.get_mut(node_id).unwrap();
    node.user_data.template = template;
    node.label = template.node_graph_label(user_state);
    if let Some(old_input) = old_input_opt {
        let new_output = graph
            .nodes
            .get(node_id)
            .unwrap()
            .output_ids()
            .next()
            .unwrap();
        graph.add_connection(new_output, old_input);
    }
}
/// Implementation copied from the library. Shame there's no function like that there.
pub fn add_node(
    state: &mut EditorStateType,
    user_state: &mut GraphState,
    template: NodeTemplate,
    pos: Pos2,
) -> NodeId {
    let new_node = state.graph.add_node(
        template.node_graph_label(user_state),
        template.user_data(user_state),
        |graph, node_id| template.build_node(graph, user_state, node_id),
    );
    state.node_positions.insert(new_node, pos);
    state.node_order.push(new_node);
    new_node
}
