use std::io;
use std::cmp;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();
    
    println!("Enter a value for a.");
    io::stdin().read_line(&mut a)
        .expect("Failed to read line.");

    println!("Enter a value for b.");
    io::stdin().read_line(&mut b)
        .expect("Failed to read line.");

    println!("Enter a value for c.");
    io::stdin().read_line(&mut c)
        .expect("Failed to read line.");

    let a: i32 = match a.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    let b: i32 = match b.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    let c: i32 = match c.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };
    
    // To computate the max of two numbers the cmp::max() function is used.
    // Since it only takes in 2 numbers at a time, first, we find the 
    // max between a and b, and then compare that against c.
    let max = cmp::max(cmp::max(a, b), c);
    // The cmp package also provides a min() function which works like max().
    let min = cmp::min(cmp::min(a, b), c);
    // The sum of three integers minus the max and middle gives us the middle
    // value without knowing exactly which values max or min are.
    let middle = a + b + c - max - min;

    println!("The max is {}", max);
    println!("The middle value is {}", middle);
    println!("The min is {}", min);
}
