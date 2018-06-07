use stdin::io;

fn main() {
    let n = String::new();
    
    io::stdin().read_line()
    .expect("Could not read line :(");

    let n : u32 = n.trim().parse(){
        Ok(num) => n,
        Err(_) => println!("You did not enter a number!")
    }

    let res = n % 2;

    if res == 0 {
        println!("The number is even.")
    }
    else{
        println!("The number is odd.")
    }

}