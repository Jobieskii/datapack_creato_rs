use std::fmt::Display;

use strum::{AsRefStr, EnumCount, EnumIter};

use crate::nodes::{data_types::SwitchableInnerValueType, node_types::NodeTemplate};
use crate::ui::ComboBoxEnum;

use super::InnerDataType;

#[derive(Copy, Clone, Debug, EnumIter, AsRefStr, EnumCount, PartialEq)]
#[strum(serialize_all = "snake_case")]
pub enum SurfaceRuleType {
    Bandlands,
    Block,
    Sequence,
    Condition,
}
impl Display for SurfaceRuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} (Surface Rule)", self)
    }
}
impl ComboBoxEnum for SurfaceRuleType {}

impl InnerDataType for SurfaceRuleType {
    fn to_NodeTemplate(&self) -> NodeTemplate {
        NodeTemplate::SurfaceRule(*self)
    }

    fn to_SwitchableInnerValueType(&self) -> SwitchableInnerValueType {
        SwitchableInnerValueType::SurfaceRule(*self)
    }
}
