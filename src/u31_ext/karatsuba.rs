use crate::pushable;
use crate::{u31_add, u31_mul, u31_sub, U31Config};
use bitcoin::ScriptBuf as Script;
use bitcoin_script::script;

// Input: A1 B1 A2 B2
// Output:
//      A1A2
//      A1B2 + A2B1
//      B1B2
pub fn karatsuba_small<M: U31Config>() -> Script {
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
        OP_ROT
    }
}

// Input:
//      A1 B1 C1 D1
//      A2 B2 C2 D2
// Output:
//      (A1, B1) * (A2, B2) - 3 elements
//      (A1, B1) * (C2, D2) + (A2, B2) * (C1, D1) - 3 elements
//      (C1, D1) * (C2, D2) - 3 elements
pub fn karatsuba_big<M: U31Config>() -> Script {
    script! {
        7 OP_PICK
        7 OP_PICK
        5 OP_PICK
        5 OP_PICK
        { karatsuba_small::<M>() }
        OP_TOALTSTACK
        OP_TOALTSTACK
        OP_TOALTSTACK
        OP_2DUP
        7 OP_PICK
        7 OP_PICK
        { karatsuba_small::<M>() }
        OP_TOALTSTACK
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
        { karatsuba_small::<M>() }
        OP_FROMALTSTACK
        OP_FROMALTSTACK
        OP_FROMALTSTACK
        OP_FROMALTSTACK
        OP_FROMALTSTACK
        OP_FROMALTSTACK
        8 OP_ROLL
        3 OP_PICK
        7 OP_PICK
        { u31_add::<M>() }
        { u31_sub::<M>() }
        8 OP_ROLL
        3 OP_PICK
        7 OP_PICK
        { u31_add::<M>() }
        { u31_sub::<M>() }
        8 OP_ROLL
        3 OP_PICK
        7 OP_PICK
        { u31_add::<M>() }
        { u31_sub::<M>() }
        8 OP_ROLL
        8 OP_ROLL
        8 OP_ROLL
    }
}

#[cfg(test)]
mod test {
    use crate::{execute_script, karatsuba_small, BabyBear};
    use crate::{karatsuba_big, pushable};
    use bitcoin_script::script;
    use core::ops::{Add, Mul};
    use p3_baby_bear::BabyBear as P3BabyBear;
    use p3_field::PrimeField32;
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    #[test]
    fn test_small_karatsuba() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);

        let a1: P3BabyBear = prng.gen();
        let b1: P3BabyBear = prng.gen();
        let a2: P3BabyBear = prng.gen();
        let b2: P3BabyBear = prng.gen();

        let first = a1.mul(a2);
        let second = a1.mul(b2).add(a2.mul(b1));
        let third = b1.mul(b2);

        let script = script! {
            { a1.as_canonical_u32() } { b1.as_canonical_u32() } { a2.as_canonical_u32() } { b2.as_canonical_u32() }
            { karatsuba_small::<BabyBear>() }
            { third.as_canonical_u32() }
            OP_EQUALVERIFY
            { second.as_canonical_u32() }
            OP_EQUALVERIFY
            { first.as_canonical_u32() }
            OP_EQUAL
        };
        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }

    #[test]
    fn test_big_karatsuba() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);

        let a1: P3BabyBear = prng.gen();
        let b1: P3BabyBear = prng.gen();
        let c1: P3BabyBear = prng.gen();
        let d1: P3BabyBear = prng.gen();

        let a2: P3BabyBear = prng.gen();
        let b2: P3BabyBear = prng.gen();
        let c2: P3BabyBear = prng.gen();
        let d2: P3BabyBear = prng.gen();

        let group1_first = a1.mul(a2);
        let group1_second = a1.mul(b2).add(a2.mul(b1));
        let group1_third = b1.mul(b2);

        let group3_first = c1.mul(c2);
        let group3_second = c1.mul(d2).add(c2.mul(d1));
        let group3_third = d1.mul(d2);

        let group2_first = a1.mul(c2).add(a2.mul(c1));
        let group2_second = a1.mul(d2).add(b1.mul(c2)).add(a2.mul(d1)).add(b2.mul(c1));
        let group2_third = b1.mul(d2).add(b2.mul(d1));

        let script = script! {
            { a1.as_canonical_u32() } { b1.as_canonical_u32() } { c1.as_canonical_u32() } { d1.as_canonical_u32() }
            { a2.as_canonical_u32() } { b2.as_canonical_u32() } { c2.as_canonical_u32() } { d2.as_canonical_u32() }
            { karatsuba_big::<BabyBear>() }
            { group3_third.as_canonical_u32() }
            OP_EQUALVERIFY
            { group3_second.as_canonical_u32() }
            OP_EQUALVERIFY
            { group3_first.as_canonical_u32() }
            OP_EQUALVERIFY
            { group2_third.as_canonical_u32() }
            OP_EQUALVERIFY
            { group2_second.as_canonical_u32() }
            OP_EQUALVERIFY
            { group2_first.as_canonical_u32() }
            OP_EQUALVERIFY
            { group1_third.as_canonical_u32() }
            OP_EQUALVERIFY
            { group1_second.as_canonical_u32() }
            OP_EQUALVERIFY
            { group1_first.as_canonical_u32() }
            OP_EQUAL
        };
        let exec_result = execute_script(script);
        assert!(exec_result.success);
    }
}
