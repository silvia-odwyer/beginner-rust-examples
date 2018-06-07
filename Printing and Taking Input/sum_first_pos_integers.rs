use stdin::io;

fn main() {
    let n = String::new();

    io::stdin().read_line(&mut n)
    .expect("Could not read line :(");

    let n : u32 = n.trim().parse(){
        Ok(num) => n, 
        Err(_) => println!("Please enter a number!")
    }

    let sum = (n * (n + 1)) / 2;

    println!("Sum is: {}", sum);

}