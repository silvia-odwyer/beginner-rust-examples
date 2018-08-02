use std::io;

fn main() {
    let mut wavelength = String::new();

    println!("Please enter a wavelength of visible light in nm.");
    io::stdin().read_line(&mut wavelength)
        .expect("Failed to read line.");

    let wavelength: u32 = match wavelength.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid wavelength entered.")
    };
    
    let colour;

    if wavelength >= 380 && wavelength < 450 {
        colour = "violet";
    } else if wavelength >= 450 && wavelength < 495 {
        colour = "blue";
    } else if wavelength >= 495 && wavelength < 570 {
        colour = "green";
    } else if wavelength >= 570 && wavelength < 590 {
        colour = "yellow";
    } else if wavelength >= 590 && wavelength < 620 {
        colour = "orange";
    } else if wavelength >= 620 && wavelength < 750 {
       colour = "red";
    } else {
         panic!("Wavelength outside of visible spectrum.");
    }

    println!("This wavelength is {}.", colour);
}
