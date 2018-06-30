use std::io;

fn main() {
    loop {

        let c_water : f64 = 4180.0;
        let mut mass = String::new();

        println!("Enter the mass (in grams): ");
        io::stdin().read_line(&mut mass).expect("Could not read line :(");

        println!("Enter the temperature change: ");
        let mut temp_change = String::new();
        io::stdin().read_line(&mut temp_change).expect("Could not read line :(");

        let mass : f64 = match mass.trim().parse() {
            Ok(mass) => mass,
            Err(_) => continue,
        };

        let temp_change : f64 = match temp_change.trim().parse() {
            Ok(mass) => mass,
            Err(_) => continue,
        };

        // Formula Q = mcAT
        let q : f64 = get_energy_req(&mass, &temp_change, &c_water);
        println!("Heat Energy Required To Bring {} grams of water to {} degrees Celsius", mass, temp_change);
        println!("= {} Joules", q);
    }
}

fn get_energy_req(&mass : &f64, &temp_change : &f64, &c_water : &f64) -> f64 {
    &mass * &c_water * &temp_change
}