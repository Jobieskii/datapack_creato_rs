use eframe::egui::{self, Checkbox, ComboBox, DragValue, Ui};
use eframe::epaint::Color32;
use egui_node_graph::{DataTypeTrait, NodeId, WidgetValueTrait, InputId};
use std::borrow::Cow;

use crate::ui::ComboBoxEnum;
use crate::window::WindowType;

use super::blocks::BLOCK_LIST;
use super::inner_data_types::surface_rule_condition::{VerticalAnchor, SurfaceType};
use super::inner_data_types::{density_function, surface_rule_condition};
use super::inner_data_types::{
    density_function::DensityFunctionType, surface_rule::SurfaceRuleType,
    surface_rule_condition::SurfaceRuleConditionType, InnerDataType,
};
use super::node_types::NodeTemplate;
use super::{GraphState, GraphType, NodeData, Response};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum DataType {
    Value,
    Integer,
    Bool,
    Block,
    ValuesArray,
    Reference(WindowType),
    DullReference,
    ValueTypeSwitcher,
    WeirdScaledSampleRarityValueMapper,
    VerticalAnchor,
    SurfaceType,
    List(ComplexDataType),
    Single(ComplexDataType),
}
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ComplexDataType {
    Noise,
    DensityFunction,
    SurfaceRule,
    SurfaceRuleCondition,
    Biome,
}

impl DataType {
    #[allow(non_snake_case)]
    /// Returns
    /// - 'NodeTemplate' if DataType can be represented by a node and is an object in JSON
    /// - 'None' if DataType is never represented by a node but is an object in JSON
    /// - is unimplemented for DataTypes that are never objects in JSON
    pub fn defualt_NodeTemplate(&self) -> Option<NodeTemplate> {
        match self {
            DataType::Value => Some(NodeTemplate::ConstantValue),
            DataType::Block => Some(NodeTemplate::ConstantBlock),
            DataType::ValuesArray => unimplemented!(),
            DataType::Reference(x) => Some(NodeTemplate::Reference(*x)),
            DataType::ValueTypeSwitcher => unimplemented!(),
            DataType::List(_x) => unimplemented!(),
            DataType::Single(x) => match x {
                ComplexDataType::Noise => Some(NodeTemplate::Noise),
                ComplexDataType::DensityFunction => {
                    Some(NodeTemplate::DensityFunction(DensityFunctionType::Constant))
                }
                ComplexDataType::SurfaceRule => {
                    Some(NodeTemplate::SurfaceRule(SurfaceRuleType::Sequence))
                }
                ComplexDataType::SurfaceRuleCondition => {
                    Some(NodeTemplate::SurfaceRuleCondition(SurfaceRuleConditionType::YAbove))
                }
                ComplexDataType::Biome => Some(NodeTemplate::Reference(WindowType::Biome)),
            },
            DataType::WeirdScaledSampleRarityValueMapper => unimplemented!(),
            DataType::Integer => unimplemented!(),
            DataType::DullReference => unimplemented!(),
            DataType::VerticalAnchor => None,
            DataType::Bool => unimplemented!(),
            DataType::SurfaceType => unimplemented!(),
        }
    }
    #[allow(non_snake_case)]
    pub fn default_ValueType(&self) -> ValueType {
        match self {
            DataType::Value => ValueType::Value(0.),
            DataType::Block => ValueType::Block(0),
            DataType::ValuesArray => ValueType::ValuesArray(vec![0.]),
            DataType::Reference(x) => ValueType::Reference(*x, "".to_string()),
            DataType::ValueTypeSwitcher => unimplemented!(),
            DataType::List(_x) => ValueType::List(1),
            DataType::Single(x) => match x {
                ComplexDataType::Noise => ValueType::Noise,
                ComplexDataType::DensityFunction => ValueType::DensityFunction,
                ComplexDataType::SurfaceRule => ValueType::SurfaceRule,
                ComplexDataType::SurfaceRuleCondition => ValueType::SurfaceRuleCondition,
                ComplexDataType::Biome => todo!(),
            },
            DataType::WeirdScaledSampleRarityValueMapper => {
                ValueType::WeirdScaledSampleRarityValueMapper(
                    density_function::WeirdScaledSampleRarityValueMapper::Type1,
                )
            }
            DataType::Integer => ValueType::Integer(0),
            DataType::DullReference => ValueType::DullReference(String::new()),
            DataType::VerticalAnchor => {
                ValueType::VerticalAnchor(surface_rule_condition::VerticalAnchor::Absolute, 0)
            }
            DataType::Bool => ValueType::Bool(false),
            DataType::SurfaceType => ValueType::SurfaceType(surface_rule_condition::SurfaceType::Ceiling),
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
            DataType::List(_x) => unimplemented!(),
            DataType::DullReference => Color32::BROWN,
            _ => unimplemented!(),
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
            DataType::Single(ComplexDataType::SurfaceRuleCondition) => {
                Cow::Borrowed("surface rule condition")
            }
            DataType::Single(ComplexDataType::Biome) => Cow::Borrowed("biome"),
            DataType::List(x) => Cow::Owned(format!("list ({})", DataType::Single(*x).name())),
            DataType::ValueTypeSwitcher => Cow::Borrowed("value type switcher"),
            DataType::WeirdScaledSampleRarityValueMapper => Cow::Borrowed("rarity value mapper"),
            DataType::Integer => Cow::Borrowed("integer value"),
            DataType::DullReference => Cow::Borrowed("reference"),
            DataType::VerticalAnchor => Cow::Borrowed("vertical anchor"),
            DataType::Bool => Cow::Borrowed("boolean"),
            DataType::SurfaceType => Cow::Borrowed("surface type"),
        }
    }
}
type BlockId = usize;
#[derive(Clone)]
pub enum ValueType {
    // TODO: allow specifing min-max value
    Value(f32),
    ValuesArray(Vec<f32>),
    Bool(bool),
    Integer(i32),
    Block(BlockId),
    Noise,
    Biome,
    DensityFunction,
    Reference(WindowType, String),
    DullReference(String),
    SurfaceRule,
    SurfaceRuleCondition,
    WeirdScaledSampleRarityValueMapper(density_function::WeirdScaledSampleRarityValueMapper),
    VerticalAnchor(surface_rule_condition::VerticalAnchor, i32),
    SurfaceType(surface_rule_condition::SurfaceType),
    List(i32),
    InnerTypeSwitch(SwitchableInnerValueType),
}
#[derive(Clone, Copy, Debug)]
pub enum SwitchableInnerValueType {
    SurfaceRule(SurfaceRuleType),
    SurfaceRuleCondition(SurfaceRuleConditionType),
    DensityFunction(DensityFunctionType),
}

