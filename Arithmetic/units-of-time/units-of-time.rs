use std::io;

struct Duration {
    days : f64,
    hours: f64,
    minutes : f64,
    seconds : f64
}

impl Duration {
    fn get_total_seconds(&self) -> f64 {
        self.days * 3600.0 * 24.0 + self.hours * 3600.0 + self.minutes * 60.0 + self.seconds 
    }
}

fn main() {
    loop {
        let mut days = String::new();
        let mut hours = String::new();
        let mut minutes = String::new();
        let mut seconds = String::new();

        println!("Enter the number of days: ");
        io::stdin().read_line(&mut days).expect("Could not read line :(");

        let days : f64 = match days.trim().parse() {
            Ok(days) => days,
            Err(_) => continue,
        };

        println!("Enter the number of hours: ");
        io::stdin().read_line(&mut hours).expect("Could not read line :(");
        let hours : f64 = match hours.trim().parse() {
            Ok(hours) => hours,
            Err(_) => continue,
        };

        println!("Enter the number of minutes: ");
        io::stdin().read_line(&mut minutes).expect("Could not read line :(");
        let minutes : f64 = match minutes.trim().parse() {
            Ok(minutes) => minutes,
            Err(_) => continue,
        };

        println!("Enter the number of seconds: ");
        io::stdin().read_line(&mut seconds).expect("Could not read line :(");
        let seconds : f64 = match seconds.trim().parse() {
            Ok(seconds) => seconds,
            Err(_) => continue,
        };

        // Create a duration Struct
        let duration = Duration { days: days, hours : hours, minutes : minutes, seconds : seconds};
        let duration_in_seconds = duration.get_total_seconds();
        println!("Duration in seconds: {}", duration_in_seconds);
    }
}