use std::borrow::Cow;

use egui_node_graph::{InputParamKind, NodeId, NodeTemplateIter, NodeTemplateTrait};

use crate::{
    nodes::inner_data_types::{
        density_function::WeirdScaledSampleRarityValueMapper, surface_rule_condition,
    },
    window::WindowType,
};

use super::{
    data_types::{ComplexDataType, DataType, SwitchableInnerValueType, ValueType},
    inner_data_types::{
        density_function::DensityFunctionType, surface_rule::SurfaceRuleType,
        surface_rule_condition::SurfaceRuleConditionType,
    },
    GraphState, GraphType, NodeData,
};

#[derive(Copy, Clone, Debug)]
pub enum NodeTemplate {
    ConstantValue,
    AddValue,
    DensityFunction(DensityFunctionType),
    ConstantBlock,
    Noise,
    Reference(WindowType),
    Output(WindowType),
    SurfaceRule(SurfaceRuleType),
    SurfaceRuleCondition(SurfaceRuleConditionType),
}

impl NodeTemplateTrait for NodeTemplate {
    type NodeData = NodeData;
    type DataType = DataType;
    type ValueType = ValueType;
    type UserState = GraphState;

    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> std::borrow::Cow<str> {
        match self {
            NodeTemplate::ConstantValue => Cow::Borrowed("Value"),
            NodeTemplate::AddValue => Cow::Borrowed("Add"),
            NodeTemplate::ConstantBlock => Cow::Borrowed("Block"),
            NodeTemplate::DensityFunction(_x) => Cow::Borrowed("Density Function"),
            NodeTemplate::Noise => Cow::Borrowed("Noise"),
            NodeTemplate::Reference(x) => Cow::Owned(format!("Reference ({})", x.as_ref())),
            NodeTemplate::Output(x) => Cow::Owned(format!("Output ({})", x.as_ref())),
            NodeTemplate::SurfaceRule(_) => Cow::Borrowed("Surface Rule"),
            NodeTemplate::SurfaceRuleCondition(_) => Cow::Borrowed("Surface Rule Condition"),
        }
    }

    fn node_graph_label(&self, user_state: &mut Self::UserState) -> String {
        match self {
            NodeTemplate::SurfaceRule(x) => format!("Surface Rule ({})", x.as_ref()),
            NodeTemplate::DensityFunction(x) => format!("Denstity Function ({})", x.as_ref()),
            _ => self.node_finder_label(user_state).to_string(),
        }
    }

    fn user_data(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
        NodeData { template: *self }
    }

