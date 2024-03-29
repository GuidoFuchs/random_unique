# random_unique

## Non-Repeating Pseudo-Random Number Generator

This is a rust version of:

https://preshing.com/20121224/how-to-generate-a-sequence-of-unique-random-integers/

https://github.com/preshing/RandomSequence/tree/master

This is a simple random number generator (don't use if for statistics or crypto), with some special properties:
* It outputs a unique 32-bit integer each time it is called.  
* The random numbers are cyclically repeating after 2^32 numbers.
* All numbers that can be realized as an u32 are used exactly once in this cycle.


The core ideas behind this random numbers generators are:

* Start with an number (index) that linearly counts upward from 0 to 0xFFFFFFFF.
* Transform this number with chained bijective functions (on u32).
* Use enough bijective functions sequentially, so that after some function calls it looks random enough.

Essentially 
```
random_nr(index) = permutate( add_and_xor( permutate(index) ) )
--
random_nr: u32 -> u32 bijectively
```

The bijective functions that are used are:

```bash
add_and_xor(n) = (a + n) xor b  # with a and b being constant
```

```bash
# https://en.wikipedia.org/wiki/Quadratic_residue
r(n) = (n * n) % p  # with p being a prime number for which holds 3 = p % 4
permutate(r(n)) =  r(n) if r(n) <= p/2 else p - r(n)
```

For p the special prime number 4294967291 is used.
```bash
4294967291 = 0xFFFFFFFF - 5 # is the largest prime that fits in u32
4294967291 % 4 = 3  # is a quadratic residue modulo 3
```
Using this prime the permutate function is bijective.

## Possible applications

Transform lists of similar numbers, into very dissimilar numbers.

Generate non-repeating sequences of scrambled numbers.

## How to Build

Run the example:

```bash
cargo run
```

To run the tests:
```bash
# The tests run for a few minutes, so please be patient
cargo test --release -- --nocapture 

# To run some additional test-code
cargo test --release -- --nocapture  ----ignored
```