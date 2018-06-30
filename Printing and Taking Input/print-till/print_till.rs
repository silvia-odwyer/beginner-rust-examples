use std::io;
use std::cmp::Ordering;

fn main(){
    loop {

    println!("This program prints until a certain number.");

    println!("Enter a number: ");

    let mut end = String::new();

    io::stdin().read_line(&mut end)
    .expect("Could not read line! :/");

    let end : i32 = match end.trim().parse() {
        Ok(end) => end, 
        Err(_) => continue,
    };

    println!("You entered {}", end);
    let mut res = 0;

    loop {
        res = inc_num(&res);
        match res.cmp(&end){
            Ordering::Less => println!("{}", res),
            Ordering::Greater => break,
            Ordering::Equal => println!("{}", res),
        }
    } 
    }

}

fn inc_num(num : &i32) -> i32{
    num + 1
}