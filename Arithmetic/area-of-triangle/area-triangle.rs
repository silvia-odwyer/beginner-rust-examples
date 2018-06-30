use std::io;

struct Triangle {
    base : f64,
    height : f64
}

impl Triangle {
    fn get_area(&self) -> f64 {
        (self.base * self.height) / 2.0
    }
}
fn main() {
    loop {
        let mut base = String::new();
        let mut height = String::new();

        println!("Base of triangle: ");
        io::stdin().read_line(&mut base).expect("Could not read line :(");
        let base : f64 = match base.trim().parse() {
            Ok(base) => base,
            Err(_) => continue,
        };

        println!("Height of triangle: ");
        io::stdin().read_line(&mut height).expect("Could not read line :(");

        let height : f64 = match height.trim().parse() {
            Ok(height) => height,
            Err(_) => continue,
        };

        let triangle = Triangle {base : base, height : height};
        let area = triangle.get_area();
        println!("AREA: {}", area);
    }
}