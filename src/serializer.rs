use egui_node_graph::NodeId;
use json::{self, JsonValue, object::Object};
use log::warn;

use crate::nodes::{NodeData, GraphType, data_types::{ValueType, DataType}, blocks};

pub trait JsonSerializer {
    fn serialize(&self, node_id: NodeId, graph: &GraphType) -> Option<json::JsonValue>;
    fn deserialize(s: &str, graph: &mut GraphType);
}

impl JsonSerializer for NodeData {
    fn serialize(&self, node_id: NodeId, graph: &GraphType) -> Option<json::JsonValue> {
        let mut o = Object::new();
        let node = graph.nodes.get(node_id)?;
        for (label, in_id) in node.inputs.iter() {
            let input = graph.get_input(*in_id);
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
                DataType::List(_) => todo!(),
                DataType::Single(_) => todo!(),
            };
            if let Some(val) = value {
                o.insert(label, val);
            } else {
                warn!("Error serializing value {} at node: {:?} ({})", label, node_id, node.label);
            }
        }
        Some(JsonValue::Object(o))
    }

    fn deserialize(s: &str, graph: &mut GraphType) {
        todo!()
    }
}