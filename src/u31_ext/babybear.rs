use crate::u31::{u31_add, u31_double, BabyBear};
use crate::U31ExtConfig;
use crate::{karatsuba_big, u31_sub};
use bitvm::treepp::*;

pub struct BabyBear4;

impl BabyBear4 {
    fn mul_minus_11() -> Script {
        script! {
            OP_DUP
            { u31_double::<BabyBear>() }
            { u31_double::<BabyBear>() }
            OP_DUP
            { u31_double::<BabyBear>() }
            { u31_add::<BabyBear>() }
            { u31_sub::<BabyBear>() }
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
            { Self::mul_minus_11() }
            { u31_add::<BabyBear>() }
            5 OP_ROLL
            { Self::mul_minus_11() }
            2 OP_ROLL
            { u31_add::<BabyBear>() }
            5 OP_ROLL
            { Self::mul_minus_11() }
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
        u31ext_add, u31ext_copy, u31ext_double, u31ext_equalverify, u31ext_mul, u31ext_mul_u31,
        u31ext_mul_u31_by_constant, u31ext_roll, u31ext_sub,
    };
    use bitvm::treepp::*;
    use core::ops::{Add, Mul, Neg};
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;
    use risc0_core::field::baby_bear::{BabyBearElem, BabyBearExtElem};
    use risc0_core::field::Elem;

    use super::*;

    #[test]
    fn test_u31ext_add() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("babybear4 add: {}", u31ext_add::<BabyBear4>().len());

        let a = BabyBearExtElem::random(&mut prng);
        let b = BabyBearExtElem::random(&mut prng);

        let c = a.add(b);

        let a: &[BabyBearElem] = a.elems();
        let b: &[BabyBearElem] = b.elems();
        let c: &[BabyBearElem] = c.elems();

        let script = script! {
            { a[3].as_u32() } { a[2].as_u32() } { a[1].as_u32() } { a[0].as_u32() }
            { b[3].as_u32() } { b[2].as_u32() } { b[1].as_u32() } { b[0].as_u32() }
            { u31ext_add::<BabyBear4>() }
            { c[3].as_u32() } { c[2].as_u32() } { c[1].as_u32() } { c[0].as_u32() }
            { u31ext_equalverify::<BabyBear4>() }
            OP_PUSHNUM_1
        };
        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }

