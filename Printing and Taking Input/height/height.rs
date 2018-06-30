use std::io;

struct Height {
    feet : f64, 
    inches : f64,
}
fn main() {

    loop {
        let mut feet = String::new();
        let mut inches = String::new();

        println!("Enter the number of feet.");

        io::stdin().read_line(&mut feet).expect("Enter a number please");

        println!("Enter the number of inches.");
        io::stdin().read_line(&mut inches).expect("Please, enter a number.");

        let feet: f64 = match feet.trim().parse(){
            Ok(feet) => feet,
            Err(_) => continue,
        };

        let inches : f64 = match inches.trim().parse() {
            Ok(inches) => inches,
            Err(_) => continue,
        };

        let height1 = Height { feet : feet, inches: inches};


        let feet_in_inches = convert_feet_inches(&feet);
        let total_inches = add_inches(&feet_in_inches, &inches);
        let total_cms = convert_feet_cms(&total_inches);
        println!("Feet: {}\nInches: {}", height1.feet, height1.inches);
        println!("Your Height In cms: {}", total_cms);
    }

    fn convert_feet_inches(&feet : &f64) -> f64{
        &feet * 12.0
    }

    fn add_inches(&feet_in_inches : &f64, &inches : &f64) -> f64 {
        &feet_in_inches + &inches
    }

    fn convert_feet_cms(&total_inches : &f64) -> f64 {
        &total_inches * 2.54
    }
}