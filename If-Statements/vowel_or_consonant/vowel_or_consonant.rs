use std::io;

fn main() {
    let mut letter = String::new();

    println!("Please enter a one letter of the alphabet.");
    io::stdin().read_line(&mut letter)
        .expect("Failed to read line.");

    let letter: char = match letter.trim().parse() {
        Ok(letter) => letter,
        Err(_) => 'z'
    };
    
    // The String::from() method allows us to create a String struct
    // from a string literal, ie text wrapped in quotes.
    let vowels = String::from("aeiou");
    
    // The String::contains() method will return true if the letter 
    // that we pass in as an argument is contained within the String
    // vowels. If so, print out the message saying it is a vowel, if
    // not print the message saying it is a consonant.
    if vowels.contains(letter) {
        println!("The letter {} is a vowel.", letter);
    }
    else {
        println!("The letter {} is a consonant.", letter);
    }
}
