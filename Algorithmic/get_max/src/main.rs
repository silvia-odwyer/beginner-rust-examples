extern crate time;
use time::PreciseTime;
use rand::prelude::*;
extern crate rand;

fn main() {
    let start = PreciseTime::now();
    let vec = get_random_vec();
    let mut max = &vec[0];
    let mut index = 0;
    for num in &vec {
        if num > max {
            max = &num;
        }

        if index == vec.len() - 1 {
            println!("Max number is: {}", max);
        }
        index += 1;
    }

    let end = PreciseTime::now();
    println!("{} seconds", start.to(end));
}

fn get_random_vec() -> std::vec::Vec<u32>{
    let mut vec = vec![];
    for _i in 1..1000000 {
        let num : u32 = rand::thread_rng().gen_range(0, 1000001);
        vec.push(num);
    }
    vec
}