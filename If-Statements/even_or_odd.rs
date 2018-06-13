use std::io;

fn main() {

    loop {
    let mut n = String::new();
    println!("Enter a number: ");
    io::stdin().read_line(&mut n)
    .expect("Could not read line :(");

    let n : i32 = match n.trim().parse(){
        Ok(n) => n,
        Err(_) => continue,
    };

    let res = n % 2;

    if res == 0 {
        println!("The number is even.")
    }
    else{
        println!("The number is odd.")
    }

    }
    

}