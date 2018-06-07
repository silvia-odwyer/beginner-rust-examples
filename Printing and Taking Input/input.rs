use std::io

fn main(){
    let name = String::new()

    io::stdin().read_line().(&mut name)
    .expect("Could not read line!");

    println!("Hello, {}", name)

}