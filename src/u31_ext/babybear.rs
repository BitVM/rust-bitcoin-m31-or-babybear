use crate::u31::{u31_add, u31_double, BabyBear};
use crate::U31ExtConfig;
use crate::{karatsuba_big, pushable};
use bitcoin::ScriptBuf as Script;
use bitcoin_script::script;

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
            { karatsuba_big::<BabyBear>() }
            6 OP_ROLL
            6 OP_ROLL
            { u31_add::<BabyBear>() }
            { Self::mul_11() }
            { u31_add::<BabyBear>() }
            5 OP_ROLL
            { Self::mul_11() }
            2 OP_ROLL
            { u31_add::<BabyBear>() }
            5 OP_ROLL
            { Self::mul_11() }
            3 OP_ROLL
            4 OP_ROLL
            { u31_add::<BabyBear>() }
            { u31_add::<BabyBear>() }
            OP_SWAP
            OP_ROT
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        execute_script, u31ext_add, u31ext_double, u31ext_equalverify, u31ext_mul, u31ext_sub,
    };
    use core::ops::{Add, Mul, Neg};
    use p3_field::{AbstractExtensionField, AbstractField, PrimeField32};
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    use super::*;

    type F = p3_field::extension::BinomialExtensionField<p3_baby_bear::BabyBear, 4>;

    #[test]
    fn test_u31ext_add() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("babybear4 add: {}", u31ext_add::<BabyBear4>().len());

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
        eprintln!("babybear4 sub: {}", u31ext_sub::<BabyBear4>().len());

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
        eprintln!("babybear4 mul: {}", u31ext_mul::<BabyBear4>().len());

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
