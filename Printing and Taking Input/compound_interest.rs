use std::io;

fn main() {
    println!("Enter amount deposited into account initially :");

    let amount = String::new();

    io::stdin().read_line(&mut amount)
    .expect("Could not read line :(");

    let amount = amount.trim().parse(){
        Ok(num) => amount,
        Err(_) => println!("You did not enter a number!")
    }

    let interest = 0.04;
    let amount_with_interest = amount;
    for x in 0..3 {
        amount_with_interest = amount_with_interest * 0.04
    }

    println!("Amount after 3 years: {}", amount_with_interest)

}