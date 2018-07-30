use std::f32::consts::PI;
use std::io;    

fn main() {
    let mut s = String::new();
    let mut n = String::new();

    println!("Please enter a float value for s.");

    io::stdin().read_line(&mut s)
        .expect("Failed to read line.");

    println!("Please enter a float value for n.");

    io::stdin().read_line(&mut n)
        .expect("Failed to read line.");

    let s: f32 = match s.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    let n: f32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0
    };
    
    // Both PI and n are floats, because this is how division in 
    // Rust is done. A float divided by an int is not allowed.
    // The area formula is: n * s^2
    //                      -----
    //                      4 * tan(PI/n) 
    let tan_value: f32 = PI/n;
    // Numbers are raised to a power using the pow() function
    // if the exponent is an integer.
    let area: f32 = n*s.pow(2)/4.0*tan_value.tan();
    println!("The area of this regular polygon is {} units squared.", area);
}