    fn build_node(
        &self,
        graph: &mut egui_node_graph::Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
        node_id: NodeId,
    ) {
        let input_value = |graph: &mut GraphType, name: &str, kind: InputParamKind| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DataType::Value,
                ValueType::Value(0.),
                kind,
                true,
            );
        };
        let output_value = |graph: &mut GraphType, name: &str| {
            graph.add_output_param(node_id, name.to_string(), DataType::Value);
        };
        let input_int = |graph: &mut GraphType, name: &str| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DataType::Integer,
                ValueType::Integer(0),
                InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_bool = |graph: &mut GraphType, name: &str| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DataType::Bool,
                ValueType::Bool(false),
                InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_block = |graph: &mut GraphType, name: &str, kind: InputParamKind| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DataType::Block,
                ValueType::Block(0),
                kind,
                true,
            );
        };
        let output_block = |graph: &mut GraphType, name: &str| {
            graph.add_output_param(node_id, name.to_string(), DataType::Block);
        };
        let input_df = |graph: &mut GraphType, name: &str| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DataType::Single(ComplexDataType::DensityFunction),
                ValueType::DensityFunction,
                InputParamKind::ConnectionOnly,
                true,
            );
        };
        let output_df = |graph: &mut GraphType, name: &str| {
            graph.add_output_param(
                node_id,
                name.to_string(),
                DataType::Single(ComplexDataType::DensityFunction),
            );
        };
        let input_noise = |graph: &mut GraphType, name: &str| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DataType::Single(ComplexDataType::Noise),
                ValueType::Noise,
                InputParamKind::ConnectionOnly,
                true,
            );
        };
        let output_noise = |graph: &mut GraphType, name: &str| {
            graph.add_output_param(
                node_id,
                name.to_string(),
                DataType::Single(ComplexDataType::Noise),
            );
        };
        let input_values_arr = |graph: &mut GraphType, name: &str| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DataType::ValuesArray,
                ValueType::ValuesArray(vec![0.]),
                InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_reference =
            |graph: &mut GraphType, name: &str, kind: InputParamKind, window_type: &WindowType| {
                graph.add_input_param(
                    node_id,
                    name.to_string(),
                    DataType::Reference(*window_type),
                    ValueType::Reference(*window_type, String::new()),
                    kind,
                    true,
                );
            };
        let input_type_switch = |graph: &mut GraphType, st: SwitchableInnerValueType| {
            graph.add_input_param(
                node_id,
                String::from("type"),
                DataType::ValueTypeSwitcher,
                ValueType::InnerTypeSwitch(st),
                InputParamKind::ConstantOnly,
                true,
            );
        };
        let input_biome = |graph: &mut GraphType, name: &str, kind: InputParamKind| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                DataType::Single(ComplexDataType::Biome),
                ValueType::Biome,
                kind,
                true,
            );
        };
        let output_biome = |graph: &mut GraphType, name: &str| {
            graph.add_output_param(
                node_id,
                name.to_string(),
                DataType::Single(ComplexDataType::Biome),
            );
        };
        //TODO: Make sure label wording matches JSON
        match self {
            NodeTemplate::ConstantValue => {
                input_value(graph, "value", InputParamKind::ConstantOnly);
                output_value(graph, "out");
            }
            NodeTemplate::AddValue => {
                input_value(graph, "arg1", InputParamKind::ConnectionOrConstant);
                input_value(graph, "arg2", InputParamKind::ConnectionOrConstant);
                output_value(graph, "out");
            }
            NodeTemplate::ConstantBlock => {
                input_block(graph, "block", InputParamKind::ConstantOnly);
                output_block(graph, "out");
            }
            NodeTemplate::DensityFunction(x) => {
                output_df(graph, "out");
                input_type_switch(graph, SwitchableInnerValueType::DensityFunction(*x));
                use DensityFunctionType::*;
                match x {
                    Interpolated | FlatCache | Cache2d | CacheOnce | CacheAllInCell => {
                        input_df(graph, "argument");
                    }
                    Abs | Square | Cube | HalfNegative | QuarterNegative | Squeeze => {
                        input_df(graph, "argument");
                    }
                    Add | Mul | Min | Max => {
                        input_df(graph, "argument1");
                        input_df(graph, "argument2");
                    }
                    BlendAlpha | BlendOffset | Beardifier | EndIslands => {}
                    BlendDensity => {
                        input_df(graph, "argument");
                    }
                    Constant => {
                        input_value(graph, "argument", InputParamKind::ConstantOnly);
                    }
                    OldBlendedNoise => {
                        input_value(graph, "xz_scale", InputParamKind::ConnectionOrConstant);
                        input_value(graph, "y_scale", InputParamKind::ConnectionOrConstant);
                        input_value(graph, "xz_factor", InputParamKind::ConnectionOrConstant);
                        input_value(graph, "y_factor", InputParamKind::ConnectionOrConstant);
                        input_value(
                            graph,
                            "smear_scale_multiplier",
                            InputParamKind::ConnectionOrConstant,
                        );
                    }
                    Noise => {
                        input_reference(
                            graph,
                            "noise",
                            InputParamKind::ConstantOnly,
                            &WindowType::Noise,
                        );
                        input_value(graph, "xz_scale", InputParamKind::ConnectionOrConstant);
                        input_value(graph, "y_scale", InputParamKind::ConnectionOrConstant);
                    }
                    WeirdScaledSampler => {
                        graph.add_input_param(
                            node_id,
                            String::from("rarity_value_mapper"),
                            DataType::WeirdScaledSampleRarityValueMapper,
                            ValueType::WeirdScaledSampleRarityValueMapper(
                                WeirdScaledSampleRarityValueMapper::Type1,
                            ),
                            InputParamKind::ConstantOnly,
                            true,
                        );
                        input_reference(
                            graph,
                            "noise",
                            InputParamKind::ConstantOnly,
                            &WindowType::Noise,
                        );
                        input_df(graph, "input");
                    }
                    ShiftedNoise => {
                        input_reference(
                            graph,
                            "noise",
                            InputParamKind::ConstantOnly,
                            &WindowType::Noise,
                        );
                        input_value(graph, "xz_scale", InputParamKind::ConnectionOrConstant);
                        input_value(graph, "y_scale", InputParamKind::ConnectionOrConstant);
                        input_df(graph, "shift_x");
                        input_df(graph, "shift_y");
                        input_df(graph, "shift_z");
                    }
                    RangeChoice => {
                        input_df(graph, "input");
                        input_value(graph, "min_inclusive", InputParamKind::ConnectionOrConstant);
                        input_value(graph, "max_inclusive", InputParamKind::ConnectionOrConstant);
                        input_df(graph, "when_in_range");
                        input_df(graph, "when_out_of_range");
                    }
                    ShiftA | ShiftB | Shift => {
                        input_reference(
                            graph,
                            "argument",
                            InputParamKind::ConnectionOrConstant,
                            &WindowType::Noise,
                        );
                    }
                    Clamp => {
                        input_df(graph, "input");
                        input_value(graph, "min", InputParamKind::ConnectionOrConstant);
                        input_value(graph, "max", InputParamKind::ConnectionOrConstant);
                    }
                    Spline => {
                        todo!("Splines are a lot of work, huh");
                    }
                    YClampedGradient => {
                        input_int(graph, "from_y");
                        input_int(graph, "to_y");
                        input_value(graph, "from_value", InputParamKind::ConnectionOrConstant);
                        input_value(graph, "to_value", InputParamKind::ConnectionOrConstant);
                    }
                }
            }

            NodeTemplate::Noise => {
                input_value(graph, "firstOctave", InputParamKind::ConstantOnly);
                input_values_arr(graph, "amplitudes");
                output_noise(graph, "out");
            }
            NodeTemplate::Reference(x) => {
                input_reference(graph, "Reference", InputParamKind::ConstantOnly, x);
                match x {
                    WindowType::DensityFunction => output_df(graph, "out"),
                    WindowType::Noise => output_noise(graph, "out"),
                    WindowType::Biome => output_biome(graph, "out"),
                }
            }
            NodeTemplate::Output(x) => match x {
                WindowType::DensityFunction => {
                    input_df(graph, "output");
                }
                WindowType::Noise => {
                    input_value(graph, "firstOctave", InputParamKind::ConstantOnly);
                    input_values_arr(graph, "amplitudes");
                }
                WindowType::Biome => todo!(),
            },
            NodeTemplate::SurfaceRule(x) => {
                graph.add_output_param(
                    node_id,
                    "out".to_string(),
                    DataType::Single(ComplexDataType::SurfaceRule),
                );
                input_type_switch(graph, SwitchableInnerValueType::SurfaceRule(*x));
                use SurfaceRuleType::*;
                match x {
                    Bandlands => {}
                    Block => {
                        graph.add_input_param(
                            node_id,
                            "result_state".to_string(),
                            DataType::Block,
                            ValueType::Block(0),
                            InputParamKind::ConnectionOrConstant,
                            true,
                        );
                    }
                    Sequence => {
                        graph.add_input_param(
                            node_id,
                            "sequence".to_string(),
                            DataType::List(ComplexDataType::SurfaceRule),
                            ValueType::List(1),
                            InputParamKind::ConstantOnly,
                            true,
                        );
                        graph.add_input_param(
                            node_id,
                            "".to_string(),
                            DataType::Single(ComplexDataType::SurfaceRule),
                            ValueType::SurfaceRule,
                            InputParamKind::ConnectionOrConstant,
                            true,
                        );
                    }
                    Condition => {
                        graph.add_input_param(
                            node_id,
                            "if true".to_string(),
                            DataType::Single(ComplexDataType::SurfaceRuleCondition),
                            ValueType::SurfaceRuleCondition,
                            InputParamKind::ConnectionOnly,
                            true,
                        );
                        graph.add_input_param(
                            node_id,
                            "then run".to_string(),
                            DataType::Single(ComplexDataType::SurfaceRule),
                            ValueType::SurfaceRule,
                            InputParamKind::ConnectionOnly,
                            true,
                        );
                    }
                };
            }
            NodeTemplate::SurfaceRuleCondition(x) => {
                graph.add_output_param(
                    node_id,
                    "out".to_string(),
                    DataType::Single(ComplexDataType::SurfaceRuleCondition),
                );
                input_type_switch(graph, SwitchableInnerValueType::SurfaceRuleCondition(*x));
                use SurfaceRuleConditionType::*;
                match x {
                    Biome => {
                        graph.add_input_param(
                            node_id,
                            "biome_is".to_string(),
                            DataType::List(ComplexDataType::Biome),
                            ValueType::List(1),
                            InputParamKind::ConstantOnly,
                            true,
                        );
                        input_biome(graph, "", InputParamKind::ConnectionOnly);
                    }
                    NoiseThreshold => {
                        input_reference(
                            graph,
                            "noise",
                            InputParamKind::ConnectionOrConstant,
                            &WindowType::Noise,
                        );
                        input_value(graph, "min_threshold", InputParamKind::ConstantOnly);
                        input_value(graph, "max_threshold", InputParamKind::ConstantOnly);
                    }
                    VerticalGradient => {
                        graph.add_input_param(
                            node_id,
                            "random_name".to_string(),
                            DataType::DullReference,
                            ValueType::DullReference(String::new()),
                            InputParamKind::ConnectionOrConstant,
                            true,
                        );
                        graph.add_input_param(
                            node_id,
                            "true_at_and_below".to_string(),
                            DataType::VerticalAnchor,
                            ValueType::VerticalAnchor(
                                surface_rule_condition::VerticalAnchor::Absolute,
                                0,
                            ),
                            InputParamKind::ConstantOnly,
                            true,
                        );
                        graph.add_input_param(
                            node_id,
                            "false_at_and_above".to_string(),
                            DataType::VerticalAnchor,
                            ValueType::VerticalAnchor(
                                surface_rule_condition::VerticalAnchor::Absolute,
                                0,
                            ),
                            InputParamKind::ConstantOnly,
                            true,
                        );
                    }
                    YAbove => {
                        graph.add_input_param(
                            node_id,
                            "anchor".to_string(),
                            DataType::VerticalAnchor,
                            ValueType::VerticalAnchor(
                                surface_rule_condition::VerticalAnchor::Absolute,
                                0,
                            ),
                            InputParamKind::ConstantOnly,
                            true,
                        );
                        input_int(graph, "surface_depth_multiplier");
                        input_bool(graph, "add_stone_depth");
                    }
                    Water => {
                        input_int(graph, "offset");
                        input_int(graph, "surface_depth_multiplier");
                        input_bool(graph, "add_stone_depth");
                    }
                    Temperature | Steep | Hole | AbovePreliminarySurface => {}
                    Not => {
                        graph.add_input_param(
                            node_id,
                            "invert".to_string(),
                            DataType::Single(ComplexDataType::SurfaceRuleCondition),
                            ValueType::SurfaceRuleCondition,
                            InputParamKind::ConnectionOnly,
                            true,
                        );
                    }
                    StoneDepth => {
                        input_int(graph, "offset");
                        input_bool(graph, "add_surface_depth");
                        input_int(graph, "secondary_depth_range");
                        graph.add_input_param(
                            node_id,
                            "surface_type".to_string(),
                            DataType::SurfaceType,
                            ValueType::SurfaceType(surface_rule_condition::SurfaceType::Ceiling),
                            InputParamKind::ConstantOnly,
                            true,
                        );
                    }
                }
            }
        }
    }
}

pub struct AllNodeTemplates;
impl NodeTemplateIter for AllNodeTemplates {
    type Item = NodeTemplate;

    fn all_kinds(&self) -> Vec<Self::Item> {
        vec![
            NodeTemplate::ConstantValue,
            NodeTemplate::AddValue,
            NodeTemplate::ConstantBlock,
            NodeTemplate::DensityFunction(DensityFunctionType::Constant),
            NodeTemplate::SurfaceRule(SurfaceRuleType::Sequence),
            NodeTemplate::SurfaceRuleCondition(SurfaceRuleConditionType::VerticalGradient),
            NodeTemplate::Noise,
            NodeTemplate::Reference(WindowType::DensityFunction),
            NodeTemplate::Reference(WindowType::Noise),
        ]
    }
}
