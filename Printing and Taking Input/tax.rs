use std::io;

fn main() {
    let cost = String::new();

    io::stdin().read_line()
    .expect("Could not read line :(");

    let cost : u32 = cost.trim().parse(){
        Ok(num) => cost,
        Err(_) => println!("You did not enter a number!")
    }

    // Assuming a tax rate of 20% (VAT) on the meal
    let tax_rate = 0.2;
    let total_tax = cost - (cost * 0.8);


    let tip = cost * 0.18;

    let cost = cost + tip;

    println!("Tip: \n Tax (included in original meal price) : {} \n Grand Total: {}", tip, total_tax, cost)


}