use std::io

fn main() {
    let a = String::new();
    io::stdin().read_line(&mut a)
    .expect("Could not read line. :(");

    let a = a.trim().parse(){
        Ok(num) => a, 
        Err(_) => println!("You did not enter a number!")
    }

    let b = String::new();

    io::std().read_line(&mut b)
    .expect("Could not read line :(");

    let b = b.trim.parse(){
        Ok(num) => b, 
        Err(_) => println!("You did not enter a number!")
    }

    let sum = a + b;
    let product = a * b;
    let difference = a - b;

    println!("Sum: {} \nProduct: {}\n Difference: {}", sum, product, difference);
}