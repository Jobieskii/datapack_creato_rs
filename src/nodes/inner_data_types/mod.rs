pub mod density_function;
pub mod surface_rule;
pub mod surface_rule_condition;

use std::fmt::Display;

use strum::IntoEnumIterator;

use crate::ui::ComboBoxEnum;

use super::{data_types::SwitchableInnerValueType, node_types::NodeTemplate};

pub trait InnerDataType: AsRef<str> + Display + ComboBoxEnum + IntoEnumIterator {
    #[allow(non_snake_case)]
    fn to_NodeTemplate(&self) -> NodeTemplate;
    #[allow(non_snake_case)]
    fn to_SwitchableInnerValueType(&self) -> SwitchableInnerValueType;

    fn inner_data_type_from(str: &str) -> Option<Self> {
        for x in Self::iter() {
            if x.as_ref() == str {
                return Some(x);
            }
        }
        None
    }
}
