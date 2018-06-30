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
        let mut guess = n / 2.0;
        for _i in 1..100 {
            let guess = (guess + (n / 2.0)) / 2.0;
            println!("{}", guess);
        }
    }

}