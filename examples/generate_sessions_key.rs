use rand::distributions::{Alphanumeric, DistString};

fn main() {
    let key = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);
    println!("{:?}", key);
}