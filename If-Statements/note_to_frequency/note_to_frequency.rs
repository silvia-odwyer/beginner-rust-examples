use std::io;

fn main() {
    let mut note_and_octave = String::new();

    println!("Please enter a note and an octave. Such as, C4.");
    io::stdin().read_line(&mut note_and_octave)
        .expect("Failed to read line.");
    
    // The next two lines define string slices.
    // The second index given is exclusive, meaning note and octave only have
    // one character each.
    let note = &note_and_octave[0..1];
    let octave = &note_and_octave[1..2];

    // The octave variable is used as an int in a calculation below, so
    // it is converted.
    let octave: i32 = match octave.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    let mut frequency = 0.0;
    
    // This code sets frequency to the frequency of the note
    // in the 4th octave. This means the 261.63Hz below is C4.
    if note == "C" {
        frequency = 261.63;
    } else if note == "D" {
        frequency = 293.66;
    } else if note == "E" {
        frequency = 329.63;
    } else if note == "F" {
        frequency = 349.23;
    } else if note == "G" {
        frequency = 392.00;
    } else if note == "A" {
        frequency = 440.00;
    } else if note == "B" {
        frequency = 493.88;
    }

    // This calculation doubles or halves the frequency to the 
    // correct octave. 
    //
    // Only f32s or f64s can be raised to a signed integer, such as an i32.
    // This is done with the powi() method. We define the base as an f32, and
    // the power as an i32. We annotate the type of 4 by appending its type after 
    // the number. Generally, it's best to do math operations with the same number type,
    // because the compiler often complains if you don't.
    // 
    // Notice again, that because frequency is an f32 , we can only divide by
    // either an f32 or f64, so we cast the expression safely to an f32 using "as f32".
    
    let base: f32 = 2.0;
    let power: i32 = 4i32 - octave;

    frequency = frequency / base.powi(power) as f32;
    println!("The frequency is {} Hz.", frequency);
}
