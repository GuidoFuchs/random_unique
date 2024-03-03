use random_unique::random_unique;
use std::time::{SystemTime, UNIX_EPOCH};

fn get_seed() -> (u32, u32) {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let seed1  = since_the_epoch.subsec_nanos() as u32;
    let seed2  = since_the_epoch.as_secs() as u32;
    return (seed1, seed2);
}

fn main() {
    let seed = get_seed();

    let mut r = random_unique::RandomSequenceOfUniqueU32::new(seed.0, seed.1);
    for _i in 0..10{
        println!("{}", r.next());
    }
}