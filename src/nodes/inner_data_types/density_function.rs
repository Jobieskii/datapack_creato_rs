use std::fmt::Display;

use strum::{EnumIter, AsRefStr, EnumCount};

use crate::{ui::ComboBoxEnum, nodes::node_types::NodeTemplate};

use super::{InnerDataType};

#[derive(Copy, Clone, Debug, EnumIter, AsRefStr, EnumCount, PartialEq)]
#[strum(serialize_all = "snake_case")]
pub enum DensityFunctionType {
    Add,
    Constant,
    Mul,
    Noise,
    //TODO: Finish this
}
impl Display for DensityFunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} (Density Function)", self)
    }
}

impl ComboBoxEnum for DensityFunctionType{}

impl InnerDataType for DensityFunctionType {
    fn to_NodeTemplate (&self) -> NodeTemplate {
        NodeTemplate::DensityFunction(*self)
    }
}