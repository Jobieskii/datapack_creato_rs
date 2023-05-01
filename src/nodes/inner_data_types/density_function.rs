use std::fmt::Display;

use strum::{AsRefStr, EnumCount, EnumIter};

use crate::nodes::{data_types::SwitchableInnerValueType, node_types::NodeTemplate};
use crate::ui::ComboBoxEnum;

use super::InnerDataType;

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

impl ComboBoxEnum for DensityFunctionType {}

impl InnerDataType for DensityFunctionType {
    fn to_NodeTemplate(&self) -> NodeTemplate {
        NodeTemplate::DensityFunction(*self)
    }

    fn to_SwitchableInnerValueType(&self) -> crate::nodes::data_types::SwitchableInnerValueType {
        SwitchableInnerValueType::DensityFunction(*self)
    }
}
