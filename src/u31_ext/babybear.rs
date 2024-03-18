use crate::pushable;
use crate::u31::{u31_add, u31_double, u31_mul, u31_sub, BabyBear};
use crate::U31ExtConfig;
use bitcoin::ScriptBuf as Script;
use bitcoin_script::bitcoin_script as script;

pub struct BabyBear4;

impl BabyBear4 {
    fn mul_11() -> Script {
        script! {
            OP_DUP
            { u31_double::<BabyBear>() }
            OP_DUP
            { u31_double::<BabyBear>() }
            { u31_double::<BabyBear>() }
            { u31_add::<BabyBear>() }
            { u31_add::<BabyBear>() }
        }
    }
}

impl U31ExtConfig for BabyBear4 {
    type BaseFieldConfig = BabyBear;
    const DEGREE: u32 = 4;

    fn mul_impl() -> Script {
        script! {
            // split the number as (ah | al) * (bh * bl)

            // compute al * bl, push to the alt stack (lower first in)
            OP_DUP 5 OP_PICK { u31_mul::<Self::BaseFieldConfig>() } OP_TOALTSTACK
            OP_DUP 6 OP_PICK { u31_mul::<Self::BaseFieldConfig>() }
            2 OP_PICK 6 OP_PICK { u31_mul::<Self::BaseFieldConfig>() }
            { u31_add::<Self::BaseFieldConfig>() }
            OP_TOALTSTACK
            OP_OVER 6 OP_PICK { u31_mul::<Self::BaseFieldConfig>() } OP_TOALTSTACK

            // compute ah * bh, push to the alt stack (lower first in)
            2 OP_PICK 7 OP_PICK { u31_mul::<Self::BaseFieldConfig>() } OP_TOALTSTACK
            2 OP_PICK 8 OP_PICK { u31_mul::<Self::BaseFieldConfig>() }
            4 OP_PICK 8 OP_PICK { u31_mul::<Self::BaseFieldConfig>() }
            { u31_add::<Self::BaseFieldConfig>() }
            OP_TOALTSTACK
            3 OP_PICK 8 OP_PICK { u31_mul::<Self::BaseFieldConfig>() } OP_TOALTSTACK

            // compute ah + al and bh + bl
            5 OP_ROLL 7 OP_ROLL { u31_add::<Self::BaseFieldConfig>() }
            5 OP_ROLL 6 OP_ROLL { u31_add::<Self::BaseFieldConfig>() }
            3 OP_ROLL 5 OP_ROLL { u31_add::<Self::BaseFieldConfig>() }
            3 OP_ROLL 4 OP_ROLL { u31_add::<Self::BaseFieldConfig>() }

            // compute (ah + al) * (bh + bl) and push to the alt stack (lower first in)
            OP_DUP 3 OP_PICK { u31_mul::<Self::BaseFieldConfig>() } OP_TOALTSTACK
            3 OP_PICK { u31_mul::<Self::BaseFieldConfig>() }
            OP_OVER 3 OP_ROLL { u31_mul::<Self::BaseFieldConfig>() }
            { u31_add::<Self::BaseFieldConfig>() }
            OP_TOALTSTACK
            { u31_mul::<Self::BaseFieldConfig>() }

            // pull (ah + al) * (bh + bl) out
            OP_FROMALTSTACK
            OP_FROMALTSTACK

            // pull ah * bh out
            OP_FROMALTSTACK
            OP_FROMALTSTACK
            OP_FROMALTSTACK

            // subtract ah * bh from (ah + al) * (bh + bl)
            5 OP_ROLL 3 OP_PICK { u31_sub::<Self::BaseFieldConfig>() }
            5 OP_ROLL 3 OP_PICK { u31_sub::<Self::BaseFieldConfig>() }
            5 OP_ROLL 3 OP_PICK { u31_sub::<Self::BaseFieldConfig>() }

            // pull al * bl out
            OP_FROMALTSTACK
            OP_FROMALTSTACK
            OP_FROMALTSTACK

            // subtract al * bl from (ah + al) * (bh + bl) - ah * bh
            5 OP_ROLL 3 OP_PICK { u31_sub::<Self::BaseFieldConfig>() }
            5 OP_ROLL 3 OP_PICK { u31_sub::<Self::BaseFieldConfig>() }
            5 OP_ROLL 3 OP_PICK { u31_sub::<Self::BaseFieldConfig>() }

            // move al * bl closer, they are the lower three limbs
            3 OP_ROLL 4 OP_ROLL 5 OP_ROLL

            // handle (ah + al) * (bh + bl) - ah * bh - al * bl
            3 OP_ROLL { u31_add::<Self::BaseFieldConfig>() }
            3 OP_ROLL
            4 OP_ROLL { Self::mul_11() } 4 OP_ROLL { u31_add::<Self::BaseFieldConfig>() }

            // handle ah * bh
            4 OP_ROLL { Self::mul_11() } { u31_add::<Self::BaseFieldConfig>() }
            4 OP_ROLL { Self::mul_11() } 4 OP_ROLL { u31_add::<Self::BaseFieldConfig>() }
            4 OP_ROLL { Self::mul_11() } 4 OP_ROLL { u31_add::<Self::BaseFieldConfig>() }

            // reorganize (a4, a1, a2, a3) -> (a4, a3, a2, a1)
            OP_SWAP
            2 OP_ROLL
        }
    }
}
