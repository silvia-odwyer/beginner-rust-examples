use std::io;

struct Triangle {
    side1 : f64, 
    side2 : f64, 
    side3 : f64
}

impl Triangle {
    fn get_area(&self) -> f64 {
        let s = (self.side1 + self.side2 + self.side3) / 2.0;
        (s * (s - self.side1) * (s - self.side2) * (s - self.side3)).sqrt()
    }
}

fn main() {
    loop {

        let mut side1 = String::new();

        let mut side2 = String::new();

        let mut side3 = String::new();

        println!("Enter side 1 of triangle: ");
        io::stdin().read_line(&mut side1).expect("Could not read line :(");

        let side1 : f64 = match side1.trim().parse() {
            Ok(side1) => side1,
            Err(_) => continue,
        };

        println!("Enter side 2 of triangle: ");
        io::stdin().read_line(&mut side2).expect("Could not read line :(");

        let side2 : f64 = match side2.trim().parse() {
            Ok(side2) => side2,
            Err(_) => continue,
        };

        println!("Enter side 3 of triangle: ");
        io::stdin().read_line(&mut side3).expect("Could not read line :(");

        let side3 : f64 = match side3.trim().parse() {
            Ok(side3) => side3,
            Err(_) => continue,
        };

        let triangle = Triangle { side1 : side1, side2 : side2, side3 : side3};

        let area = triangle.get_area();
        println!("Area: {}", area);
    }
}