use std::fmt::Display;

use strum::{AsRefStr, EnumCount, EnumIter};
use strum_macros::EnumString;

use crate::nodes::{data_types::SwitchableInnerValueType, node_types::NodeTemplate};
use crate::ui::ComboBoxEnum;

use super::InnerDataType;

#[derive(Copy, Clone, Debug, EnumIter, AsRefStr, EnumCount, PartialEq)]
#[strum(serialize_all = "snake_case")]
pub enum DensityFunctionType {
    // maker function
    Interpolated,
    FlatCache,
    #[strum(serialize = "cache_2d")]
    Cache2d,
    CacheOnce,
    CacheAllInCell,
    // one argument
    Abs,
    Square,
    Cube,
    HalfNegative,
    QuarterNegative,
    Squeeze,
    // two arguments
    Add,
    Mul,
    Min,
    Max,
    // Other
    BlendAlpha,
    BlendOffset,
    BlendDensity,
    Beardifier,
    OldBlendedNoise,
    Noise,
    EndIslands,
    WeirdScaledSampler,
    ShiftedNoise,
    RangeChoice,
    ShiftA,
    ShiftB,
    Shift,
    Clamp,
    Spline,
    Constant,
    YClampedGradient,
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

#[derive(Copy, Clone, Debug, EnumIter, AsRefStr, EnumCount, PartialEq, EnumString)]
pub enum WeirdScaledSampleRarityValueMapper {
    #[strum(serialize = "type_1")]
    Type1,
    #[strum(serialize = "type_2")]
    Type2,
}

impl ComboBoxEnum for WeirdScaledSampleRarityValueMapper {}
