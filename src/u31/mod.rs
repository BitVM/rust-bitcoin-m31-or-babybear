use bitvm::treepp::*;

mod m31;
pub use m31::*;

mod babybear;
use crate::unroll;
pub use babybear::*;

pub trait U31Config {
    const MOD: u32;
}

pub fn u31_to_v31<M: U31Config>() -> Script {
    script! {
        { M::MOD } OP_SUB
    }
}

pub fn v31_to_u31<M: U31Config>() -> Script {
    script! {
        { M::MOD } OP_ADD
    }
}

pub fn u31_add_v31<M: U31Config>() -> Script {
    script! {
        OP_ADD
        { u31_adjust::<M>() }
    }
}

pub fn v31_add_u31<M: U31Config>() -> Script {
    script! {
        OP_ADD
        { v31_adjust::<M>() }
    }
}

fn u31_adjust<M: U31Config>() -> Script {
    script! {
        OP_DUP
        0 OP_LESSTHAN
        OP_IF { M::MOD } OP_ADD OP_ENDIF
    }
}

fn v31_adjust<M: U31Config>() -> Script {
    script! {
        OP_DUP
        0 OP_GREATERTHANOREQUAL
        OP_IF { M::MOD } OP_SUB OP_ENDIF
    }
}

pub fn u31_add<M: U31Config>() -> Script {
    script! {
        { u31_to_v31::<M>() }
        { u31_add_v31::<M>() }
    }
}

pub fn v31_add<M: U31Config>() -> Script {
    script! {
        { v31_to_u31::<M>() }
        { v31_add_u31::<M>() }
    }
}

pub fn u31_double<M: U31Config>() -> Script {
    script! {
        OP_DUP
        { u31_add::<M>() }
    }
}

pub fn v31_double<M: U31Config>() -> Script {
    script! {
        OP_DUP
        { v31_add::<M>() }
    }
}

pub fn u31_sub<M: U31Config>() -> Script {
    script! {
        OP_SUB
        { u31_adjust::<M>() }
    }
}

pub fn v31_sub<M: U31Config>() -> Script {
    script! {
        OP_SUB
        { v31_adjust::<M>() }
    }
}

pub fn u31_neg<M: U31Config>() -> Script {
    script! {
        { M::MOD }
        OP_SWAP
        OP_SUB
    }
}

pub fn v31_neg<M: U31Config>() -> Script {
    script! {
        { -(M::MOD as i64) }
        OP_SWAP
        OP_SUB
    }
}

pub fn u31_to_bits() -> Script {
    script! {
        {
            unroll(30, |i| {
                let a = 1 << (30 - i);
                let b = a - 1;
                script! {
                    OP_DUP
                    { b } OP_GREATERTHAN
                    OP_SWAP OP_OVER
                    OP_IF { a } OP_SUB OP_ENDIF
                }
        })}
    }
}

pub(crate) fn u31_mul_common<M: U31Config>() -> Script {
    script! {
        0
        OP_SWAP
        { u31_to_v31::<M>() }
        OP_DUP
        { v31_double::<M>() }
        OP_2DUP
        { v31_add::<M>() }
        0
        OP_FROMALTSTACK
        OP_IF
            3 OP_PICK
            { u31_add_v31::<M>() }
        OP_ENDIF
        { u31_double::<M>() }
        { u31_double::<M>() }
        { unroll(14, |_| script! {
            OP_FROMALTSTACK
            OP_FROMALTSTACK
            OP_SWAP OP_DUP OP_ADD OP_ADD
            4 OP_SWAP OP_SUB OP_PICK
            { u31_add_v31::<M>() }
            { u31_double::<M>() }
            { u31_double::<M>() }
        })}
        OP_FROMALTSTACK
        OP_FROMALTSTACK
        OP_SWAP OP_DUP OP_ADD OP_ADD
        4 OP_SWAP OP_SUB OP_PICK
        { u31_add_v31::<M>() }
        OP_TOALTSTACK
        OP_2DROP OP_2DROP
        OP_FROMALTSTACK
    }
}

pub fn u31_mul<M: U31Config>() -> Script {
    script! {
        u31_to_bits
        { unroll(31, |_| script! {
            OP_TOALTSTACK
        }) }
        { u31_mul_common::<M>() }
    }
}

