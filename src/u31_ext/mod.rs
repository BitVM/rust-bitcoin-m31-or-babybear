use crate::{u31_add_v31, u31_mul_by_constant, u31_to_bits, u31_to_v31, unroll, v31_add, v31_double};
use bitvm::treepp::*;

mod babybear;
pub use babybear::*;

mod m31;
pub use m31::*;

mod karatsuba;
pub use karatsuba::*;

mod karatsuba_complex;
pub use karatsuba_complex::*;

use crate::u31::{u31_add, u31_double, u31_sub, U31Config};

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
        // push a, b to altstack
        OP_SWAP OP_TOALTSTACK OP_SWAP OP_TOALTSTACK

        // push d, c to altstack
        OP_SWAP OP_TOALTSTACK OP_SWAP OP_TOALTSTACK

        { u31_to_v31::<C::BaseFieldConfig>() }

        // create a precomputed table (30 times)
        OP_DUP { v31_double::<C::BaseFieldConfig>() }
        OP_2DUP { v31_add::<C::BaseFieldConfig>() }

        for _ in 1..15 {
            OP_OVER { v31_double::<C::BaseFieldConfig>() }
            OP_DUP { v31_double::<C::BaseFieldConfig>() }
            OP_2DUP { v31_add::<C::BaseFieldConfig>() }
        }

        OP_OVER { v31_double::<C::BaseFieldConfig>() }

        // now the stack looks like:
        //    2^0 e
        //    2^1 e
        //    (2^1 + 2^0) e
        //    2^2 e
        //    2^3 e
        //    (2^3 + 2^2) e
        //    ...
        //    2^28 e
        //    2^29 e
        //    (2^29 + 2^28) e
        //    2^30 e

        // leave some stack space
        { 0 } { 0 } { 0 } { 0 } { 0 }

        for i in 0..4 {
            OP_FROMALTSTACK { u31_to_bits() }
            for _ in 0..30 {
                OP_TOALTSTACK
            }

            OP_IF
                { 5 } OP_PICK
                { u31_add_v31::<C::BaseFieldConfig>() }
            OP_ENDIF

            { 6 }

            for _ in 0..15 {
                OP_FROMALTSTACK
                OP_FROMALTSTACK
                2 OP_PICK OP_TOALTSTACK
                OP_IF
                    OP_NOTIF
                        2 OP_ADD
                    OP_ENDIF
                OP_ELSE
                    OP_IF
                        OP_1ADD
                    OP_ELSE
                        OP_DROP { 4 }
                    OP_ENDIF
                OP_ENDIF
                OP_PICK
                { u31_add_v31::<C::BaseFieldConfig>() }
                OP_FROMALTSTACK
                3 OP_ADD
            }

            OP_DROP
            if i != 3 {
                3 OP_ROLL
            }
        }

        OP_TOALTSTACK OP_TOALTSTACK OP_TOALTSTACK OP_TOALTSTACK

        OP_DROP
        for _ in 0..23 {
            OP_2DROP
        }

        OP_FROMALTSTACK OP_FROMALTSTACK OP_FROMALTSTACK OP_FROMALTSTACK
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