pub mod surface_rule_condition;
pub mod density_function;
pub mod surface_rule;

use std::fmt::Display;

use strum::IntoEnumIterator;

use crate::ui::ComboBoxEnum;

use super::{node_types::NodeTemplate, data_types::SwitchableInnerValueType};

pub trait InnerDataType : AsRef<str> + Display + ComboBoxEnum + IntoEnumIterator {
    fn to_NodeTemplate (&self) -> NodeTemplate;
    fn to_SwitchableInnerValueType (&self) -> SwitchableInnerValueType;

    fn inner_data_type_from(str: &str) -> Option<Self> {
        for x in Self::iter() {
            if x.as_ref() == str {
                return Some(x)
            }
        }
        None
    }
}

