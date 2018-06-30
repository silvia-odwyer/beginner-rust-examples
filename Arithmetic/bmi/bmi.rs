use std::io;

fn get_bmi_imperial(height : f64, weight : f64) -> f64 {
    (weight * 703.0) / (height * height)
}

fn get_bmi_metric(height : f64, weight : f64) -> f64 {
    weight / (height * height) 
}

fn main() {
    loop {
        let mut height = String::new();
        let mut weight = String::new();
        let mut choice = String::new();

        println!("Would you like the metric version or the imperial version?\n(Metric involves entering your height in metres and weight in kilos, and imperial involves height in inches, and weight in pounds.)");
        println!("Type imperial for imperial and metric for metric");
        io::stdin().read_line(&mut choice).expect("Could not read line :(");
        if choice == "metric" || choice == "imperial" {

        println!("Enter your height: ");
        io::stdin().read_line(&mut height);
        let height : f64 = match height.trim().parse() {
            Ok(height) => height,
            Err(_) => continue,
        };

        println!("Enter your weight");
        io::stdin().read_line(&mut weight);
        let weight : f64 = match weight.trim().parse() {
            Ok(weight) => weight,
            Err(_) => continue,
        };

        let metric = String::from("metric");
        let imperial = String::from("imperial");

        let bmi = match choice {
            _metric => get_bmi_imperial(height, weight),
            _imperial => get_bmi_metric(height, weight),
        };
        println!("Your BMI is: {}", bmi);
        }

        else {
            println!("You didn't type either metric or imperial");
            continue
        }


    }
}