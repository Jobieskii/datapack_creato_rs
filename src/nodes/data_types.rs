use std::borrow::Cow;
use eframe::{egui::{self, DragValue}, epaint::Color32};
use egui_node_graph::{DataTypeTrait, WidgetValueTrait, NodeId};

use crate::{file::WindowType, app::Identifier};

use super::{GraphState, Response, NodeData, blocks::BLOCK_LIST};



#[derive(PartialEq, Eq, Clone)]
pub enum DataType {
    Value,
    Block,
    Noise,
    DensityFunction,
    ValuesArray,
    Reference(WindowType)
}

impl DataTypeTrait<GraphState> for DataType {
    fn data_type_color(&self, _user_state: &mut GraphState) -> egui::Color32 {
        match self {
            DataType::Value => Color32::WHITE,
            DataType::Block => Color32::GREEN,
            DataType::Noise => Color32::GOLD,
            DataType::DensityFunction => Color32::LIGHT_GRAY,
            DataType::ValuesArray => Color32::LIGHT_YELLOW,
            DataType::Reference(x) => Color32::BLUE,
        }
    }

    fn name(&self) -> std::borrow::Cow<str> {
        match self {
            DataType::Value => Cow::Borrowed("value"),
            DataType::Block => Cow::Borrowed("block"),
            DataType::Noise => Cow::Borrowed("noise"),
            DataType::DensityFunction => Cow::Borrowed("density function"),
            DataType::ValuesArray => Cow::Borrowed("array of values"),
            DataType::Reference(x) => Cow::Owned(format!("Reference ({})", x.as_ref())),
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
    Reference(WindowType, String)
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
        _node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut Self::UserState,
        _node_data: &Self::NodeData,
    ) -> Vec<Self::Response> {
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
            ValueType::DensityFunction => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                });
            }
            ValueType::Noise => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                });
            }
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
                    let mut text = String::new();
                    //TODO: add autocompletion here somehow
                    ui.text_edit_singleline(id);
                });
            },
        }
        Vec::new()
    }

}