use crate::{karatsuba_complex_big, u31_add, u31_double, u31_sub, U31ExtConfig, M31};
use bitvm::treepp::*;

pub struct QM31;

impl U31ExtConfig for QM31 {
    type BaseFieldConfig = M31;
    const DEGREE: u32 = 4;

    fn mul_impl() -> Script {
        script! {
            { karatsuba_complex_big::<M31>() }
            4 OP_ROLL
            OP_DUP
            { u31_double::<M31>() }
            6 OP_ROLL
            OP_DUP
            { u31_double::<M31>() }
            OP_ROT
            OP_ROT
            { u31_sub::<M31>() }
            3 OP_ROLL
            { u31_add::<M31>() }
            OP_ROT
            OP_ROT
            { u31_add::<M31>() }
            OP_ROT
            { u31_add::<M31>() }
            OP_SWAP
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
    use p3_field::extension::Complex;
    use p3_field::{AbstractExtensionField, AbstractField, PrimeField32};
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    use super::*;

    type F = p3_field::extension::BinomialExtensionField<Complex<p3_mersenne_31::Mersenne31>, 2>;

    #[test]
    fn test_u31ext_add() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("qm31 add: {}", u31ext_add::<QM31>().len());

        let a = rng.gen::<F>();
        let b = rng.gen::<F>();

        let c = a.add(b);

        let a: &[Complex<p3_mersenne_31::Mersenne31>] = a.as_base_slice();
        let b: &[Complex<p3_mersenne_31::Mersenne31>] = b.as_base_slice();
        let c: &[Complex<p3_mersenne_31::Mersenne31>] = c.as_base_slice();

        let script = script! {
            { a[1].imag().as_canonical_u32() }
            { a[1].real().as_canonical_u32() }
            { a[0].imag().as_canonical_u32() }
            { a[0].real().as_canonical_u32() }
            { b[1].imag().as_canonical_u32() }
            { b[1].real().as_canonical_u32() }
            { b[0].imag().as_canonical_u32() }
            { b[0].real().as_canonical_u32() }
            { u31ext_add::<QM31>() }
            { c[1].imag().as_canonical_u32() }
            { c[1].real().as_canonical_u32() }
            { c[0].imag().as_canonical_u32() }
            { c[0].real().as_canonical_u32() }
            { u31ext_equalverify::<QM31>() }
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

        let a: &[Complex<p3_mersenne_31::Mersenne31>] = a.as_base_slice();
        let c: &[Complex<p3_mersenne_31::Mersenne31>] = c.as_base_slice();

        let script = script! {
            { a[1].imag().as_canonical_u32() }
            { a[1].real().as_canonical_u32() }
            { a[0].imag().as_canonical_u32() }
            { a[0].real().as_canonical_u32() }
            { u31ext_double::<QM31>() }
            { c[1].imag().as_canonical_u32() }
            { c[1].real().as_canonical_u32() }
            { c[0].imag().as_canonical_u32() }
            { c[0].real().as_canonical_u32() }
            { u31ext_equalverify::<QM31>() }
            OP_PUSHNUM_1
        };
        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }

    #[test]
    fn test_u31ext_sub() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("qm31 sub: {}", u31ext_sub::<QM31>().len());

        let a = rng.gen::<F>();
        let b = rng.gen::<F>();
        let c = a.add(b.neg());

        let a: &[Complex<p3_mersenne_31::Mersenne31>] = a.as_base_slice();
        let b: &[Complex<p3_mersenne_31::Mersenne31>] = b.as_base_slice();
        let c: &[Complex<p3_mersenne_31::Mersenne31>] = c.as_base_slice();

        let script = script! {
            { a[1].imag().as_canonical_u32() }
            { a[1].real().as_canonical_u32() }
            { a[0].imag().as_canonical_u32() }
            { a[0].real().as_canonical_u32() }
            { b[1].imag().as_canonical_u32() }
            { b[1].real().as_canonical_u32() }
            { b[0].imag().as_canonical_u32() }
            { b[0].real().as_canonical_u32() }
            { u31ext_sub::<QM31>() }
            { c[1].imag().as_canonical_u32() }
            { c[1].real().as_canonical_u32() }
            { c[0].imag().as_canonical_u32() }
            { c[0].real().as_canonical_u32() }
            { u31ext_equalverify::<QM31>() }
            OP_PUSHNUM_1
        };
        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }

    #[test]
    fn test_u31ext_mul() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("qm31 mul: {}", u31ext_mul::<QM31>().len());

        let a = rng.gen::<F>();
        let b = rng.gen::<F>();
        let c = a.mul(b);

        let a: &[Complex<p3_mersenne_31::Mersenne31>] = a.as_base_slice();
        let b: &[Complex<p3_mersenne_31::Mersenne31>] = b.as_base_slice();
        let c: &[Complex<p3_mersenne_31::Mersenne31>] = c.as_base_slice();

        let script = script! {
            { a[1].imag().as_canonical_u32() }
            { a[1].real().as_canonical_u32() }
            { a[0].imag().as_canonical_u32() }
            { a[0].real().as_canonical_u32() }
            { b[1].imag().as_canonical_u32() }
            { b[1].real().as_canonical_u32() }
            { b[0].imag().as_canonical_u32() }
            { b[0].real().as_canonical_u32() }
            { u31ext_mul::<QM31>() }
            { c[1].imag().as_canonical_u32() }
            { c[1].real().as_canonical_u32() }
            { c[0].imag().as_canonical_u32() }
            { c[0].real().as_canonical_u32() }
            { u31ext_equalverify::<QM31>() }
            OP_PUSHNUM_1
        };
        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }

