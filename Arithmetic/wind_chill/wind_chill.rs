use std::io;

fn main() {
   let mut air_temp = String::new();
   let mut wind_speed = String::new();

   println!("Enter a float value for the air temperature.");

   io::stdin().read_line(&mut air_temp)
       .expect("Failed to read line.");

   println!("Enter a float value for the wind speed.");

   io::stdin().read_line(&mut wind_speed)
       .expect("Failed to read line.");

   let air_temp: f32 = match air_temp.trim().parse() {
       Ok(num) => num,
       Err(_) => 0.0
   };

   let wind_speed: f32 = match wind_speed.trim().parse() {
       Ok(num) => num,
       Err(_) => 0.0
   };

   // A number is raised to a power in Rust using the .pow() function
   // e.g., x.pow(2) 
   // However, the wind chill formula requires raising the wind speed variable
   // to a float, namely, 0.16, so the powf() funtion is used instead.
   let wind_chill = 13.12 + 0.6215*air_temp - 11.37*wind_speed.powf(0.16) + 0.3965*air_temp*wind_speed.powf(0.16);

   // The as keyword is used for converting types in Rust
   // We want only the integer part of the above calculation.
   // The number is not rounded, but instead the float part is truncated/discarded.
   // The as keyword allows for "safe casting", with this cast being a numeric cast.
   // let wind_chill_index: u32 = wind_chill as u32;
   println!("The index is {}.", wind_chill_index);
}
