use std::collections::HashSet;

use egui_node_graph::{NodeId, InputParam, InputId};
use json::{self, JsonValue, object::Object};
use log::warn;

use crate::{nodes::{NodeData, GraphType, data_types::{ValueType, DataType, ComplexDataType, SwitchableInnerValueType}, blocks, node_types::NodeTemplate}, window::{WindowType, Window}};
impl Window {
    pub fn serialize(&self) -> Option<json::JsonValue> {
        self.serialize_inner(self.root_node, &mut HashSet::new())
    }
    
    fn serialize_inner(&self, node_id: NodeId, visited: &mut HashSet<NodeId>) -> Option<JsonValue>{
        if visited.contains(&node_id) {
            warn!("Circular graph cannot be serialized! {:?}", node_id);
            return None
        }
        visited.insert(node_id);
        let graph = &self.state.graph;
        let node = graph.nodes.get(node_id)?;
    
        match node.user_data.template {
            NodeTemplate::Output(WindowType::DensityFunction) => {
                let (_,in_id) = node.inputs.first().unwrap();
                if let Some(out_id) = graph.connection(*in_id) {
                    let next_node = graph.get_output(out_id).node;
                    return self.serialize_inner(next_node, visited)
                } else {
                    warn!("Nothing connected to output!");
                    return None
                }
            },
            _ => {}
        }
        let mut o = Object::new();
    
        for (n, (label, in_id)) in node.inputs.iter().enumerate() {
            let input = graph.get_input(*in_id);
        
            if let DataType::List(_) = input.typ {
                let rest = node.inputs.iter()
                    .skip(n)
                    .filter_map(|(_,o_id)| {
                    let o_input = graph.get_input(*o_id);
                    self.input_to_json_value(o_id, o_input, visited)
                }).collect();
                o.insert(label, JsonValue::Array(rest));
                break;
        
            } else {
                let value = self.input_to_json_value(in_id, input, visited);
                if let Some(val) = value {
                    o.insert(label, val);
                } else {
                    warn!("Error serializing value {} at node: {:?} ({})", label, node_id, node.label);
                    return None
                }
            };
        }
        Some(JsonValue::Object(o))
    }
    
    pub fn deserialize(&mut self, s: &JsonValue, window_type: WindowType, graph: &mut GraphType) {
        match window_type {
            WindowType::DensityFunction => {
                todo!();
            },
            WindowType::Noise => {
                
            },
        }
    }
    
    fn input_to_json_value(&self, in_id: &InputId, input: &InputParam<DataType, ValueType>, visited: &mut HashSet<NodeId>) -> Option<JsonValue>{
        let graph = &self.state.graph;
        let connected_node = graph.connection(*in_id);
        if let Some(out_id) = connected_node {
            let next_node = graph.get_output(out_id).node;
            self.serialize_inner(next_node, visited)
    
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
            // This should never happen
            DataType::List(_) => unimplemented!(),
            // Not a blanket `unimplemented!` as we may want to add some cdt that has constant type of input, a.k.a non empty ValueType.
            DataType::Single(cdt) => match cdt {
                ComplexDataType::DensityFunction => Some(JsonValue::from(0.0)),
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
}
