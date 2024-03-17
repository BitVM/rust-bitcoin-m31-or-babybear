use bitcoin::{ScriptBuf as Script};
use bitcoin_script::{bitcoin_script as script};
use crate::{pushable, unroll};

use crate::u31::{BabyBear, u31_add, u31_double, u31_mul, u31_sub, U31Config};

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

#[cfg(test)]
mod test {
    use core::ops::{Add, Mul, Neg};
    use p3_field::{AbstractExtensionField, AbstractField, PrimeField32};
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha20Rng;
    use crate::execute_script;

    use super::*;

    type F = p3_field::extension::BinomialExtensionField<p3_baby_bear::BabyBear, 4>;

    #[test]
    fn test_u31ext_add() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);

        let a = rng.gen::<F>();
        let b = rng.gen::<F>();

        let c = a.add(b);

        let a: &[p3_baby_bear::BabyBear] = a.as_base_slice();
        let b: &[p3_baby_bear::BabyBear] = b.as_base_slice();
        let c: &[p3_baby_bear::BabyBear] = c.as_base_slice();

        let script = script! {
            { a[3].as_canonical_u32() } { a[2].as_canonical_u32() } { a[1].as_canonical_u32() } { a[0].as_canonical_u32() }
            { b[3].as_canonical_u32() } { b[2].as_canonical_u32() } { b[1].as_canonical_u32() } { b[0].as_canonical_u32() }
            { u31ext_add::<BabyBear4>() }
            { c[3].as_canonical_u32() } { c[2].as_canonical_u32() } { c[1].as_canonical_u32() } { c[0].as_canonical_u32() }
            { u31ext_equalverify::<BabyBear4>() }
            OP_PUSHNUM_1
        };
        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }

    #[test]
    fn test_u31ext_double() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);

        let a = rng.gen::<F>();
        let c = a.double();

        let a: &[p3_baby_bear::BabyBear] = a.as_base_slice();
        let c: &[p3_baby_bear::BabyBear] = c.as_base_slice();

        let script = script! {
            { a[3].as_canonical_u32() } { a[2].as_canonical_u32() } { a[1].as_canonical_u32() } { a[0].as_canonical_u32() }
            { u31ext_double::<BabyBear4>() }
            { c[3].as_canonical_u32() } { c[2].as_canonical_u32() } { c[1].as_canonical_u32() } { c[0].as_canonical_u32() }
            { u31ext_equalverify::<BabyBear4>() }
            OP_PUSHNUM_1
        };
        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }

    #[test]
    fn test_u31ext_sub() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);

        let a = rng.gen::<F>();
        let b = rng.gen::<F>();
        let c = a.add(b.neg());

        let a: &[p3_baby_bear::BabyBear] = a.as_base_slice();
        let b: &[p3_baby_bear::BabyBear] = b.as_base_slice();
        let c: &[p3_baby_bear::BabyBear] = c.as_base_slice();

        let script = script! {
            { a[3].as_canonical_u32() } { a[2].as_canonical_u32() } { a[1].as_canonical_u32() } { a[0].as_canonical_u32() }
            { b[3].as_canonical_u32() } { b[2].as_canonical_u32() } { b[1].as_canonical_u32() } { b[0].as_canonical_u32() }
            { u31ext_sub::<BabyBear4>() }
            { c[3].as_canonical_u32() } { c[2].as_canonical_u32() } { c[1].as_canonical_u32() } { c[0].as_canonical_u32() }
            { u31ext_equalverify::<BabyBear4>() }
            OP_PUSHNUM_1
        };
        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }

    #[test]
    fn test_u31ext_mul() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);

        let a = rng.gen::<F>();
        let b = rng.gen::<F>();
        let c = a.mul(b);

        let a: &[p3_baby_bear::BabyBear] = a.as_base_slice();
        let b: &[p3_baby_bear::BabyBear] = b.as_base_slice();
        let c: &[p3_baby_bear::BabyBear] = c.as_base_slice();

        let script = script! {
            { a[3].as_canonical_u32() } { a[2].as_canonical_u32() } { a[1].as_canonical_u32() } { a[0].as_canonical_u32() }
            { b[3].as_canonical_u32() } { b[2].as_canonical_u32() } { b[1].as_canonical_u32() } { b[0].as_canonical_u32() }
            { u31ext_mul::<BabyBear4>() }
            { c[3].as_canonical_u32() } { c[2].as_canonical_u32() } { c[1].as_canonical_u32() } { c[0].as_canonical_u32() }
            { u31ext_equalverify::<BabyBear4>() }
            OP_PUSHNUM_1
        };
        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }
}