extern crate rand;

use rand::Rng;

pub fn print_rands() {
    let mut rng = rand::thread_rng();
    let mut low = 1;
    let mut high = 255;

    for i in 1..10 {
        low *= -i;
        high *= i;
        println!(
            "Generate random integer ({}, {}) {} and bool -> {}",
            low,
            high,
            rng.gen_range(low, high),
            rng.gen::<bool>()
        );
    }
}
