use random_unique::random_unique;
use std::time::{SystemTime, UNIX_EPOCH};

/// get_seed uses the system time, to produce two somewhat random numbers
fn get_seed() -> (u32, u32) {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let seed1  = since_the_epoch.subsec_nanos();
    let seed2  = since_the_epoch.as_secs() as u32;
    (seed1, seed2)
}


/// Simple example of using the random number generator.
/// Produces a list of 10 random numbers, between 0 and 0xFFFFFFFF.
fn main() {
    let seed = get_seed();

    // Seed can be replaced by any two u32 numbers.

    let mut r = random_unique::RandomSequenceOfUniqueU32::new(seed.0, seed.1);
    println!("Random numbers, with changing seed: ");
    for _i in 0..4{
        println!("    {}", r.next_nr());
    } 
    r.re_seed(0,0);
    println!("Random numbers, with fixed seed: ");
    for _i in 0..4{
        println!("    {}", r.next_nr());
    }
    r.re_seed(42, 127);
    println!("Deterministic Scrambling numbers u32 -> u32: ");
    for i in 0..4{
        println!("    {} -> {}",i, r.scramble_number(i));
    }
}