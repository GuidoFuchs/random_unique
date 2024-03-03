use random_unique::random_unique;
use std::time::{SystemTime, UNIX_EPOCH};

/// get_seed uses the system time, to produce two somewhat random numbers
fn get_seed() -> (u32, u32) {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let seed1  = since_the_epoch.subsec_nanos() as u32;
    let seed2  = since_the_epoch.as_secs() as u32;
    return (seed1, seed2);
}


/// Simple example of using the random number generator.
/// Produces a list of 10 random numbers, between 0 and 0xFFFFFFFF.
fn main() {
    let seed = get_seed();

    // Seed could be replaced by any two u32 numbers.
    let mut r = random_unique::RandomSequenceOfUniqueU32::new(seed.0, seed.1);
    for _i in 0..10{
        println!("{}", r.next());
    }
}