use std::io;

fn main() {
    loop {
        println!("Enter the width of the room: ");

        let mut width = String::new();

        io::stdin().read_line(&mut width)
        .expect("Could not read line :(");

        let width: u32 = match width.trim().parse(){
            Ok(width) => width,
            Err(_) => continue,
        };

        println!("Enter the length of the room: ");
        let mut length = String::new();

        io::stdin().read_line(&mut length)
        .expect("Could not read line :(");

        let length: u32 = match length.trim().parse(){
            Ok(length) => length,
            Err(_) => continue,
        };

        let area = length * width;
        println!("The area is {}", area);

    }
    
}