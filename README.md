## rust-bitcoin-m31-or-babybear

This repository implements the Bitcoin script for processing the M31 or BabyBear field.

### Performance

In the current implementation, M31 and BabyBear has equivalent performance for the standalone field. 
The overhead for field extension is slightly different due to the extension polynomial.

- addition: 18 weight units
- subtraction: 12 weight units
- multiplication: 1505 weight units

For the degree-4 extension of BabyBear over x^4 - 11, we have:

- addition: 84 weight units
- subtraction: 63 weight units
- multiplication: 14404 weight units

For the degree-4 extension of M31 using y^2 - 2 - i over the complex field x^2 + 1, we have:

- addition: 84 weight units
- subtraction: 63 weight units
- multiplication: 14131 weight units

### Credits

Thanks to [Robin Linus](https://robinlinus.com/) for pointing out an optimization that reduces the multiplication from 1767 to 1736 (`1 OP_ROLL` is 
equivalent to `OP_SWAP`). 

Thanks to [Shahar Papini](https://twitter.com/PapiniShahar) from Starkware for pointing out that double Karatsuba can improve the performance for QM31, which also works for 
BabyBear4 and reduces the multiplication cost down from 21992 to 16483 for BabyBear.

A windowing method is used to reduce the multiplication overhead further, but it was not as powerful as expected.