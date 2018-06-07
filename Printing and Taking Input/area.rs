use std::io

fn main() {
    println!("Please enter the width of the room: ");

    let width = String::new();
    io::stdin().read_line(&mut width)
    .expect("Could not read line :(");

    let width: i32 = width.trim().parse(){
        Ok(num) => width,
        Err(_) => println!("Enter a number!")
    }

    let length = String::new();

    io::stdin().read_line(&mut length)
    .expect("Could not read line :(");

    let length = length.trim().parse(){
        Ok(num) => length,
        Err(_) => println!("Enter a number!")
    }

    let area = length * width;
    println!("The area is {}", area)
}