    #[test]
    fn test_u31ext_double() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);

        let a = BabyBearExtElem::random(&mut prng);
        let c = a.add(a);

        let a: &[BabyBearElem] = a.elems();
        let c: &[BabyBearElem] = c.elems();

        let script = script! {
            { a[3].as_u32() } { a[2].as_u32() } { a[1].as_u32() } { a[0].as_u32() }
            { u31ext_double::<BabyBear4>() }
            { c[3].as_u32() } { c[2].as_u32() } { c[1].as_u32() } { c[0].as_u32() }
            { u31ext_equalverify::<BabyBear4>() }
            OP_PUSHNUM_1
        };
        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }

    #[test]
    fn test_u31ext_sub() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("babybear4 sub: {}", u31ext_sub::<BabyBear4>().len());

        let a = BabyBearExtElem::random(&mut prng);
        let b = BabyBearExtElem::random(&mut prng);
        let c = a.add(b.neg());

        let a: &[BabyBearElem] = a.elems();
        let b: &[BabyBearElem] = b.elems();
        let c: &[BabyBearElem] = c.elems();

        let script = script! {
            { a[3].as_u32() } { a[2].as_u32() } { a[1].as_u32() } { a[0].as_u32() }
            { b[3].as_u32() } { b[2].as_u32() } { b[1].as_u32() } { b[0].as_u32() }
            { u31ext_sub::<BabyBear4>() }
            { c[3].as_u32() } { c[2].as_u32() } { c[1].as_u32() } { c[0].as_u32() }
            { u31ext_equalverify::<BabyBear4>() }
            OP_PUSHNUM_1
        };
        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }

    #[test]
    fn test_u31ext_mul() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("babybear4 mul: {}", u31ext_mul::<BabyBear4>().len());

        let a = BabyBearExtElem::random(&mut prng);
        let b = BabyBearExtElem::random(&mut prng);
        let c = a.mul(b);

        let a: &[BabyBearElem] = a.elems();
        let b: &[BabyBearElem] = b.elems();
        let c: &[BabyBearElem] = c.elems();

        let script = script! {
            { a[3].as_u32() } { a[2].as_u32() } { a[1].as_u32() } { a[0].as_u32() }
            { b[3].as_u32() } { b[2].as_u32() } { b[1].as_u32() } { b[0].as_u32() }
            { u31ext_mul::<BabyBear4>() }
            { c[3].as_u32() } { c[2].as_u32() } { c[1].as_u32() } { c[0].as_u32() }
            { u31ext_equalverify::<BabyBear4>() }
            OP_PUSHNUM_1
        };
        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }

    #[test]
    fn test_u31ext_mul_u31() {
        let mul_script = u31ext_mul_u31::<BabyBear4>();

        let mut prng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("babybear4 mul_by_babybear: {}", mul_script.len());

        let a = BabyBearExtElem::random(&mut prng);
        let b = BabyBearElem::random(&mut prng);

        let c = a * b;

        let a: &[BabyBearElem] = a.elems();
        let c: &[BabyBearElem] = c.elems();

        let script = script! {
            { a[3].as_u32() } { a[2].as_u32() } { a[1].as_u32() } { a[0].as_u32() }
            { b.as_u32() }
            { mul_script.clone() }
            { c[3].as_u32() } { c[2].as_u32() } { c[1].as_u32() } { c[0].as_u32() }
            { u31ext_equalverify::<BabyBear4>() }
            OP_TRUE
        };

        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }

    #[test]
    fn test_u31ext_mul_u31_by_constant() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);
        let mut total_len = 0;

        for _ in 0..100 {
            let a = BabyBearExtElem::random(&mut prng);
            let b = BabyBearElem::random(&mut prng);

            let mul_script = u31ext_mul_u31_by_constant::<BabyBear4>(b.as_u32());
            total_len += mul_script.len();

            let c = a * b;

            let a: &[BabyBearElem] = a.elems();
            let c: &[BabyBearElem] = c.elems();

            let script = script! {
                { a[3].as_u32() } { a[2].as_u32() } { a[1].as_u32() } { a[0].as_u32() }
                { mul_script.clone() }
                { c[3].as_u32() } { c[2].as_u32() } { c[1].as_u32() } { c[0].as_u32() }
                { u31ext_equalverify::<BabyBear4>() }
                OP_TRUE
            };

            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }

        eprintln!(
            "babybear4 mul_by_babybear_by_constant: {}",
            total_len as f64 / 100.0
        );
    }

    #[test]
    fn test_u31ext_copy() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);

        let a = BabyBearExtElem::random(&mut prng);
        let b = BabyBearExtElem::random(&mut prng);

        let a: &[BabyBearElem] = a.elems();
        let b: &[BabyBearElem] = b.elems();

        let copy_script = u31ext_copy::<BabyBear4>(1);

        let script = script! {
            { a[3].as_u32() } { a[2].as_u32() } { a[1].as_u32() } { a[0].as_u32() }
            { b[3].as_u32() } { b[2].as_u32() } { b[1].as_u32() } { b[0].as_u32() }
            { copy_script.clone() }
            { a[3].as_u32() } { a[2].as_u32() } { a[1].as_u32() } { a[0].as_u32() }
            { u31ext_equalverify::<BabyBear4>() }
            { b[3].as_u32() } { b[2].as_u32() } { b[1].as_u32() } { b[0].as_u32() }
            { u31ext_equalverify::<BabyBear4>() }
            { a[3].as_u32() } { a[2].as_u32() } { a[1].as_u32() } { a[0].as_u32() }
            { u31ext_equalverify::<BabyBear4>() }
            OP_TRUE
        };

        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }

    #[test]
    fn test_u31ext_roll() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);

        let a = BabyBearExtElem::random(&mut prng);
        let b = BabyBearExtElem::random(&mut prng);

        let a: &[BabyBearElem] = a.elems();
        let b: &[BabyBearElem] = b.elems();

        let roll_script = u31ext_roll::<BabyBear4>(1);

        let script = script! {
            { a[3].as_u32() } { a[2].as_u32() } { a[1].as_u32() } { a[0].as_u32() }
            { b[3].as_u32() } { b[2].as_u32() } { b[1].as_u32() } { b[0].as_u32() }
            { roll_script.clone() }
            { a[3].as_u32() } { a[2].as_u32() } { a[1].as_u32() } { a[0].as_u32() }
            { u31ext_equalverify::<BabyBear4>() }
            { b[3].as_u32() } { b[2].as_u32() } { b[1].as_u32() } { b[0].as_u32() }
            { u31ext_equalverify::<BabyBear4>() }
            OP_TRUE
        };

        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }
}
