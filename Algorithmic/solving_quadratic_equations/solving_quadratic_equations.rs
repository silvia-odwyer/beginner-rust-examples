use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Please enter a float value for a.");
    io::stdin().read_line(&mut a)
        .expect("Failed to read line.");

    println!("Please enter a float value for b.");
    io::stdin().read_line(&mut b)
        .expect("Failed to read line.");

    println!("Please enter a float value for c.");
    io::stdin().read_line(&mut c)
        .expect("Failed to read line.");

    let a: f32 = match a.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Couldn't process variable a.")
    };

    let b: f32 = match b.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Couldn't process variable b.")
    };

    let c: f32 = match c.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Couldn't process variable c.")
    };

    let discriminant: f32 = b.powf(2.0) - 4.0*a*c;

    if discriminant < 0.0 {
        println!("This quadratic equation does not have any real roots.");
    }
    else if discriminant == 0.0 {
        let root: f32 = -b / 2.0*a;
        println!("One real root found: {}.", root);
    }
    else {
        let root1: f32 = (-b + discriminant.sqrt()) / 2.0*a;
        let root2: f32 = (-b - discriminant.sqrt()) / 2.0*a;
        println!("Two real roots found: {}, {}.", root1, root2);
    }
}
