use crate::{u31_add_v31, u31_to_bits, u31_to_v31, unroll, v31_double};
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
        OP_DUP { v31_double::<C::BaseFieldConfig>() }
        OP_DUP { v31_double::<C::BaseFieldConfig>() }
        OP_DUP { v31_double::<C::BaseFieldConfig>() }
        OP_DUP { v31_double::<C::BaseFieldConfig>() }

        OP_DUP { v31_double::<C::BaseFieldConfig>() }
        OP_DUP { v31_double::<C::BaseFieldConfig>() }
        OP_DUP { v31_double::<C::BaseFieldConfig>() }
        OP_DUP { v31_double::<C::BaseFieldConfig>() }
        OP_DUP { v31_double::<C::BaseFieldConfig>() }

        OP_DUP { v31_double::<C::BaseFieldConfig>() }
        OP_DUP { v31_double::<C::BaseFieldConfig>() }
        OP_DUP { v31_double::<C::BaseFieldConfig>() }
        OP_DUP { v31_double::<C::BaseFieldConfig>() }
        OP_DUP { v31_double::<C::BaseFieldConfig>() }

        OP_DUP { v31_double::<C::BaseFieldConfig>() }
        OP_DUP { v31_double::<C::BaseFieldConfig>() }
        OP_DUP { v31_double::<C::BaseFieldConfig>() }
        OP_DUP { v31_double::<C::BaseFieldConfig>() }
        OP_DUP { v31_double::<C::BaseFieldConfig>() }

        OP_DUP { v31_double::<C::BaseFieldConfig>() }
        OP_DUP { v31_double::<C::BaseFieldConfig>() }
        OP_DUP { v31_double::<C::BaseFieldConfig>() }
        OP_DUP { v31_double::<C::BaseFieldConfig>() }
        OP_DUP { v31_double::<C::BaseFieldConfig>() }

        OP_DUP { v31_double::<C::BaseFieldConfig>() }
        OP_DUP { v31_double::<C::BaseFieldConfig>() }
        OP_DUP { v31_double::<C::BaseFieldConfig>() }
        OP_DUP { v31_double::<C::BaseFieldConfig>() }
        OP_DUP { v31_double::<C::BaseFieldConfig>() }

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

            { 4 }

            for _ in 0..31 {
                OP_FROMALTSTACK
                OP_IF
                    OP_DUP OP_TOALTSTACK OP_PICK
                    { u31_add_v31::<C::BaseFieldConfig>() }
                    OP_FROMALTSTACK
                OP_ENDIF
                OP_1ADD
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
    }
}

#[cfg(test)]
mod test {
    use crate::{u31ext_equalverify, u31ext_mul_u31, QM31};
    use bitvm::treepp::*;
    use p3_field::extension::Complex;
    use p3_field::{AbstractExtensionField, AbstractField, PrimeField32};
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    type F4 = p3_field::extension::BinomialExtensionField<Complex<p3_mersenne_31::Mersenne31>, 2>;
    type F = p3_mersenne_31::Mersenne31;

    #[test]
    fn test_u31ext_mul_u31() {
        let mul_script = u31ext_mul_u31::<QM31>();

        let mut rng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("qm31 mul_by_m31: {}", u31ext_mul_u31::<QM31>().len());

        let a = rng.gen::<F4>();
        let b = rng.gen::<F>();

        let c = a * F4::new(
            Complex::<p3_mersenne_31::Mersenne31>::new(b, F::zero()),
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
        println!("{:4}", exec_result.final_stack);
        println!("{:?}", exec_result.error);
        assert!(exec_result.success);
    }
}
