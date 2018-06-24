use std::io;
use std::f64::consts;


fn main() {

    loop {
        let mut r = String::new();

        println!("Please enter the radius: ");

        io::stdin().read_line(&mut r).expect("Could not read line :(");

        let r : f64 = match r.trim().parse() {
            Ok(r) => r,
            Err(_) => continue,
        };

        let area : f64 = get_area(&r); 
        println!("Area: {}", area);

        let volume : f64 = get_sphere_volume(&r);
        println!("Volume: {}", volume)
    }

}

fn get_area(&r : &f64) -> f64 {
    consts::PI * &r.powf(2.0)
}

fn get_sphere_volume(&r : &f64) -> f64 {
    (consts::PI * &r.powf(3.0)) / 3.0
}