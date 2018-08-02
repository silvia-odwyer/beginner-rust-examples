use std::io;

fn main() {       
    let mut mpg = String::new();

    println!("Please enter a float value for the mpg.");
    io::stdin().read_line(&mut mpg)
        .expect("Failed to read line");

    let mpg: f32 = match mpg.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0
    };
    
    // Only a float divided by a float will produce another float in Rust
    // If we had taken in mpg as an int, we wouldn't be able to divide it 
    // by 235.21, because Rust doesn't allow for this.
    // Note: This formula is for US Gallons
    let l100 = 235.21/mpg; 
    println!("{} miles/gallon is {} litres/100km.", mpg, l100); 
}
