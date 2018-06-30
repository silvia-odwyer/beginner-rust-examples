use std::io;

struct Freefall {
    initial_v: f64,
    d : f64
}

impl Freefall {
    fn get_final_speed(&self) -> f64 {
        const ACCELERATION : f64 = 9.8;
        (self.initial_v.powf(2.0) + (2.0 * ACCELERATION * self.d)).sqrt()
    }
}

fn main() {
    loop {
        let mut height = String::new();

        println!("Enter the height from which the object is dropped: ");

        io::stdin().read_line(&mut height).expect("Could not read line :(");

        let height : f64 = match height.trim().parse() {
            Ok(height) => height,
            Err(_) => continue,
        };

        // Create a free-fall struct
        let freefall = Freefall { initial_v : 0.0, d : height};
        let final_speed = freefall.get_final_speed();
        println!("Final Speed Before Hitting The Ground: {}", final_speed)

    }
}