pub fn u31_mul_by_constant<M: U31Config>(constant: u32) -> Script {
    let mut naf = ark_ff::biginteger::arithmetic::find_naf(&[constant as u64]);

    if naf.len() > 3 {
        let len = naf.len();
        if naf[len - 2] == 0 && naf[len - 3] == -1 {
            naf[len - 3] = 1;
            naf[len - 2] = 1;
            naf.resize(len - 1, 0);
        }
    }

    let mut cur = 0usize;
    let mut script_bytes = vec![];

    let double = u31_double::<M>();
    while cur < naf.len() && naf[cur] == 0 {
        script_bytes.extend_from_slice(double.as_bytes());
        cur += 1;
    }

    if cur < naf.len() {
        if naf[cur] == 1 {
            script_bytes.extend_from_slice(&[0x76]); // OP_DUP
            script_bytes.extend_from_slice(double.as_bytes());
            cur += 1;
        } else if naf[cur] == -1 {
            script_bytes.extend_from_slice(
                script! {
                    OP_DUP { u31_neg::<M>() } OP_SWAP
                }
                .as_bytes(),
            );
            script_bytes.extend_from_slice(double.as_bytes());
            cur += 1;
        } else {
            unreachable!()
        }
    } else {
        script_bytes.extend_from_slice(
            script! {
                OP_DROP { 0 }
            }
            .as_bytes(),
        );

        return Script::from(script_bytes);
    }

    if cur < naf.len() {
        while cur < naf.len() {
            if naf[cur] == 0 {
                script_bytes.extend_from_slice(double.as_bytes());
            } else if naf[cur] == 1 {
                script_bytes.extend_from_slice(
                    script! {
                        OP_SWAP OP_OVER { u31_add::<M>() } OP_SWAP
                    }
                    .as_bytes(),
                );
                if cur != naf.len() - 1 {
                    script_bytes.extend_from_slice(double.as_bytes());
                }
            } else if naf[cur] == -1 {
                script_bytes.extend_from_slice(
                    script! {
                        OP_SWAP OP_OVER { u31_sub::<M>() } OP_SWAP
                    }
                    .as_bytes(),
                );
                if cur != naf.len() - 1 {
                    script_bytes.extend_from_slice(double.as_bytes());
                }
            }
            cur += 1;
        }
    }

    script_bytes.extend_from_slice(&[0x75]); // OP_DROP
    Script::from(script_bytes)
}

#[cfg(test)]
mod test {
    use bitvm::treepp::*;
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha20Rng;

    use super::*;

    #[test]
    fn test_u31_add() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("u31 add: {}", u31_add::<BabyBear>().len());

        for _ in 0..100 {
            let a: u32 = prng.gen();
            let b: u32 = prng.gen();

            let a_m31 = a % M31::MOD;
            let b_m31 = b % M31::MOD;
            let sum_m31 = (a_m31 + b_m31) % M31::MOD;

            let script = script! {
                { a_m31 }
                { b_m31 }
                { u31_add::<M31>() }
                { sum_m31 }
                OP_EQUAL
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }

        for _ in 0..100 {
            let a: u32 = prng.gen();
            let b: u32 = prng.gen();

            let a_babybear = a % BabyBear::MOD;
            let b_babybear = b % BabyBear::MOD;
            let sum_babybear = (a_babybear + b_babybear) % BabyBear::MOD;

            let script = script! {
                { a_babybear }
                { b_babybear }
                { u31_add::<BabyBear>() }
                { sum_babybear }
                OP_EQUAL
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success)
        }
    }

