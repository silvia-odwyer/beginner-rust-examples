use std::io

fn main() {

    // Get Length
    println!("Enter length (in feet)");

    let length = String::new();

    io::stdin().read_line(&mut length)
    .expect("Could not read line :(");

    let length : u32 = length.trim().parse(){
        Ok(num) => length,
        Err(_) => println!("Enter a number!")
    }
    
    // Get Width
    let width = String::new();
    println!("Enter width (in feet)");

    io::stdin().read_line()
    .expect("Could not read line :(");

    let width : u32 = width.trim().parse(){
        Ok(num) => width, 
        Err(_) => println!("Enter a number, please!")
    }

    // Get Area (in feet)
    let area = width * length;

    // Convert To Acres
    let area_acres = area / 43560;
    println!("Area in acres: {}", area_acres);


}