use crate::pushable;
use crate::{u31_add, u31_mul, u31_sub, U31Config};
use bitcoin::ScriptBuf as Script;
use bitcoin_script::script;

// Input: A1 B1 A2 B2
// Output:
//      A1B2 + A2B1
//      B1B2 - A1A2
pub fn karatsuba_complex_small<M: U31Config>() -> Script {
    script! {
        OP_OVER 4 OP_PICK
        { u31_mul::<M>() }
        OP_TOALTSTACK
        OP_DUP
        3 OP_PICK
        { u31_mul::<M>() }
        OP_TOALTSTACK
        { u31_add::<M>() }
        OP_TOALTSTACK
        { u31_add::<M>() }
        OP_FROMALTSTACK
        { u31_mul::<M>() }
        OP_FROMALTSTACK
        OP_FROMALTSTACK
        OP_2DUP
        { u31_add::<M>() }
        3 OP_ROLL
        OP_SWAP
        { u31_sub::<M>() }
        OP_TOALTSTACK
        { u31_sub::<M>() }
        OP_FROMALTSTACK
        OP_SWAP
    }
}

// Input:
//      A1 B1 C1 D1
//      A2 B2 C2 D2
// Output:
//      (A1, B1) * (A2, B2) - 2 elements
//      (A1, B1) * (C2, D2) + (A2, B2) * (C1, D1) - 2 elements
//      (C1, D1) * (C2, D2) - 2 elements
pub fn karatsuba_complex_big<M: U31Config>() -> Script {
    script! {
        7 OP_PICK
        7 OP_PICK
        5 OP_PICK
        5 OP_PICK
        { karatsuba_complex_small::<M>() }
        OP_TOALTSTACK
        OP_TOALTSTACK
        OP_2DUP
        7 OP_PICK
        7 OP_PICK
        { karatsuba_complex_small::<M>() }
        OP_TOALTSTACK
        OP_TOALTSTACK
        OP_ROT
        { u31_add::<M>() }
        OP_TOALTSTACK
        { u31_add::<M>() }
        OP_TOALTSTACK
        OP_ROT
        { u31_add::<M>() }
        OP_TOALTSTACK
        { u31_add::<M>() }
        OP_FROMALTSTACK
        OP_FROMALTSTACK
        OP_FROMALTSTACK
        { karatsuba_complex_small::<M>() }
        OP_FROMALTSTACK
        OP_FROMALTSTACK
        OP_FROMALTSTACK
        OP_FROMALTSTACK
        5 OP_ROLL
        2 OP_PICK
        5 OP_PICK
        { u31_add::<M>() }
        { u31_sub::<M>() }
        5 OP_ROLL
        2 OP_PICK
        5 OP_PICK
        { u31_add::<M>() }
        { u31_sub::<M>() }
        5 OP_ROLL
        5 OP_ROLL
    }
}

#[cfg(test)]
mod test {
    use crate::{execute_script, M31};
    use crate::{karatsuba_complex_big, karatsuba_complex_small, pushable};
    use bitcoin_script::script;
    use core::ops::{Add, Mul, Sub};
    use p3_field::extension::Complex;
    use p3_field::PrimeField32;
    use p3_mersenne_31::Mersenne31 as P3M31;
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    #[test]
    fn test_small_karatsuba_complex() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);

        let a1: P3M31 = prng.gen();
        let b1: P3M31 = prng.gen();
        let a2: P3M31 = prng.gen();
        let b2: P3M31 = prng.gen();

        let first = a1.mul(b2).add(a2.mul(b1));
        let second = b1.mul(b2).sub(a1.mul(a2));

        let script = script! {
            { a1.as_canonical_u32() } { b1.as_canonical_u32() } { a2.as_canonical_u32() } { b2.as_canonical_u32() }
            { karatsuba_complex_small::<M31>() }
            { second.as_canonical_u32() }
            OP_EQUALVERIFY
            { first.as_canonical_u32() }
            OP_EQUAL
        };
        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }

    #[test]
    fn test_big_karatsuba_complex() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);

        let a1: P3M31 = prng.gen();
        let b1: P3M31 = prng.gen();
        let c1: P3M31 = prng.gen();
        let d1: P3M31 = prng.gen();

        let a2: P3M31 = prng.gen();
        let b2: P3M31 = prng.gen();
        let c2: P3M31 = prng.gen();
        let d2: P3M31 = prng.gen();

        let group1_first = a1.mul(b2).add(a2.mul(b1));
        let group1_second = b1.mul(b2).sub(a1.mul(a2));

        let group3_first = c1.mul(d2).add(c2.mul(d1));
        let group3_second = d1.mul(d2).sub(c1.mul(c2));

        let group2_first = a1.mul(d2).add(b1.mul(c2)).add(a2.mul(d1)).add(b2.mul(c1));
        let group2_second = b1.mul(d2).add(b2.mul(d1)).sub(a1.mul(c2).add(a2.mul(c1)));

        let script = script! {
            { a1.as_canonical_u32() } { b1.as_canonical_u32() } { c1.as_canonical_u32() } { d1.as_canonical_u32() }
            { a2.as_canonical_u32() } { b2.as_canonical_u32() } { c2.as_canonical_u32() } { d2.as_canonical_u32() }
            { karatsuba_complex_big::<M31>() }
            { group3_second.as_canonical_u32() }
            OP_EQUALVERIFY
            { group3_first.as_canonical_u32() }
            OP_EQUALVERIFY
            { group2_second.as_canonical_u32() }
            OP_EQUALVERIFY
            { group2_first.as_canonical_u32() }
            OP_EQUALVERIFY
            { group1_second.as_canonical_u32() }
            OP_EQUALVERIFY
            { group1_first.as_canonical_u32() }
            OP_EQUAL
        };
        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }

    #[test]
    fn test_small_karatsuba_complex_consistency() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);

        let a: Complex<p3_mersenne_31::Mersenne31> = rng.gen();
        let b: Complex<p3_mersenne_31::Mersenne31> = rng.gen();
        let c = a.mul(b);

        let script = script! {
            { a.imag().as_canonical_u32() } { a.real().as_canonical_u32() }
            { b.imag().as_canonical_u32() } { b.real().as_canonical_u32() }
            { karatsuba_complex_small::<M31>() }
            { c.real().as_canonical_u32() }
            OP_EQUALVERIFY
            { c.imag().as_canonical_u32() }
            OP_EQUAL
        };
        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }
}
