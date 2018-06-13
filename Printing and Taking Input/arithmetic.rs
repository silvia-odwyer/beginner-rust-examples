use std::io;

fn main() {

    loop {
    let mut a = String::new();

    println!("Enter a number: ");

    io::stdin().read_line(&mut a)
    .expect("Could not read line. :(");

    let a: i32 = match a.trim().parse(){
        Ok(a) => a, 
        Err(_) => continue,
    };
    println!("Enter a number: ");
    let mut b = String::new();

    io::stdin().read_line(&mut b)
    .expect("Could not read line :(");

    let b: i32 = match b.trim().parse(){
        Ok(b) => b, 
        Err(_) => continue, 
    };

    let sum = a + b;
    let product = a * b;
    let difference = a - b;

    println!("Sum: {} \nProduct: {}\nDifference: {}", sum, product, difference);

    }
   
}