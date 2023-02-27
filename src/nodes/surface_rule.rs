use std::fmt::Display;

use strum::{EnumIter, AsRefStr, EnumCount};

#[derive(Copy, Clone, Debug, EnumIter, AsRefStr, EnumCount)]
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

#[derive(Copy, Clone, Debug, EnumIter, AsRefStr, EnumCount)]
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