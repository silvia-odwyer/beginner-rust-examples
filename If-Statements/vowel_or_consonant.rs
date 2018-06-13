use std::str;
use std::io;

fn main() {
    let mut letter = String::new(); // should convert to char instead.
    loop {
        println!("Enter a single letter: ");
        io::stdin().read_line(&mut letter).expect("Could not read line :(");
        check_letter(&letter);
    }
}

fn check_letter(letter : &String) {
    let vowels = vec!["a", "e", "i", "o", "u"];

    for vowel in &vowels{
        if str::eq(vowel, &letter){
            println!("False");

        
    }
    println!("False");



}
}