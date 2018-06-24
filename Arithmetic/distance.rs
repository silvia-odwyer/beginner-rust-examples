use std::io;

fn main() {

    loop {
        let mut feet = String::new();
        println!("Enter number of feet: ");
        io::stdin().read_line(&mut feet).expect("Error reading line :(");

        let feet : f64 = match feet.trim().parse() {
            Ok(feet) => feet,
            Err(_) => continue,
        };

        let feet_inches = convert_feet_inches(&feet);

        let feet_yards = convert_feet_yards(&feet);

        let feet_miles = convert_feet_miles(&feet);

        println!("Inches: {0}\nYards: {1}\nMiles: {2}", feet_inches, feet_yards, feet_miles);
    }

}

fn convert_feet_inches(&feet : &f64) -> f64 {
    &feet * 12.0
}

fn convert_feet_yards(&feet : &f64) -> f64 {
    &feet / 3.0
}

fn convert_feet_miles(&feet : &f64) -> f64 {
    &feet / 5280.0
}