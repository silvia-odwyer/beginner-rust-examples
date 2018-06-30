use std::io;

fn main() {

    loop {
        let mut n = String::new();

        println!("Enter a number you would like to get the factorial of:");

        io::stdin().read_line(&mut n).expect("Could not read line :(");

        let n : u32 = match n.trim().parse() {
            Ok(n) => n, 
            Err(_) => continue,
        };
        let mut factorial = 1;

        for i in 2..n + 1 {
            factorial = factorial * i;
            if i == n {
                println!("Factorial: {}", factorial);
            }
        }
    }
}