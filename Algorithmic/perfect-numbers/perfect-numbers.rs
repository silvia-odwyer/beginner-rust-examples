use std::io;

fn main() {
    loop {
        let mut n = String::new();

        println!("Enter a number: ");
        io::stdin().read_line(&mut n).expect("Could not rad line.");

        let n : u64 = match n.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        let mut sum = 0;
        for i in 1..n - 1 {
            let div = n % i;
            if div == 0 {
                sum = sum + i; //keep track of the moving sum here.
            }

            if i == n - 2 && sum == n {
                println!("{} is perfect", n);
            }
        }
    }
}