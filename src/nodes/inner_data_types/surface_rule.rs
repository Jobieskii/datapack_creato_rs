use std::fmt::Display;

use strum::{EnumIter, AsRefStr, EnumCount};

use crate::{ui::ComboBoxEnum, nodes::node_types::NodeTemplate};

use super::InnerDataType;

#[derive(Copy, Clone, Debug, EnumIter, AsRefStr, EnumCount, PartialEq)]
pub enum SurfaceRuleType {
    Bandlands,
    Block,
    Sequence,
    Condition
}
impl Display for SurfaceRuleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} (Surface Rule)", self)
    }
}
impl ComboBoxEnum for SurfaceRuleType{}

impl InnerDataType for SurfaceRuleType {
    fn to_NodeTemplate (&self) -> NodeTemplate {
        NodeTemplate::SurfaceRule(*self)
    }
}