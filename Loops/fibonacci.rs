use std::io;

fn main () {
    loop {
    let mut n = String::new();
    println!("Please enter a number: ");
    io::stdin().read_line(&mut n).expect("Please enter a number.");

    let n : u32 = match n.trim().parse(){
    Ok(n) => n,
    Err(_) => break,
    };


    let mut sum = 0;

    for x in 1..n {
        sum = sum + x;
        if x == n - 1{
            println!("Sum: {}", sum + x + 1)
        }
    };
    }

}