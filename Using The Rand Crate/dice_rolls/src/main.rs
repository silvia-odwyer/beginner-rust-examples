extern crate time;
use time::PreciseTime;
use rand::prelude::*;
extern crate rand;

fn main() {

    let start = PreciseTime::now();

    let mut rng = thread_rng();

    if rng.gen() {
    let vec = vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let mut freq_vec = vec![0 ; 11];
    let end = 100000000;
    let mut index_into = 0;
    for i in 1..end {
        let mut num = rng.gen_range(1, 7);
        let mut num2 = rng.gen_range(1, 7);
        let sum = num + num2;
        let index = vec.iter().position(|&r| r == sum).unwrap();
        freq_vec[index] += 1;

        if i == end - 1 {
            // println!("{:?}", freq_vec);
            for freq in &freq_vec {
                let num_display = &vec[index_into];
                index_into += 1;
                // println!("Num: {} {}", num_display, freq);
            }

        }
    }

    let end = PreciseTime::now();
    println!("{} seconds for whatever you did.", start.to(end));

}
}
