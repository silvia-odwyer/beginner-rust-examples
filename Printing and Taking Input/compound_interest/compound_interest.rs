use std::io;

fn main() {
    loop {
        println!("Enter amount deposited into account initially :");

        let mut amount = String::new();

        io::stdin().read_line(&mut amount)
        .expect("Could not read line :(");

        let amount: f64 = match amount.trim().parse(){
            Ok(amount) => amount,
            Err(_) => continue,
        };

        let interest = 0.04;
        let mut amount_with_interest = amount;
        println!("{}", amount_with_interest);
        for _x in 0..3 {
            amount_with_interest = amount_with_interest + (amount_with_interest * interest);
            println!("{}", amount_with_interest);
        };

        println!("Amount after 3 years: {}", amount_with_interest);
    }
}