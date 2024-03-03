# random_unique

## Non-Repeating Pseudo-Random Number Generator

This is a rust version of:

https://preshing.com/20121224/how-to-generate-a-sequence-of-unique-random-integers/
https://github.com/preshing/RandomSequence/tree/master

This is a random number generator, 
* which outputs a unique 32-bit integer each time it is called. 
* The random numbers are cyclicially repeating after 2^32 numbers
* All numbers that can be realized as an u32 are used exactly once in this cycle



The core idea behind it are:

Start with an index that counts from 0 to 0xFFFFFFFF.

Transform this index with chained bijective functions so that it looks random.

The bijective functions that are used are

```
f(n) = (a + n) xor b    # with a and b being constant 
r(n) = (n * n) % p      # with p being a prime number for which holds 3 = p % 4
g(r(n)) =  r(n) if r(n) <= p/2 else p - r(n)
```

The prime number 4294967291 = 0xFFFFFFFF - 5, has the property 3 = 4294967291 % 4.

It has the remarkable property that g(r(n)) is bijective.

Thus any number from 0 to 4294967291 will be mapped to exactly one other number from 0 to 4294967291. The exceptions, the numbers 4294967292 to 4294967295 are mapped to themselves.

Then additonaly the addition and XORs are applied to further scrample the numbers.

## How to Build

Have a working rust installation:

```
cargo run
```

To run the tests.
```
cargo test --release -- --nocapture  # The tests run for a few minutes 
```