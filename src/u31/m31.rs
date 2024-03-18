use crate::u31::U31Config;

pub struct M31;
impl U31Config for M31 {
    const MOD: u32 = (1 << 31) - 1;
}
