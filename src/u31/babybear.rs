use crate::u31::U31Config;

pub struct BabyBear;
impl U31Config for BabyBear {
    const MOD: u32 = 15 * (1 << 27) + 1;
}
