use std::io;
struct GasProblem {
    temperature : f64,
    pressure : f64,
    volume : f64
}

impl GasProblem {
    fn get_amt_gas(&self) -> f64 {
        // Formula : PV/RT
        const IDEALGASCONSTANT : f64 = 8.314;
        (self.pressure * self.volume) / (IDEALGASCONSTANT * self.temperature)
    }
}

fn main() {
    loop {
        // Formula PV = nRT 
        let mut pressure = String::new();
        let mut volume = String::new();
        let mut temperature = String::new();

        println!("Pressure: ");
        io::stdin().read_line(&mut pressure).expect("Could not read line :(");

        let pressure : f64 = match pressure.trim().parse() {
            Ok(pressure) => pressure,
            Err(_) => continue,
        };

        println!("Volume: ");
        io::stdin().read_line(&mut volume).expect("Could not read line :(");
        let volume : f64 = match volume.trim().parse() {
            Ok(volume) => volume,
            Err(_) => continue,
        };

        println!("Temperature: (Kelvins) ");
        io::stdin().read_line(&mut temperature).expect("Could not read line :(");

        let temperature : f64 = match temperature.trim().parse() {
            Ok(temperature) => temperature,
            Err(_) => continue,
        };

        let gas_problem = GasProblem { temperature : temperature, volume : volume, pressure : pressure};
                // Convert temperature to Kelvins

        let amt_gas = gas_problem.get_amt_gas();
        println!("Amount of gas left in the tank: {}", amt_gas);
    }
}