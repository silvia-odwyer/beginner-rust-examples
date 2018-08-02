use std::io;

fn main() {
    let mut cents = String::new();

    println!("Please enter a number of cents.");

    io::stdin().read_line(&mut cents)
        .expect("Failed to read line");

    let mut cents: u32 = match cents.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };
    
    let copy_of_cents = cents; // this copy is used in the output

    // These are Candadian coins (toonie = $2, loonie = $1)
    // Division of two integers in Rust results in an integer
    // If the division would mathematically result in a float, or
    // a remainder, the remainder is simply discarded.
    // e.g, 201/200 = 1 R 1 => 1
    // This makes sense because 201 cents contains 1 toonie.
    let toonies: u32 = cents / 200;
    cents -= toonies*200;

    let loonies: u32 = cents / 100;
    cents -= loonies*100;

    let quarters: u32 = cents / 25;
    cents -= quarters*25;

    let dimes: u32 = cents / 20;
    cents -= dimes*20;

    let nickels: u32 = cents / 5; 
    cents -= nickels*5;

    let pennies = cents;

    println!("{} cents makes:", copy_of_cents);

    println!("{} toonies,", toonies);
    println!("{} loonies,", loonies);
    println!("{} quarters,", quarters);
    println!("{} dimes,", dimes);
    println!("{} nickels,", nickels);
    println!("and {} pennies,", pennies);
    
}
