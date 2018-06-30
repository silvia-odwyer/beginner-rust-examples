use std::io;

fn main(){
    println!("What is your name?");
    let mut name = String::new();

    io::stdin().read_line(&mut name)
    .expect("Could not read line!");

    println!("Hello, {}", name);

}