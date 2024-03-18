use crate::{U31ExtConfig, M31};
use bitcoin::ScriptBuf as Script;

pub struct QM31;

impl U31ExtConfig for QM31 {
    type BaseFieldConfig = M31;
    const DEGREE: u32 = 4;

    fn mul_impl() -> Script {
        todo!()
    }
}
