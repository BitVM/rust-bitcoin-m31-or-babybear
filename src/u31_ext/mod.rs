use crate::{pushable, unroll};
use bitcoin::ScriptBuf as Script;
use bitcoin_script::bitcoin_script as script;

mod babybear;
pub use babybear::*;

mod m31;
pub use m31::*;

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

#[cfg(test)]
mod test {
    use crate::execute_script;
    use core::ops::{Add, Mul, Neg};
    use p3_field::{AbstractExtensionField, AbstractField, PrimeField32};
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    use super::*;

    type F = p3_field::extension::BinomialExtensionField<p3_baby_bear::BabyBear, 4>;

    #[test]
    fn test_u31ext_add() {
        let mut rng = ChaCha20Rng::seed_from_u64(0u64);
        // println!("ext add: {}", u31ext_add::<BabyBear4>().len());

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
        // println!("ext sub: {}", u31ext_sub::<BabyBear4>().len());

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
        // println!("ext mul: {}", u31ext_mul::<BabyBear4>().len());

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
