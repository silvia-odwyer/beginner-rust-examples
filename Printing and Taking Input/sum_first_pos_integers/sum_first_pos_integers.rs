use std::io;

fn main() {

    loop {
        println!("This program finds the sum to the first n positive integers.");
        println!("Enter a number (n):");
        let mut n = String::new();

        io::stdin().read_line(&mut n)
        .expect("Could not read line :(");

        let n : u32 = match n.trim().parse(){
            Ok(n) => n, 
            Err(_) => continue,
        };

        let sum = (n * (n + 1)) / 2;

        println!("Sum is: {}", sum);
    }  
}