    #[test]
    fn test_u31ext_mul_u31() {
        let mul_script = u31ext_mul_u31::<QM31>();

        let mut rng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("qm31 mul_by_m31: {}", mul_script.len());

        let a = rng.gen::<F>();
        let b = rng.gen::<p3_mersenne_31::Mersenne31>();

        let c = a * F::new(
            Complex::<p3_mersenne_31::Mersenne31>::new(b, p3_mersenne_31::Mersenne31::zero()),
            Complex::<p3_mersenne_31::Mersenne31>::zero(),
        );

        let a: &[Complex<p3_mersenne_31::Mersenne31>] = a.as_base_slice();
        let c: &[Complex<p3_mersenne_31::Mersenne31>] = c.as_base_slice();

        let script = script! {
            { a[1].imag().as_canonical_u32() }
            { a[1].real().as_canonical_u32() }
            { a[0].imag().as_canonical_u32() }
            { a[0].real().as_canonical_u32() }
            { b.as_canonical_u32() }
            { mul_script.clone() }
            { c[1].imag().as_canonical_u32() }
            { c[1].real().as_canonical_u32() }
            { c[0].imag().as_canonical_u32() }
            { c[0].real().as_canonical_u32() }
            { u31ext_equalverify::<QM31>() }
            OP_TRUE
        };

        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }

    #[test]
    fn test_u31ext_mul_u31_by_constant() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);
        let mut total_len = 0;

        for _ in 0..100 {
            let a = rng.gen::<F>();
            let b = rng.gen::<p3_mersenne_31::Mersenne31>();

            let mul_script = u31ext_mul_u31_by_constant::<QM31>(b.as_canonical_u32());
            total_len += mul_script.len();

            let c = a * F::new(
                Complex::<p3_mersenne_31::Mersenne31>::new(b, p3_mersenne_31::Mersenne31::zero()),
                Complex::<p3_mersenne_31::Mersenne31>::zero(),
            );

            let a: &[Complex<p3_mersenne_31::Mersenne31>] = a.as_base_slice();
            let c: &[Complex<p3_mersenne_31::Mersenne31>] = c.as_base_slice();

            let script = script! {
                { a[1].imag().as_canonical_u32() }
                { a[1].real().as_canonical_u32() }
                { a[0].imag().as_canonical_u32() }
                { a[0].real().as_canonical_u32() }
                { mul_script.clone() }
                { c[1].imag().as_canonical_u32() }
                { c[1].real().as_canonical_u32() }
                { c[0].imag().as_canonical_u32() }
                { c[0].real().as_canonical_u32() }
                { u31ext_equalverify::<QM31>() }
                OP_TRUE
            };

            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }

        eprintln!("qm31 mul_by_m31_by_constant: {}", total_len as f64 / 100.0);
    }

    #[test]
    fn test_u31ext_copy() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);

        let a = rng.gen::<F>();
        let b = rng.gen::<F>();

        let a: &[Complex<p3_mersenne_31::Mersenne31>] = a.as_base_slice();
        let b: &[Complex<p3_mersenne_31::Mersenne31>] = b.as_base_slice();

        let copy_script = u31ext_copy::<QM31>(1);

        let script = script! {
            { a[1].imag().as_canonical_u32() }
            { a[1].real().as_canonical_u32() }
            { a[0].imag().as_canonical_u32() }
            { a[0].real().as_canonical_u32() }
            { b[1].imag().as_canonical_u32() }
            { b[1].real().as_canonical_u32() }
            { b[0].imag().as_canonical_u32() }
            { b[0].real().as_canonical_u32() }
            { copy_script.clone() }
            { a[1].imag().as_canonical_u32() }
            { a[1].real().as_canonical_u32() }
            { a[0].imag().as_canonical_u32() }
            { a[0].real().as_canonical_u32() }
            { u31ext_equalverify::<QM31>() }
            { b[1].imag().as_canonical_u32() }
            { b[1].real().as_canonical_u32() }
            { b[0].imag().as_canonical_u32() }
            { b[0].real().as_canonical_u32() }
            { u31ext_equalverify::<QM31>() }
            { a[1].imag().as_canonical_u32() }
            { a[1].real().as_canonical_u32() }
            { a[0].imag().as_canonical_u32() }
            { a[0].real().as_canonical_u32() }
            { u31ext_equalverify::<QM31>() }
            OP_TRUE
        };

        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }

    #[test]
    fn test_u31ext_roll() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);

        let a = rng.gen::<F>();
        let b = rng.gen::<F>();

        let a: &[Complex<p3_mersenne_31::Mersenne31>] = a.as_base_slice();
        let b: &[Complex<p3_mersenne_31::Mersenne31>] = b.as_base_slice();

        let roll_script = u31ext_roll::<QM31>(1);

        let script = script! {
            { a[1].imag().as_canonical_u32() }
            { a[1].real().as_canonical_u32() }
            { a[0].imag().as_canonical_u32() }
            { a[0].real().as_canonical_u32() }
            { b[1].imag().as_canonical_u32() }
            { b[1].real().as_canonical_u32() }
            { b[0].imag().as_canonical_u32() }
            { b[0].real().as_canonical_u32() }
            { roll_script.clone() }
            { a[1].imag().as_canonical_u32() }
            { a[1].real().as_canonical_u32() }
            { a[0].imag().as_canonical_u32() }
            { a[0].real().as_canonical_u32() }
            { u31ext_equalverify::<QM31>() }
            { b[1].imag().as_canonical_u32() }
            { b[1].real().as_canonical_u32() }
            { b[0].imag().as_canonical_u32() }
            { b[0].real().as_canonical_u32() }
            { u31ext_equalverify::<QM31>() }
            OP_TRUE
        };

        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }
}
