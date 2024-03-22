use crate::{U31ExtConfig, M31};
use bitcoin::ScriptBuf as Script;
use bitcoin_script::bitcoin_script as script;

pub struct QM31;

impl U31ExtConfig for QM31 {
    type BaseFieldConfig = M31;
    const DEGREE: u32 = 4;

    fn mul_impl() -> Script {
        todo!()
    }
}

// A    2 elements
// Bj   2 elements
// C    2 elements
// Dj   2 elements

// (A + Bj) * (C + Dj)
// Compute:
//   A * C
//   (A * D + B * C) j
//   B * D * (2 + i)

// (A + B) * (C + D) = A * C + A * D + B * C + B * D

// first compute A * C
// A = (a + bi)
// C = (c + di)
//
// ac + (ad + bc)i - bd