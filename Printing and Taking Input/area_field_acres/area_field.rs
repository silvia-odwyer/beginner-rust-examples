use std::io;

fn main() {
    loop {
        // Get Length
        println!("Enter length (in feet)");

        let mut length = String::new();

        io::stdin().read_line(&mut length)
        .expect("Could not read line :(");

        let length : f64 = match length.trim().parse(){
            Ok(length) => length,
            Err(_) => continue,
        };
        
        // Get Width
        let mut width = String::new();
        println!("Enter width (in feet)");

        io::stdin().read_line(&mut width)
        .expect("Could not read line :(");

        let width : f64 = match width.trim().parse(){
            Ok(width) => width, 
            Err(_) => continue,
        };

        // Get Area (in feet)
        let area = width * length;

        // Convert To Acres
        let area_acres = area / 43560.0;
        println!("Area in acres: {}", area_acres);
    }

}