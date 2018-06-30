extern crate time;
use time::PreciseTime;
use rand::prelude::*;
extern crate rand;

fn main() {
    let start = PreciseTime::now();

    let mut vec = get_random_vec();

    for _ in 0..vec.len() - 1 {
        for index in 0..vec.len() - 1 {
            let mut current_num = vec[index];
            let mut next_num = vec[index + 1];

            if current_num > next_num {
                let temp = current_num;
                vec[index] = next_num;
                vec[index + 1] = temp;
            }
        }
    }

    // println!("{:?}", vec);
    let end = PreciseTime::now();
    println!("{} seconds", start.to(end));

}

fn get_random_vec() -> std::vec::Vec<u32>{
    let mut vec = vec![];
    for _i in 0..100000 {
        let num : u32 = rand::thread_rng().gen_range(0, 1000001);
        vec.push(num);
    }
    vec
}
