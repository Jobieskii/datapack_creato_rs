use egui_node_graph::{NodeId, InputParam, InputId};
use json::{self, JsonValue, object::Object};
use log::warn;

use crate::{nodes::{NodeData, GraphType, data_types::{ValueType, DataType, ComplexDataType, SwitchableInnerValueType}, blocks, node_types::NodeTemplate}, window::WindowType};

pub fn serialize(node_id: NodeId, graph: &GraphType) -> Option<json::JsonValue> {
    let node = graph.nodes.get(node_id)?;
    match node.user_data.template {
        NodeTemplate::Output(WindowType::DensityFunction) => {
            let (_,in_id) = node.inputs.first().unwrap();
            if let Some(out_id) = graph.connection(*in_id) {
                let next_node = graph.get_output(out_id).node;
                return serialize(next_node, graph)
            }
        },
        _ => {}
    }
    let mut o = Object::new();

    for (n, (label, in_id)) in node.inputs.iter().enumerate() {
        let input = graph.get_input(*in_id);
        let lowercase_label = &*label.to_lowercase();
    
        if let DataType::List(_) = input.typ {
            let rest = node.inputs.iter()
                .skip(n)
                .filter_map(|(_,o_id)| {
                let o_input = graph.get_input(*o_id);
                input_to_json_value(o_id, o_input, graph)
            }).collect();
            o.insert(lowercase_label, JsonValue::Array(rest));
            break;
    
        } else {
            let value = input_to_json_value(in_id, input, graph);
            if let Some(val) = value {
                o.insert(lowercase_label, val);
            } else {
                warn!("Error serializing value {} at node: {:?} ({})", label, node_id, node.label);
            }
        };
    }
    Some(JsonValue::Object(o))
}

pub fn deserialize(s: &str, graph: &mut GraphType) {
    todo!()
}

fn input_to_json_value(in_id: &InputId, input: &InputParam<DataType, ValueType>, graph: &GraphType) -> Option<JsonValue>{
    let connected_node = graph.connection(*in_id);
    if let Some(out_id) = connected_node {
        let next_node = graph.get_output(out_id).node;
        serialize(next_node, graph)

    } else { match input.typ {
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
        DataType::List(dt) => unimplemented!(),
        DataType::Single(dt) => match dt {
            _ => unimplemented!()
        },
        DataType::ValueTypeSwitcher => {
            if let ValueType::InnerTypeSwitch(x) = input.value() {
                let val = match x {
                    SwitchableInnerValueType::SurfaceRule(y) => y.as_ref(),
                    SwitchableInnerValueType::SurfaceRuleCondition(y) => y.as_ref(),
                    SwitchableInnerValueType::DensityFunction(y) => y.as_ref(),
                };
                Some(JsonValue::String(val.to_string()))
            } else { None }
        },
        DataType::Reference(_) => {
            if let ValueType::Reference(_, name) = input.value() {
                Some(JsonValue::String(name.clone()))
            } else { None }
        },
    }
    }
}