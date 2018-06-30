extern crate time;
use time::PreciseTime;
use std::io;

fn main() {

    loop {
        
        let mut n = String::new();

        println!("Enter a number: ");

        io::stdin().read_line(&mut n).expect("Could not read line :(");
                let start = PreciseTime::now();
        let n : f64 = match n.trim().parse() {
            Ok(n) => n, 
            Err(_) => continue,
        };
        let end = n.sqrt().floor() as u64;
            for div in 2..end - 1 {
                let divf = div as f64;
                if n % divf == 0.0 {
                    break
                } 
                else {
                    if divf == n.sqrt().floor() - 2.0 {
                        println!("{} is prime.", n);
                    }
                    continue
                }
            }
    let end = PreciseTime::now();
    println!("{} seconds for whatever you did.", start.to(end));

    }
}