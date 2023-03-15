pub mod surface_rule_condition;
pub mod density_function;
pub mod surface_rule;

use std::fmt::Display;

use crate::ui::ComboBoxEnum;

use super::node_types::NodeTemplate;

pub trait InnerDataType : AsRef<str> + Display + ComboBoxEnum {
    fn to_NodeTemplate (&self) -> NodeTemplate;
}