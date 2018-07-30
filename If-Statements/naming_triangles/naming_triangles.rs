use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Please enter a value for a.");
    io::stdin().read_line(&mut a)
        .expect("Failed to read line.");

    println!("Please enter a value for b.");
    io::stdin().read_line(&mut b)
        .expect("Failed to read line.");

    println!("Please enter a value for c.");
    io::stdin().read_line(&mut c)
        .expect("Failed to read line.");

    let a: u32 = match a.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };
    let b: u32 = match b.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };
    let c: u32 = match c.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };
    
    // We can use the || operator to check if a is equal to b,
    // or a is equal to b, or b is equal to c.
    // Note: rust does not support the use of brackets in if statements
    if a == b && a == c && b == c {
        println!("This triangle is an equilateral.");
    } else if a == b || a == c || b == c {
        println!("This triangle is an isosceles.");
    } else {
        println!("This triangle is a scalene.");
    }

}
