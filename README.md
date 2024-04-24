## rust-bitcoin-m31-or-babybear

This repository implements M31 and BabyBear field arithmetic in Bitcoin Script.

### Performance

In the current implementation, M31 and BabyBear has equivalent performance for the standalone field. 
The overhead for field extension is slightly different due to the extension polynomial.

- addition: 18 weight units
- subtraction: 12 weight units
- multiplication: 1415 weight units

For the degree-4 extension of BabyBear over x^4 - 11, we have:

- addition: 84 weight units
- subtraction: 63 weight units
- multiplication: 13594 weight units

For the degree-4 extension of M31 using y^2 - 2 - i over the complex field x^2 + 1, we have:

- addition: 84 weight units
- subtraction: 63 weight units
- multiplication: 13321 weight units

### Credits

Thanks to [Robin Linus](https://robinlinus.com/) for pointing out an optimization that reduces the multiplication from 1767 to 1736 (`1 OP_ROLL` is 
equivalent to `OP_SWAP`). 

Thanks to [Shahar Papini](https://twitter.com/PapiniShahar) from Starkware for pointing out that double Karatsuba can improve the performance for QM31, which also works for 
BabyBear4 and reduces the multiplication cost down from 21992 to 16483 for BabyBear4.

A windowing method is used to reduce the multiplication overhead further, making it from 16483 to 14404 for BabyBear4, but it was not as powerful as expected.

The introduction of a dual form, `v31`, for which `u31 + v31` are more efficient than `u31 + u31` or `v31 + v31`, brings the cost from 14404 to 13594 for BabyBear4.