impl SwitchableInnerValueType {
    /// Returns exact `NodeTemplate` for this type.
    #[allow(non_snake_case)]
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
            ValueType::Integer(x) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(DragValue::new(x));
                });
            }
            ValueType::Bool(x) => {
                ui.add(Checkbox::new(x, param_name));
            }
            ValueType::Block(x) => {
                ui.horizontal(|ui| {
                    ComboBox::from_label(param_name)
                        .show_index(ui, x, BLOCK_LIST.len(), |i| BLOCK_LIST[i].id.to_string());
                });
            }
            ValueType::ValuesArray(arr) => {
                ui.vertical(|ui| {
                    for (i, val) in arr.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("Octave {}", i + 1));
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
            }
            ValueType::Reference(window_type, id) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    //TODO: add autocompletion here somehow
                    ui.text_edit_singleline(id);
                });
            }
            ValueType::DullReference(s) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.text_edit_singleline(s);
                });
            }
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
            ValueType::InnerTypeSwitch(x) => match x {
                SwitchableInnerValueType::SurfaceRule(x) => {
                    switcher_widget(x, ui, param_name, &mut ret, node_id)
                }
                SwitchableInnerValueType::SurfaceRuleCondition(x) => {
                    switcher_widget(x, ui, param_name, &mut ret, node_id)
                }
                SwitchableInnerValueType::DensityFunction(x) => {
                    switcher_widget(x, ui, param_name, &mut ret, node_id)
                }
            },
            // TODO: Refactor into common type for enumerations
            ValueType::WeirdScaledSampleRarityValueMapper(x) => {
                ui.horizontal(|ui| {
                    ComboBox::from_label(param_name)
                        .selected_text(x.as_ref())
                        .show_ui(ui, |ui| {
                            density_function::WeirdScaledSampleRarityValueMapper::show_ui(ui, x)
                        })
                });
            }
            ValueType::SurfaceType(x) => {
                ui.horizontal(|ui| {
                    ComboBox::from_label(param_name)
                        .selected_text(x.as_ref())
                        .show_ui(ui, |ui| {
                            SurfaceType::show_ui(ui, x)
                        })
                });
            }
            ValueType::VerticalAnchor(x, i) => {
                let y = x.clone();
                ui.horizontal(|ui| {
                    ComboBox::from_label(param_name)
                        .selected_text(x.as_ref())
                        .show_ui(ui, |ui| VerticalAnchor::show_ui(ui, x));
                    ui.add(DragValue::new(i));
                });
                if y != *x {
                    ret.push(Response::ChangeInputLabel(
                        node_id,
                        y.as_ref().into(),
                        x.as_ref().into(),
                    ))
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
fn switcher_widget<T: InnerDataType>(
    x: &mut T,
    ui: &mut Ui,
    param_name: &str,
    ret: &mut Vec<Response>,
    node_id: NodeId,
) {
    let y = x.clone();
    ui.horizontal(|ui| {
        ComboBox::from_label(param_name)
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

/// Decreases inputs by one by removing the last input.
pub fn decrease_node_list_length(graph: &mut GraphType, node_id: NodeId) -> Option<InputId> {
    if let Some(in_id) = graph.nodes.get(node_id).unwrap().input_ids().last() {
        graph.connections.remove(in_id);
        graph
            .nodes
            .get_mut(node_id)
            .unwrap()
            .inputs
            .retain(|(_, id)| *id != in_id);
        Some(in_id)
    } else {
        None
    }
}
/// Increses inputs by one by copying everything from the last input.
pub fn increase_node_list_length(graph: &mut GraphType, node_id: NodeId) -> InputId {
    let in_id = graph.nodes.get(node_id).unwrap().inputs.last().unwrap().1;
    let input = graph.inputs.get(in_id).unwrap();
    graph.add_input_param(
        node_id,
        "".to_string(),
        input.typ,
        input.value.clone(),
        input.kind,
        input.shown_inline,
    )
}
