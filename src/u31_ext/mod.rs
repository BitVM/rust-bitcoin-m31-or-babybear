use crate::{u31_mul_by_constant, u31_to_bits, unroll};
use bitvm::treepp::*;

mod babybear;
pub use babybear::*;

mod m31;
pub use m31::*;

mod karatsuba;
pub use karatsuba::*;

mod karatsuba_complex;
pub use karatsuba_complex::*;

use crate::u31::{u31_add, u31_double, u31_mul_common, u31_sub, U31Config};

pub trait U31ExtConfig {
    type BaseFieldConfig: U31Config;
    const DEGREE: u32;

    fn mul_impl() -> Script;
}

pub fn u31ext_add<C: U31ExtConfig>() -> Script {
    script! {
        { unroll(C::DEGREE - 1, |i| {
            let gap = C::DEGREE - i;
            script!{
                { gap } OP_ROLL
                { u31_add::<C::BaseFieldConfig>() }
                OP_TOALTSTACK
        }}) }
        { u31_add::<C::BaseFieldConfig>() }
        { unroll(C::DEGREE - 1, |_| script!{ OP_FROMALTSTACK }) }
    }
}

pub fn u31ext_equalverify<C: U31ExtConfig>() -> Script {
    script! {
        { unroll(C::DEGREE - 1, |i| {
            let gap = C::DEGREE - i;
            script!{
                { gap } OP_ROLL
                OP_EQUALVERIFY
        }}) }
        OP_EQUALVERIFY
    }
}

pub fn u31ext_sub<C: U31ExtConfig>() -> Script {
    script! {
        { unroll(C::DEGREE - 1, |i| {
            let gap = C::DEGREE - i;
            script!{
                { gap } OP_ROLL OP_SWAP
                { u31_sub::<C::BaseFieldConfig>() }
                OP_TOALTSTACK
        }}) }
        { u31_sub::<C::BaseFieldConfig>() }
        { unroll(C::DEGREE - 1, |_| script!{ OP_FROMALTSTACK }) }
    }
}

pub fn u31ext_double<C: U31ExtConfig>() -> Script {
    script! {
        { unroll(C::DEGREE - 1, |_|
            script! {
                { u31_double::<C::BaseFieldConfig>() }
                OP_TOALTSTACK
        })}
        { u31_double::<C::BaseFieldConfig>() }
        { unroll(C::DEGREE - 1, |_| script!{ OP_FROMALTSTACK }) }
    }
}

pub fn u31ext_mul<C: U31ExtConfig>() -> Script {
    C::mul_impl()
}

pub fn u31ext_mul_u31<C: U31ExtConfig>() -> Script {
    // input stack:
    //
    // u31ext
    // d, c, b, a
    //
    // u31
    // e

    script! {
        { u31_to_bits() }

        // duplicate 3 times
        for _ in 0..31 {
            30 OP_PICK
        }
        for _ in 0..31 {
            OP_TOALTSTACK
        }

        for _ in 0..31 {
            30 OP_PICK
        }
        for _ in 0..31 {
            OP_TOALTSTACK
        }

        for _ in 0..31 {
            30 OP_PICK
        }
        for _ in 0..31 {
            OP_TOALTSTACK
        }

        for _ in 0..31 {
            OP_TOALTSTACK
        }

        // d
        3 OP_ROLL
        { u31_mul_common::<C::BaseFieldConfig>() }

        // c
        3 OP_ROLL
        { u31_mul_common::<C::BaseFieldConfig>() }

        // b
        3 OP_ROLL
        { u31_mul_common::<C::BaseFieldConfig>() }

        // a
        3 OP_ROLL
        { u31_mul_common::<C::BaseFieldConfig>() }
    }
}

pub fn u31ext_mul_u31_by_constant<C: U31ExtConfig>(constant: u32) -> Script {
    // input stack:
    //
    // u31ext
    // d, c, b, a

    script! {
        OP_TOALTSTACK OP_TOALTSTACK OP_TOALTSTACK
        { u31_mul_by_constant::<C::BaseFieldConfig>(constant) }
        OP_FROMALTSTACK
        { u31_mul_by_constant::<C::BaseFieldConfig>(constant) }
        OP_FROMALTSTACK
        { u31_mul_by_constant::<C::BaseFieldConfig>(constant) }
        OP_FROMALTSTACK
        { u31_mul_by_constant::<C::BaseFieldConfig>(constant) }
    }
}

pub fn u31ext_toaltstack<C: U31ExtConfig>() -> Script {
    script! {
        for _ in 0..C::DEGREE {
            OP_TOALTSTACK
        }
    }
}

pub fn u31ext_fromaltstack<C: U31ExtConfig>() -> Script {
    script! {
        for _ in 0..C::DEGREE {
            OP_FROMALTSTACK
        }
    }
}

pub fn u31ext_copy<C: U31ExtConfig>(offset: usize) -> Script {
    let a = offset * (C::DEGREE as usize) + (C::DEGREE as usize) - 1;

    script! {
        for _ in 0..C::DEGREE {
            { a } OP_PICK
        }
    }
}

pub fn u31ext_roll<C: U31ExtConfig>(offset: usize) -> Script {
    let a = offset * (C::DEGREE as usize) + (C::DEGREE as usize) - 1;

    script! {
        for _ in 0..C::DEGREE {
            { a } OP_ROLL
        }
    }
}
