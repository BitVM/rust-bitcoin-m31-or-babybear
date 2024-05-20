## rust-bitcoin-m31-or-babybear

This repository implements M31 and BabyBear field arithmetic in Bitcoin Script.

### Performance

In the current implementation, M31 and BabyBear has equivalent performance for the standalone field. 
The overhead for field extension is slightly different due to the extension polynomial.

- addition: 18 weight units
- subtraction: 12 weight units
- multiplication: 1415 weight units
- multiplication by constant: ~744 weight units (M31), ~738 weight units (BabyBear)

For the degree-4 extension of BabyBear over x^4 + 11, we have:

- addition: 84 weight units
- subtraction: 63 weight units
- multiplication: 13576 weight units
- multiplication by BabyBear: 4702 weight units
- multiplication by BabyBear constant: ~2973 weight units

Note that Plonky3 uses x^4 - 11 as the extension polynomial. Here we use the one from RISC Zero, which is more heavily 
used in production, and it is x^4 + 11.

For the degree-4 extension of M31 using y^2 - 2 - i over the complex field x^2 + 1, we have:

- addition: 84 weight units
- subtraction: 63 weight units
- multiplication: 13321 weight units
- multiplication by M31: 4702 weight units
- multiplication by M31 constant: ~2981 weight units

### Credits

Thanks to [Robin Linus](https://robinlinus.com/) for pointing out an optimization that reduces the multiplication from 1767 to 1736 (`1 OP_ROLL` is 
equivalent to `OP_SWAP`). 

Thanks to [Shahar Papini](https://twitter.com/PapiniShahar) from Starkware for pointing out that double Karatsuba can improve the performance for QM31, which also works for 
BabyBear4 and reduces the multiplication cost down from 21992 to 16483 for BabyBear4.

A windowing method is used to reduce the multiplication overhead further, making it from 16483 to 14404 for BabyBear4, but it was not as powerful as expected.

The introduction of a dual form, `v31`, for which `u31 + v31` are more efficient than `u31 + u31` or `v31 + v31`, brings 
the cost from 1505 to 1415 for BabyBear and from 14404 to 13594 for BabyBear4.

When multiplying a degree-4 element with a degree-1 base element, we reuse the bit decomposition, this avoids the redundancy 
of doing the bit decomposition multiple times, from 5660 to 4702. We note that an alternative route is to produce a 
larger lookup table for the degree-1 base element and share this table between the four subelements in the degree-4 
element. But our attempts show that it is slower than this naive approach (which is expected because the naive method 
already uses a lookup table). 

In case one of the multipliers is a constant, we can have more efficient multiplication using a relaxed NAF representation, 
which saves from 1415 down to \~738 for BabyBear on degree-1 element multiplication in this special case. We use "\~" to 
emphasize that this cost is variable and depends on the constant.

The BabyBear4's multiplication overhead slightly goes down from 13594 to 13576 because we switched the extension polynomial 
into x^4 + 11 (the one used by RISC Zero) from x^4 - 11 (the one used by Plonky3) as the former is more heavily used in 
production. This slightly reduces the cost because multiplication by -11 can be done slightly cheaper than multiplication 
by 11.