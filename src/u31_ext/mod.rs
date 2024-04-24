use bitcoin::opcodes::Ordinary::{OP_FROMALTSTACK, OP_ROLL, OP_TOALTSTACK};
use crate::{pushable, u31_add_v31, u31_to_bits, u31_to_v31, unroll};
use bitcoin::ScriptBuf as Script;
use bitcoin_script::script;

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
    // a, b, c, d
    //
    // u31
    // e

    script! {
        // push d, c to altstack
        OP_ROT OP_TOALTSTACK OP_TOALTSTACK

        // push b, a to altstack
        OP_ROT OP_TOALTSTACK OP_TOALTSTACK

        // create a precomputed table (29 times)
        OP_DUP { u31_double() }
        OP_DUP { u31_double() }
        OP_DUP { u31_double() }
        OP_DUP { u31_double() }
        OP_DUP { u31_double() }

        OP_DUP { u31_double() }
        OP_DUP { u31_double() }
        OP_DUP { u31_double() }
        OP_DUP { u31_double() }
        OP_DUP { u31_double() }

        OP_DUP { u31_double() }
        OP_DUP { u31_double() }
        OP_DUP { u31_double() }
        OP_DUP { u31_double() }
        OP_DUP { u31_double() }

        OP_DUP { u31_double() }
        OP_DUP { u31_double() }
        OP_DUP { u31_double() }
        OP_DUP { u31_double() }
        OP_DUP { u31_double() }

        OP_DUP { u31_double() }
        OP_DUP { u31_double() }
        OP_DUP { u31_double() }
        OP_DUP { u31_double() }
        OP_DUP { u31_double() }

        OP_DUP { u31_double() }
        OP_DUP { u31_double() }
        OP_DUP { u31_double() }
        OP_DUP { u31_double() }
        OP_DUP { u31_double() }

        // now the stack looks like:
        //    2^0 e
        //    2^1 e
        //    2^2 e
        //    ...
        //    2^29 e
        //    2^30 e

        // leave some stack space
        { 0 } { 0 } { 0 } { 0 }

        for i in 0..4 {
            OP_FROMALTSTACK { u31_to_bits() }
            for _ in 0..31 {
                OP_TOALTSTACK
            }

            { 4 + 30 }

            for _ in 0..31 {
                OP_FROMALTSTACK
                OP_IF
                    OP_DUP OP_TOALTSTACK OP_PICK
                    { u31_add_v31() }
                    OP_FROMALTSTACK
                OP_ENDIF
                OP_1SUB
            }

            OP_DROP
            if i != 3 {
                3 OP_ROLL
            }
        }

        OP_TOALTSTACK OP_TOALTSTACK OP_TOALTSTACK OP_TOALTSTACK

        for _ in 0..15 {
            OP_2DROP
        }
        OP_DROP

        OP_FROMALTSTACK OP_FROMALTSTACK OP_FROMALTSTACK OP_FROMALTSTACK

        OP_SWAP OP_2SWAP OP_SWAP
    }
}

#[cfg(test)]
mod test {
    use p3_field::extension::Complex;

    type F4 = p3_field::extension::BinomialExtensionField<Complex<p3_mersenne_31::Mersenne31>, 2>;
    type F = p3_mersenne_31::Mersenne31;

    #[test]
    fn test_u31ext_mul_u31() {
    }
}