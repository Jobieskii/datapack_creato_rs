use std::fmt::Display;

use strum::{EnumIter, AsRefStr, EnumCount};

use crate::{nodes::node_types::NodeTemplate, ui::ComboBoxEnum};

use super::InnerDataType;

#[derive(Copy, Clone, Debug, EnumIter, AsRefStr, EnumCount, PartialEq)]
pub enum SurfaceRuleConditionType {
    Biome, 
    NoiseThreshold, 
    VerticalGradient, 
    YAbove, 
    Water, 
    Temperature, 
    Steep, 
    Not, 
    Hole, 
    AbovePreliminarySurface, 
    StoneDepth
}
impl Display for SurfaceRuleConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} (Surface Rule Condition)", self)
    }
}

impl ComboBoxEnum for SurfaceRuleConditionType {}

impl InnerDataType for SurfaceRuleConditionType {
    fn to_NodeTemplate (&self) -> NodeTemplate {
        NodeTemplate::SurfaceRuleCondition(*self)
    }
}