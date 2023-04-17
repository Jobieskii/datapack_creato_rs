use std::collections::HashSet;

use eframe::epaint::{Pos2, Vec2};
use egui_node_graph::{NodeId, InputParam, InputId, Node, InputParamKind};
use json::{self, JsonValue, object::Object};
use log::{warn, error};

use crate::{nodes::{NodeData, GraphType, data_types::{ValueType, DataType, ComplexDataType, SwitchableInnerValueType, self, increase_node_list_length}, blocks, node_types::NodeTemplate, inner_data_types::{density_function::DensityFunctionType, InnerDataType, surface_rule::SurfaceRuleType, surface_rule_condition::SurfaceRuleConditionType}, add_node, rebuild_node}, window::{WindowType, Window}, errors::AppError, app::App};
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
    
    pub fn deserialize(&mut self, s: &JsonValue) {
        let root_id = self.root_node;
        let root = self.state.graph.nodes.get(root_id).unwrap();

        let (label, input_id) = &self.state.graph.nodes.get(root_id).unwrap().inputs[0];
        // If root node has only `out` input, create a new node and connect
        if label == "out" {
            let template = self.state.graph.get_input(*input_id).typ.defualt_NodeTemplate();
            let next = add_node(&mut self.state, &mut self.user_state, template, Pos2::ZERO);
            let input_id = self.state.graph.nodes.get(root_id).unwrap().inputs[0].1;
            let output_id = self.state.graph.nodes.get(next).unwrap().outputs.last().unwrap().1;
            self.state.graph.add_connection(output_id, input_id);
        } else {
            self.deserialize_inner(s, &root_id);
        }
    }
    fn deserialize_inner(&mut self, s: &JsonValue, node_id: &NodeId) {
        let root = self.state.graph.nodes.get(*node_id).unwrap();
        if let Some((_, entry)) = s.entries().find(|(label, _)| *label == "type") {
            if let Ok(ValueType::InnerTypeSwitch(value_type))
             = Self::json_value_to_value_type(entry, &DataType::ValueTypeSwitcher, &root.user_data.template) 
            {
                rebuild_node(*node_id, &mut self.state.graph, &mut self.user_state, value_type.to_NodeTemplate())
            }
        }

        let root = self.state.graph.nodes.get(*node_id).unwrap().clone();
        let template = &root.user_data.template;
        for (entry, json_value) in s.entries() {
            if let Ok(input_id) = root.get_input(entry) {
                let input = self.state.graph.get_input(input_id).clone();

                if json_value.is_object() {
                    
                    let curr_pos = *self.state.node_positions.get(*node_id).unwrap();
                    let next = add_node(
                        &mut self.state, 
                        &mut self.user_state, 
                        input.typ.defualt_NodeTemplate(), 
                        curr_pos + Vec2::new(-5., 5.)
                    );
                    let output_id = self.state.graph.nodes.get(next).unwrap().outputs.last().unwrap().1;
                    
                    self.state.graph.add_connection(output_id, input_id);
                    
                    self.deserialize_inner(json_value, &next);
                } 
                else if json_value.is_array() {
                    if let DataType::ValuesArray = input.typ {
                        if let Ok(value) = Self::json_value_to_value_type(json_value, &DataType::ValuesArray, template) {
                            let input_mut = self.state.graph.inputs.get_mut(input_id).unwrap();
                            input_mut.value = value;
                        }
                    } else if let DataType::List(x) = input.typ {
                        if let Ok(value) = Self::json_value_to_value_type(json_value, &DataType::List(x), template) {
                            let input_mut = self.state.graph.inputs.get_mut(input_id).unwrap();
                            input_mut.value = value;

                            for item in json_value.members() {
                                let data_type = DataType::Single(x);
                                increase_node_list_length(&mut self.state.graph, *node_id);

                                let curr_pos = *self.state.node_positions.get(*node_id).unwrap();

                                let next = add_node(
                                    &mut self.state, 
                                    &mut self.user_state, 
                                    data_type.defualt_NodeTemplate(), 
                                    curr_pos + Vec2::new(-5., 5.)
                                );
                                let output_id = self.state.graph.nodes.get(next).unwrap().outputs.last().unwrap().1;
                                let input_id = self.state.graph.nodes.get(*node_id).unwrap().inputs.last().unwrap().1;
                                self.state.graph.add_connection(output_id, input_id);

                                self.deserialize_inner(item, &next);
                            }
                        }
                    }
                } 
                else {
                    if let Ok(value) = Self::json_value_to_value_type(json_value, &input.typ, template) {
                        let input_mut = self.state.graph.inputs.get_mut(input_id).unwrap();
                        input_mut.value = value;
                    }
                }
            } else {
                error!("Wrong json data! \n{}", s);
            }
        }
    }
    /// Returns the `ValueType` variant and nothing else (complex data types must be taken care of elsewhere)
    ///  - for List returns `ValueType::List(N)`, N = Length of json array
    ///  - for Complex returns equivalent `ValueType`
    ///  - TODO: for Block
    /// This method is not recursive.
    fn json_value_to_value_type(value: &JsonValue, data_type: &DataType, node_type: &NodeTemplate) -> Result<ValueType, AppError> {
        match data_type {
            DataType::Value => {
                let value = value.as_f32().ok_or(AppError::JsonError(json::Error::wrong_type("f32")))?;
                Ok(ValueType::Value(value))
            },
            DataType::Block => {
                // TODO: internal represenation of a block needs to be rethinked.
                todo!()
            },
            DataType::ValuesArray => {
                if !value.is_array() {
                    Err(AppError::JsonError(json::Error::wrong_type("Vec<f32>")))
                } else {
                    if value.members().any(|val| val.as_f32().is_none()) {
                        Err(AppError::JsonError(json::Error::wrong_type("Vec<f32>")))
                    } else {
                        Ok(ValueType::ValuesArray(
                            value.members()
                            .filter_map(|val| val.as_f32())
                            .collect()
                        ))
                    }
                }
            },
            DataType::Reference(x) => {
                let value = value.as_str().ok_or(AppError::JsonError(json::Error::wrong_type("str")))?;
                Ok(ValueType::Reference(*x, value.to_string()))
            },
            DataType::ValueTypeSwitcher => {
                let value = value.as_str().ok_or(AppError::JsonError(json::Error::wrong_type("str")))?;
                match node_type {
                    NodeTemplate::DensityFunction(x) => {
                        if let Some(typ) = DensityFunctionType::inner_data_type_from(value) {
                            Ok(ValueType::InnerTypeSwitch(typ.to_SwitchableInnerValueType()))
                        } else {
                            Err(AppError::WrongData(value.to_string()))
                        }
                    },
                    NodeTemplate::SurfaceRule(x) => {
                        if let Some(typ) = SurfaceRuleType::inner_data_type_from(value) {
                            Ok(ValueType::InnerTypeSwitch(typ.to_SwitchableInnerValueType()))
                        } else {
                            Err(AppError::WrongData(value.to_string()))
                        }
                    },
                    NodeTemplate::SurfaceRuleCondition(x) => {
                        if let Some(typ) = SurfaceRuleConditionType::inner_data_type_from(value) {
                            Ok(ValueType::InnerTypeSwitch(typ.to_SwitchableInnerValueType()))
                        } else {
                            Err(AppError::WrongData(value.to_string()))
                        }
                    },
                    //TODO: REMEMBER TO ADD ALL NEW NODE TYPES HERE IF NECESSARY
                    _ => unimplemented!()
                }
            },
            DataType::List(_) => {
                if value.is_array() {
                    let value = value.len();
                    Ok(ValueType::List(value as i32))
                } else { Err(AppError::WrongData("Arr".to_string())) }
            },
            DataType::Single(x) => {
                Ok(match x {
                    ComplexDataType::Noise => ValueType::Noise,
                    ComplexDataType::DensityFunction => ValueType::DensityFunction,
                    ComplexDataType::SurfaceRule => ValueType::SurfaceRule,
                    ComplexDataType::SurfaceRuleCondition => ValueType::SurfaceRuleCondition,
                })
            },
        }
    }
    /// This method recursively calls `self.serialize_inner()`.
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
