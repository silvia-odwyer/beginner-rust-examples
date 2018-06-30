extern crate time;
use time::PreciseTime;
use rand::prelude::*;
extern crate rand;

fn main() {

    let start = PreciseTime::now();
    
    let end = 1000000;

    let mut res_vec = vec![];
    let mut i = 0;
    let mut avg_vec = vec![];

    for j in 0..end {
        res_vec = vec![];
        i = 0;
        loop {
            let num : u32 = rand::thread_rng().gen_range(0, 2);

            res_vec.push(num);

            if i > 1 && res_vec[i] == res_vec[i - 1] && res_vec[i - 1] == res_vec[i - 2]{
                // println!("Required {} flips to get three consecutive equal results", res_vec.len());
                avg_vec.push(res_vec.len());
                break
            }

            // if i == end - 1 {
            //     println!("Vec: {:?}", res_vec)
            // }

            i += 1;
        }

        if j == end - 1 {
            let mut total_flips = 0;
            let mut k = 0;
            for num in &avg_vec {
                total_flips += num;
                k += 1;
                if k == avg_vec.len() - 1 {
                    println!("Average flips needed: {}", total_flips / avg_vec.len());
                }
            }
        }
    }

    let end = PreciseTime::now();
    println!("{} seconds for whatever you did.", start.to(end));
}