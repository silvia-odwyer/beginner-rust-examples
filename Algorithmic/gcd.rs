use std::io;

fn main() {
    loop {

    println!("Enter a number: ");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Could not read line :(");

    let n : f64 = match n.trim().parse() {
        Ok(n) => n, 
        Err(_) => continue,
    };

    println!("Enter a number: ");
    let mut m = String::new();
    io::stdin().read_line(&mut m).expect("Could not read line :(");

    let m : f64 = match m.trim().parse() {
        Ok(m) => m, 
        Err(_) => continue,
    };

    let mut d = match n < m {
        true => n, 
        false => m,
    };
    println!("M: {} N: {}", m, n);
    println!("D: {}", d);

    loop {
        let res = m % d;
        println!("{}", res);
        if m % d == 0.0 || n % d == 0.0 {
            println!("MODULO ZERO DIVISION {}", d);
            break
        }
        d =  d - 1.0;
        println!("{}", d);
    }
    }

}