use std::borrow::Cow;

use egui_node_graph::{NodeTemplateTrait, NodeId, InputParamKind, NodeTemplateIter};

use crate::file::WindowType;

use super::{NodeData, data_types::{DataType, ValueType}, GraphState, GraphType, density_function::DensityFunctionType};


#[derive(Copy, Clone)]
pub enum NodeTemplate {
    ConstantValue,
    AddValue,
    DensityFunction(DensityFunctionType),
    ConstantBlock,
    Noise,
    Reference(WindowType),
    Output(WindowType)
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
            NodeTemplate::DensityFunction(x) => Cow::Owned(x.to_string()),
            NodeTemplate::Noise => Cow::Borrowed("Noise"),
            NodeTemplate::Reference(x) => Cow::Owned(format!("Reference ({})", x.as_ref())),
            NodeTemplate::Output(x) => Cow::Owned(format!("Output ({})", x.as_ref())),
        }
    }

    fn node_graph_label(&self, user_state: &mut Self::UserState) -> String {
        self.node_finder_label(user_state).to_string()
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
                true
            );
        };
        let output_value = |graph: &mut GraphType, name: &str| {
            graph.add_output_param(
                node_id, 
                name.to_string(), 
                DataType::Value
            );
        };
        let input_block = |graph: &mut GraphType, name: &str, kind: InputParamKind| {
            graph.add_input_param(
                node_id, 
                name.to_string(), 
                DataType::Block, 
                ValueType::Block(0), 
                kind, 
                true
            );
        };
        let output_block = |graph: &mut GraphType, name: &str| {
            graph.add_output_param(node_id, name.to_string(), DataType::Block);
        };
        let input_df = |graph: &mut GraphType, name: &str| {
            graph.add_input_param(
                node_id, 
                name.to_string(), 
                DataType::DensityFunction,
                ValueType::DensityFunction,
                InputParamKind::ConnectionOnly, 
                true
            );
        };
        let output_df = |graph: &mut GraphType, name: &str| {
            graph.add_output_param(node_id, name.to_string(), DataType::DensityFunction);
        };
        let input_noise = |graph: &mut GraphType, name: &str| {
            graph.add_input_param(node_id, name.to_string(), DataType::Noise, ValueType::Noise, InputParamKind::ConnectionOnly, true);
        };
        let output_noise = |graph: &mut GraphType, name: &str| {
            graph.add_output_param(node_id, name.to_string(), DataType::Noise);
        };
        let input_values_arr = |graph: &mut GraphType, name: &str| {
            graph.add_input_param(
                node_id, 
                name.to_string(), 
                DataType::ValuesArray, 
                ValueType::ValuesArray(vec![0.]), 
                InputParamKind::ConstantOnly, 
                true
            );
        };
        let input_reference = |graph: &mut GraphType, name: &str, kind: InputParamKind, window_type: &WindowType| {
            graph.add_input_param(node_id, name.to_string(), DataType::Reference(*window_type), ValueType::Reference(*window_type, String::new()), kind, true);
        };
        match self {
            NodeTemplate::ConstantValue => {
                input_value(graph, "value", InputParamKind::ConstantOnly);
                output_value(graph, "out");
            },
            NodeTemplate::AddValue => {
                input_value(graph, "arg1", InputParamKind::ConnectionOrConstant);
                input_value(graph, "arg2", InputParamKind::ConnectionOrConstant);
                output_value(graph, "out");
            }
            NodeTemplate::ConstantBlock => {
                input_block(graph, "block", InputParamKind::ConstantOnly);
                output_block(graph, "out");
            },
            NodeTemplate::DensityFunction(DensityFunctionType::Add) => {
                input_df(graph, "arg1");
                input_df(graph, "arg2");
                output_df(graph, "out");
            },
            NodeTemplate::DensityFunction(DensityFunctionType::Constant) => {
                input_value(graph, "value", InputParamKind::ConstantOnly);
                output_df(graph, "out");
            }
            NodeTemplate::DensityFunction(DensityFunctionType::Mul) => {
                input_df(graph, "arg1");
                input_df(graph, "arg2");
                output_df(graph, "out");
            }
            NodeTemplate::DensityFunction(DensityFunctionType::Noise) => {
                input_noise(graph, "noise");
                input_value(graph, "XZ scale", InputParamKind::ConnectionOrConstant);
                input_value(graph, "Y scale", InputParamKind::ConnectionOrConstant);
                output_df(graph, "out");
            }
            NodeTemplate::Noise => {
                input_value(graph, "First octave", InputParamKind::ConstantOnly);
                input_values_arr(graph, "Amplitudes");
                output_noise(graph, "out");
            },
            NodeTemplate::Reference(x) => {
                input_reference(graph, "Reference", InputParamKind::ConstantOnly, x);
                match x {
                    WindowType::DensityFunction => output_df(graph, "out"),
                    WindowType::Noise => output_noise(graph, "out"),
                }
            },
            NodeTemplate::Output(x) => match x {
                WindowType::DensityFunction => {
                    input_df(graph, "output");
                },
                WindowType::Noise => {
                    input_value(graph, "First octave", InputParamKind::ConstantOnly);
                    input_values_arr(graph, "Amplitudes");
                },
            },
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
            NodeTemplate::DensityFunction(DensityFunctionType::Add),
            NodeTemplate::DensityFunction(DensityFunctionType::Constant),
            NodeTemplate::DensityFunction(DensityFunctionType::Mul),
            NodeTemplate::DensityFunction(DensityFunctionType::Noise),
            NodeTemplate::Noise,
            NodeTemplate::Reference(WindowType::DensityFunction),
            NodeTemplate::Reference(WindowType::Noise)
        ]
    }
}