    #[test]
    fn test_u31_sub() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);
        eprintln!("u31 sub: {}", u31_sub::<BabyBear>().len());

        for _ in 0..100 {
            let a: u32 = prng.gen();
            let b: u32 = prng.gen();

            let a_m31 = a % M31::MOD;
            let b_m31 = b % M31::MOD;
            let diff_m31 = (M31::MOD + a_m31 - b_m31) % M31::MOD;

            let script = script! {
                { a_m31 }
                { b_m31 }
                { u31_sub::<M31>() }
                { diff_m31 }
                OP_EQUAL
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }

        for _ in 0..100 {
            let a: u32 = prng.gen();
            let b: u32 = prng.gen();

            let a_babybear = a % BabyBear::MOD;
            let b_babybear = b % BabyBear::MOD;
            let diff_babybear = (BabyBear::MOD + a_babybear - b_babybear) % BabyBear::MOD;

            let script = script! {
                { a_babybear }
                { b_babybear }
                { u31_sub::<BabyBear>() }
                { diff_babybear }
                OP_EQUAL
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success)
        }
    }

    #[test]
    fn test_u31_to_bits() {
        let mut prng = ChaCha20Rng::seed_from_u64(0u64);

        for _ in 0..100 {
            let a: u32 = prng.gen();
            let m31 = a % M31::MOD;

            let mut bits = vec![];
            let mut cur = m31;
            for _ in 0..31 {
                bits.push(cur % 2);
                cur >>= 1;
            }
            assert_eq!(cur, 0);

            let script = script! {
                { m31 }
                u31_to_bits
                { unroll(30, |i| script! {
                    { bits[i as usize] } OP_EQUALVERIFY
                })}
                { bits[30] } OP_EQUAL
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }

        for _ in 0..100 {
            let a: u32 = prng.gen();
            let babybear = a % BabyBear::MOD;

            let mut bits = vec![];
            let mut cur = babybear;
            for _ in 0..31 {
                bits.push(cur % 2);
                cur >>= 1;
            }
            assert_eq!(cur, 0);

            let script = script! {
                { babybear }
                u31_to_bits
                { unroll(30, |i| script! {
                    { bits[i as usize] } OP_EQUALVERIFY
                })}
                { bits[30] } OP_EQUAL
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }

    #[test]
    fn test_u31_mul() {
        let mut prng = ChaCha20Rng::seed_from_u64(6u64);
        eprintln!("u31 mul: {}", u31_mul::<BabyBear>().len());

        for _ in 0..100 {
            let a: u32 = prng.gen();
            let b: u32 = prng.gen();

            let a_m31 = a % M31::MOD;
            let b_m31 = b % M31::MOD;
            let prod_m31 =
                ((((a_m31 as u64) * (b_m31 as u64)) % (M31::MOD as u64)) & 0xffffffff) as u32;

            let script = script! {
                { a_m31 }
                { b_m31 }
                { u31_mul::<M31>() }
                { prod_m31 }
                OP_EQUAL
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }

        for _ in 0..100 {
            let a: u32 = prng.gen();
            let b: u32 = prng.gen();

            let a_babybear = a % BabyBear::MOD;
            let b_babybear = b % BabyBear::MOD;
            let prod_babybear = ((((a_babybear as u64) * (b_babybear as u64))
                % (BabyBear::MOD as u64))
                & 0xffffffff) as u32;

            let script = script! {
                { a_babybear }
                { b_babybear }
                { u31_mul::<BabyBear>() }
                { prod_babybear }
                OP_EQUAL
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success)
        }
    }

    #[test]
    fn test_u31_mul_by_constant() {
        let mut prng = ChaCha20Rng::seed_from_u64(6u64);

        let mut total_len = 0;
        for _ in 0..100 {
            let a: u32 = prng.gen();
            let b: u32 = prng.gen();

            let a_m31 = a % M31::MOD;
            let b_m31 = b % M31::MOD;

            let mul_script = u31_mul_by_constant::<M31>(b_m31);
            total_len += mul_script.len();

            let prod_m31 =
                ((((a_m31 as u64) * (b_m31 as u64)) % (M31::MOD as u64)) & 0xffffffff) as u32;

            let script = script! {
                { a_m31 }
                { mul_script.clone() }
                { prod_m31 }
                OP_EQUAL
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }

        eprintln!("m31 mul_by_constant: {}", total_len as f64 / 100.0);

        let mut total_len = 0;
        for _ in 0..100 {
            let a: u32 = prng.gen();
            let b: u32 = prng.gen();

            let a_babybear = a % BabyBear::MOD;
            let b_babybear = b % BabyBear::MOD;

            let mul_script = u31_mul_by_constant::<BabyBear>(b_babybear);
            total_len += mul_script.len();

            let prod_babybear = ((((a_babybear as u64) * (b_babybear as u64))
                % (BabyBear::MOD as u64))
                & 0xffffffff) as u32;

            let script = script! {
                { a_babybear }
                { mul_script.clone() }
                { prod_babybear }
                OP_EQUAL
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success)
        }

        eprintln!("babybear mul_by_constant: {}", total_len as f64 / 100.0);
    }

    #[test]
    fn test_u31_neg() {
        let mut prng = ChaCha20Rng::seed_from_u64(6u64);
        eprintln!("u31 neg: {}", u31_neg::<M31>().len());

        for _ in 0..100 {
            let a: u32 = prng.gen();

            let a_m31 = a % M31::MOD;
            let b_m31 = M31::MOD - a_m31;

            let script = script! {
                { a_m31 }
                { u31_neg::<M31>() }
                { b_m31 }
                OP_EQUAL
            };
            let exec_result = execute_script(script);
            assert!(exec_result.success);
        }
    }
}
