extern crate rand;
extern crate time;

use rand::Rng;
use time::PreciseTime;

fn main() {
    let start = PreciseTime::now();

    for x in 1..10000{
        println!("{}", x);
    }
    let end = PreciseTime::now();

    println!("{} seconds", start.to(end));
}