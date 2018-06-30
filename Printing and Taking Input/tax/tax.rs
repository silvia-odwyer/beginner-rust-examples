use std::io;

fn main() {
    loop {
        println!("Enter price of the meal: ");
        let mut cost = String::new();

        io::stdin().read_line(&mut cost)
        .expect("Could not read line :(");

        let cost : f64 = match cost.trim().parse(){
            Ok(cost) => cost,
            Err(_) => continue,
        };

        // Assuming a tax rate of 20% (VAT) on the meal
        let tax_rate = 0.2;
        let total_tax = cost - (cost * (1.0 - tax_rate));

        let tip = cost * 0.18;

        let cost = cost + tip;

        println!("Tip: {} \n Tax (included in original meal price) : {} \n Grand Total: {}", tip, total_tax, cost);
    }
}