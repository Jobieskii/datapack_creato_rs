use egui_node_graph::{NodeId, InputParam};
use json::{self, JsonValue, object::Object};
use log::warn;

use crate::nodes::{NodeData, GraphType, data_types::{ValueType, DataType}, blocks};

pub fn serialize(node_id: NodeId, graph: &GraphType) -> Option<json::JsonValue> {
    let mut o = Object::new();
    let node = graph.nodes.get(node_id)?;
    for (label, in_id) in node.inputs.iter() {
        let input = graph.get_input(*in_id);
        let connected_node = graph.connection(*in_id);
        let value = if let Some(out_id) = connected_node {
            let next_node = graph.get_output(out_id).node;
            serialize(node_id, graph)
        } else {
            input_to_json_value(input)
        };
        if let Some(val) = value {
            o.insert(label, val);
        } else {
            warn!("Error serializing value {} at node: {:?} ({})", label, node_id, node.label);
        }
    }
    Some(JsonValue::Object(o))
}

pub fn deserialize(s: &str, graph: &mut GraphType) {
    todo!()
}

fn input_to_json_value(input: &InputParam<DataType, ValueType>) -> Option<JsonValue>{
    let value = match input.typ {
        DataType::Value => {
            if let ValueType::Value(x) = input.value() {
                Some(JsonValue::from(*x))
            } else { None }
        },
        DataType::Block => {
            if let ValueType::Block(x) = input.value() {
                Some(JsonValue::String(blocks::BLOCK_LIST[*x].id.to_string()))
            } else { None }
        },
        DataType::ValuesArray => {
            if let ValueType::ValuesArray(x) = input.value() {
                Some(JsonValue::Array(
                    x.iter()
                        .map(|val| JsonValue::Number((*val).into()))
                        .collect::<Vec<_>>()
                    ))
            } else { None }
        },
        DataType::List(dt) => match dt {
            crate::nodes::data_types::ComplexDataType::Noise => todo!(),
            crate::nodes::data_types::ComplexDataType::DensityFunction => todo!(),
            crate::nodes::data_types::ComplexDataType::SurfaceRule => todo!(),
            crate::nodes::data_types::ComplexDataType::SurfaceRuleCondition => todo!(),
            crate::nodes::data_types::ComplexDataType::Reference(_) => todo!(),
        },
        DataType::Single(_) => todo!(),
        DataType::SurfaceRuleType => todo!(),
    };
    value
}