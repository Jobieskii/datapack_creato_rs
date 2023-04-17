use std::borrow::Cow;
use eframe::{egui::{self, DragValue, Ui}, epaint::Color32};
use egui_node_graph::{DataTypeTrait, WidgetValueTrait, NodeId};

use crate::{window::WindowType, ui::ComboBoxEnum};

use super::{GraphState, Response, NodeData, blocks::BLOCK_LIST, GraphType, inner_data_types::{surface_rule::SurfaceRuleType, surface_rule_condition::SurfaceRuleConditionType, density_function::DensityFunctionType, InnerDataType}, node_types::NodeTemplate};



#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum DataType {
    Value,
    Block,
    ValuesArray,
    Reference(WindowType),
    ValueTypeSwitcher,
    List(ComplexDataType),
    Single(ComplexDataType)
}
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ComplexDataType {
    Noise,
    DensityFunction,
    SurfaceRule,
    SurfaceRuleCondition
}

impl DataType {
    pub fn defualt_NodeTemplate(&self) -> NodeTemplate {
        match self {
            DataType::Value => NodeTemplate::ConstantValue,
            DataType::Block => NodeTemplate::ConstantBlock,
            DataType::ValuesArray => unimplemented!(),
            DataType::Reference(x) => NodeTemplate::Reference(*x),
            DataType::ValueTypeSwitcher => unimplemented!(),
            DataType::List(x) => unimplemented!(),
            DataType::Single(x) => match x {
                ComplexDataType::Noise => NodeTemplate::Noise,
                ComplexDataType::DensityFunction => NodeTemplate::DensityFunction(DensityFunctionType::Constant),
                ComplexDataType::SurfaceRule => NodeTemplate::SurfaceRule(SurfaceRuleType::Sequence),
                ComplexDataType::SurfaceRuleCondition => NodeTemplate::SurfaceRuleCondition(SurfaceRuleConditionType::YAbove),
            },
        }
    }
    pub fn default_ValueType(&self) -> ValueType {
        match self {
            DataType::Value => ValueType::Value(0.),
            DataType::Block => ValueType::Block(0),
            DataType::ValuesArray => ValueType::ValuesArray(vec![0.]),
            DataType::Reference(x) => ValueType::Reference(*x, "".to_string()),
            DataType::ValueTypeSwitcher => unimplemented!(),
            DataType::List(x) => ValueType::List(1),
            DataType::Single(x) => match x {
                ComplexDataType::Noise => ValueType::Noise,
                ComplexDataType::DensityFunction => ValueType::DensityFunction,
                ComplexDataType::SurfaceRule => ValueType::SurfaceRule,
                ComplexDataType::SurfaceRuleCondition => ValueType::SurfaceRuleCondition,
            },
        }
    }
}

impl DataTypeTrait<GraphState> for DataType {
    fn data_type_color(&self, _user_state: &mut GraphState) -> egui::Color32 {
        match self {
            DataType::Value => Color32::WHITE,
            DataType::Block => Color32::GREEN,
            DataType::Single(ComplexDataType::Noise) => Color32::GOLD,
            DataType::Single(ComplexDataType::DensityFunction) => Color32::LIGHT_GRAY,
            DataType::ValuesArray => Color32::LIGHT_YELLOW,
            DataType::Reference(_) => Color32::BLUE,
            DataType::Single(ComplexDataType::SurfaceRule) => Color32::RED,
            DataType::Single(ComplexDataType::SurfaceRuleCondition) => Color32::LIGHT_RED,
            DataType::List(x) => unimplemented!(),
            _ => unimplemented!()
        }
    }

    fn name(&self) -> std::borrow::Cow<str> {
        match self {
            DataType::Value => Cow::Borrowed("value"),
            DataType::Block => Cow::Borrowed("block"),
            DataType::Single(ComplexDataType::Noise) => Cow::Borrowed("noise"),
            DataType::Single(ComplexDataType::DensityFunction) => Cow::Borrowed("density function"),
            DataType::ValuesArray => Cow::Borrowed("array of values"),
            DataType::Reference(x) => Cow::Owned(format!("Reference ({})", x.as_ref())),
            DataType::Single(ComplexDataType::SurfaceRule) => Cow::Borrowed("surface rule"),
            DataType::Single(ComplexDataType::SurfaceRuleCondition) => Cow::Borrowed("surface rule condition"),
            DataType::List(x) => Cow::Owned(format!("list ({})", DataType::Single(*x).name())),
            DataType::ValueTypeSwitcher => Cow::Borrowed("value type switcher"),
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
    InnerTypeSwitch(SwitchableInnerValueType),
}
#[derive(Clone, Copy, Debug)]
pub enum SwitchableInnerValueType {
    SurfaceRule(SurfaceRuleType),
    SurfaceRuleCondition(SurfaceRuleConditionType),
    DensityFunction(DensityFunctionType)
}

impl SwitchableInnerValueType {
    /// Returns exact `NodeTemplate` for this type.
    pub fn to_NodeTemplate(&self) -> NodeTemplate {
        match self {
            SwitchableInnerValueType::SurfaceRule(x) => x.to_NodeTemplate(),
            SwitchableInnerValueType::SurfaceRuleCondition(x) => x.to_NodeTemplate(),
            SwitchableInnerValueType::DensityFunction(x) => x.to_NodeTemplate(),
        }
    }
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
            ValueType::InnerTypeSwitch(x) => {
                match x {
                    SwitchableInnerValueType::SurfaceRule(x) => switcher_widget(x, ui, param_name, &mut ret, node_id),
                    SwitchableInnerValueType::SurfaceRuleCondition(x) => switcher_widget(x, ui, param_name, &mut ret, node_id),
                    SwitchableInnerValueType::DensityFunction(x) => switcher_widget(x, ui, param_name, &mut ret, node_id),
                }
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
fn switcher_widget<T: InnerDataType>(x: &mut T, ui: &mut Ui, param_name: &str, ret: &mut Vec<Response>, node_id: NodeId) {
    let y = x.clone();
    ui.horizontal(|ui| {
        egui::ComboBox::from_label(param_name)
            .selected_text(x.as_ref())
            .show_ui(ui, |ui| {
                T::show_ui(ui, x);
            });
    });
    if *x != y {
        let template = x.to_NodeTemplate();
        ret.push(Response::ChangeNodeType(node_id, template))
    }
}

pub fn decrease_node_list_length(graph: &mut GraphType, node_id: NodeId) {
    if let Some(in_id) = graph.nodes.get(node_id).unwrap().input_ids().last() {
        graph.connections.remove(in_id);
        graph.nodes.get_mut(node_id).unwrap().inputs.retain(|(_, id)| *id != in_id);
    }
}
pub fn increase_node_list_length(graph: &mut GraphType, node_id: NodeId) {
    let in_id = graph.nodes.get(node_id).unwrap().inputs.last().unwrap().1;
    let input = graph.inputs.get(in_id).unwrap();
    graph.add_input_param(node_id, "".to_string(), input.typ, input.value.clone(), input.kind, input.shown_inline);
}