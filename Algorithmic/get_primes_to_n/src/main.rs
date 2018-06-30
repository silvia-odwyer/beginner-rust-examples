extern crate time;
use time::PreciseTime;
use std::io;

fn main() {

    loop {
        
        let mut limit = String::new();

        println!("Enter a number: ");

        io::stdin().read_line(&mut limit).expect("Could not read line :(");
                let start = PreciseTime::now();
        let limit : u64 = match limit.trim().parse() {
            Ok(limit) => limit, 
            Err(_) => continue,
        };
        for n in 2..limit {
            let n = n as f64;
            let end = n.sqrt().floor() as u64;
                for div in 2..end - 1 {
                    let divf = div as f64;
                    if n % divf == 0.0 {
                        break
                    } 
                    else {
                        if divf == n.sqrt().floor() - 2.0 {
                            println!("{}", n);
                        }
                        continue
                    }
                }
        }
    let end = PreciseTime::now();
    println!("{} seconds for whatever you did.", start.to(end));

    }
}

