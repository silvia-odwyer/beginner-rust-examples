use std::io;
use std::f64::consts;

struct Cylinder {
    r : f64,
    height : f64
}

impl Cylinder {
    fn get_volume(&self) -> f64 {
        consts::PI * self.r.powf(2.0) * self.height
    }
}

fn main() {
    loop {
        let mut r = String::new();

        println!("Type in the radius.");
        io::stdin().read_line(&mut r).expect("Could not read line :(");

        let r : f64 = match r.trim().parse() {
            Ok(r) => r,
            Err(_) => continue,
        };

        let mut height = String::new();

        println!("Type in the height of the cylinder: ");
        io::stdin().read_line(&mut height).expect("Could not read line :(");

        let height : f64 = match height.trim().parse() {
            Ok(height) => height,
            Err(_) => continue,
        };
        
        println!("{}", height);
        let cylinder = Cylinder { r : r, height : height};

        let volume = cylinder.get_volume();
        println!("Volume of cylinder is {}", volume);

    
    }



}