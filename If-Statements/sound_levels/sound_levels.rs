use std::io;

fn main() {
    let mut sound_level = String::new();
    
    println!("Enter a sound level in dB");
    io::stdin().read_line(&mut sound_level)
        .expect("Failed to read line.");

    let sound_level: u32 = match sound_level.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    // The == operator checks if both values are the same
    if sound_level == 130 {
        println!("This is equivalent to a jackhammer.");
    } else if sound_level == 106 {
        println!("This is equivalent to a lawnmower.");
    } else if sound_level == 70 {
        println!("This is equivalent to an alarm clock.");
    } else if sound_level == 40 {
        println!("This is equivalent to a quiet room.");
    }

    // The following operators can also be used for conditional programming in Rust:
    // < - less than 
    // > - greater than
    // <= - less than or equal to
    // >= - greater than or equal to
    // && - and 
    // || - or
    // ! - not
    // != - not equal to 
    if sound_level < 40 {
        println!("This is quieter than a quiet room.");
    } else if sound_level > 40 && sound_level < 70 {
        println!("This is louder than a quiet room, but quieter than an alarm clock.");
    } else if sound_level > 70 && sound_level < 106 {
        println!("This is louder than an alarm clock, but quieter than a lawnmower.");
    } else if sound_level > 106 && sound_level < 130 {
        println!("This is louder than a lawnmower, but quieter than a jackhammer.");
    } else if sound_level > 130 {
        println!("This is louder than a jackhammer.");
    }
}
