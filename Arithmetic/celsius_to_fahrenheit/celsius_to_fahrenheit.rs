use std::io;

fn main() {
    loop {
        let celsius = String::new();

        println!("Enter Celsius: ");
        io::stdin().read_line(&mut celsius).expect("Could not read line :(");

        let celsius : f64 = match celsius.trim().parse() {
            Ok(celsius) => celsius,
            Err(_) => continue,
        };

        let fahrenheit = celsius_to_fahrenheit(celsius);
        let kelvin = celsius_to_kelvin(celsius);
        println!("Fahrenheit: {}", fahrenheit);
        println!("Kelvin: {}", kelvin)

    }
}

fn celsius_to_fahrenheit(celsius : f64) -> f64 {
    celsius 
}

fn celsius_to_kelvin(celsius : f64) -> f64 {
    celsius 
}