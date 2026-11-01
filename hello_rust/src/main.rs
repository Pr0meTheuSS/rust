use rand::prelude::*;

fn main() {
    let rand_number = rand::rng().random_range(1..=100);
    println!("The random number is: {rand_number}");
}

