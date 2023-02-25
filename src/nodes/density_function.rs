use std::fmt::Display;

use strum::{EnumIter, AsRefStr, EnumCount};

#[derive(Copy, Clone, Debug, EnumIter, AsRefStr, EnumCount)]
pub enum DensityFunctionType {
    Add,
    Constant,
    Mul,
    Noise
}
impl Display for DensityFunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} (Density Function)", self)
    }
}