use std::borrow::Cow;
use eframe::{egui::{self, DragValue}, epaint::Color32};
use egui_node_graph::{DataTypeTrait, WidgetValueTrait, NodeId};
use strum::{EnumCount, IntoEnumIterator};

use crate::{window::WindowType, ui::ComboBoxEnum};

use super::{GraphState, Response, NodeData, blocks::BLOCK_LIST, GraphType, surface_rule::SurfaceRuleType};



#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DataType {
    Value,
    Block,
    ValuesArray,
    SurfaceRuleType,
    List(ComplexDataType),
    Single(ComplexDataType)
}
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ComplexDataType {
    Noise,
    DensityFunction,
    SurfaceRule,
    SurfaceRuleCondition,
    Reference(WindowType)
}

impl DataTypeTrait<GraphState> for DataType {
    fn data_type_color(&self, _user_state: &mut GraphState) -> egui::Color32 {
        match self {
            DataType::Value => Color32::WHITE,
            DataType::Block => Color32::GREEN,
            DataType::Single(ComplexDataType::Noise) => Color32::GOLD,
            DataType::Single(ComplexDataType::DensityFunction) => Color32::LIGHT_GRAY,
            DataType::ValuesArray => Color32::LIGHT_YELLOW,
            DataType::Single(ComplexDataType::Reference(_)) => Color32::BLUE,
            DataType::Single(ComplexDataType::SurfaceRule) => Color32::RED,
            DataType::Single(ComplexDataType::SurfaceRuleCondition) => Color32::LIGHT_RED,
            DataType::List(x) => match x {
                ComplexDataType::SurfaceRule => Color32::DARK_RED,
                ComplexDataType::Reference(_) => Color32::DARK_GREEN,
                _ => Color32::DEBUG_COLOR
            },
            _ => Color32::DEBUG_COLOR
        }
    }

    fn name(&self) -> std::borrow::Cow<str> {
        match self {
            DataType::Value => Cow::Borrowed("value"),
            DataType::Block => Cow::Borrowed("block"),
            DataType::Single(ComplexDataType::Noise) => Cow::Borrowed("noise"),
            DataType::Single(ComplexDataType::DensityFunction) => Cow::Borrowed("density function"),
            DataType::ValuesArray => Cow::Borrowed("array of values"),
            DataType::Single(ComplexDataType::Reference(x)) => Cow::Owned(format!("Reference ({})", x.as_ref())),
            DataType::Single(ComplexDataType::SurfaceRule) => Cow::Borrowed("surface rule"),
            DataType::Single(ComplexDataType::SurfaceRuleCondition) => Cow::Borrowed("surface rule condition"),
            DataType::List(x) => Cow::Owned(format!("list ({})", DataType::Single(*x).name())),
            DataType::SurfaceRuleType => Cow::Borrowed("surface rule type"),
        }
    }
}
type BlockId = usize;
#[derive(Clone)]
pub enum ValueType {
    Value(f32),
    ValuesArray(Vec<f32>),
    Block(BlockId),
    Noise,
    DensityFunction,
    Reference(WindowType, String),
    SurfaceRule,
    SurfaceRuleCondition,
    List(i32),
    SurfaceRuleType(SurfaceRuleType)
}

impl Default for ValueType {
    fn default() -> Self {
        Self::Value(0.)
    }
}

impl WidgetValueTrait for ValueType {
    type Response = Response;
    type UserState = GraphState;
    type NodeData = NodeData;

    fn value_widget(
        &mut self,
        param_name: &str,
        node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut Self::UserState,
        _node_data: &Self::NodeData,
    ) -> Vec<Self::Response> {
        let mut ret = Vec::new();
        match self {
            ValueType::Value(x) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(DragValue::new(x));
                });
            }
            ValueType::Block(x) => {
                ui.horizontal(|ui| {
                    egui::ComboBox::from_label(param_name).show_index(ui, x, BLOCK_LIST.len(), |i| BLOCK_LIST[i].id.to_string());
                });
            },
            ValueType::ValuesArray(arr) => {
                ui.vertical(|ui| {
                    for (i, val) in arr.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("Octave {}", i+1));
                            ui.add(DragValue::new(val));
                        });
                    }
                    ui.horizontal(|ui| {
                        if ui.small_button("+").clicked() {
                            arr.push(0.);
                        }
                        if ui.small_button("-").clicked() {
                            arr.pop();
                        }
                    });
                });
            },
            ValueType::Reference(window_type, id) => {
                ui.vertical(|ui| {
                    //TODO: add autocompletion here somehow
                    ui.text_edit_singleline(id);
                });
            },
            ValueType::List(x) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    if *x > 0 {       
                        if ui.small_button("+").clicked() {
                            ret.push(Response::IncreaseInputs(node_id));
                            *x += 1;
                        }
                        if *x > 1 && ui.small_button("-").clicked() {
                            ret.push(Response::DecreaseInputs(node_id));
                            *x -= 1;
                        }
                    }
                });
            }
            ValueType::SurfaceRuleType(x) => {
                let y = x.clone();
                ui.horizontal(|ui| {
                    egui::ComboBox::from_label(param_name)
                        .selected_text(x.as_ref())
                        .show_ui(ui, |ui| {
                            SurfaceRuleType::show_ui(ui, x);
                        });
                });
                if *x != y {ret.push(Response::ChangeSurfaceRuleType(node_id, *x))}
            }
            _ => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                });
            }
        }
        ret
    }

}

pub fn decrease_node_list_length(graph: &mut GraphType, node_id: NodeId) {
    if let Some(in_id) = graph.nodes.get(node_id).unwrap().input_ids().last() {
        graph.connections.remove(in_id);
        graph.nodes.get_mut(node_id).unwrap().inputs.retain(|(_, id)| *id != in_id);
    }
}
pub fn increase_node_list_length(graph: &mut GraphType, node_id: NodeId) {
    let in_id = graph.nodes.get(node_id).unwrap().inputs[1].1;
    let input = graph.inputs.get(in_id).unwrap();
    graph.add_input_param(node_id, "".to_string(), input.typ, input.value.clone(), input.kind, input.shown_inline);
}