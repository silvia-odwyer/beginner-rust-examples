use std::io;

fn main() {
    let mut letter = String::new();
    let letter2 = String::from("a");
    io::stdin().read_line(&mut letter).expect("Could not read line :(");

    let letter = letter.trim();
    let vowel_vec = vec!["a", "e", "i", "o", "u"];

    for vowel in &vowel_vec {
        if &letter2 == vowel {
            println!("Equal");
        }
        else {
            println!("Not equal.")
        }
    }
}