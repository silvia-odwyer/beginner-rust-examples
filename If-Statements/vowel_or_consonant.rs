use std::io;

fn main() {
    let letter = String::new(); // should convert to char instead.
    loop {
        println!("Enter a single letter: ");
        io::stdin().read_line(&letter).expect("Could not read line :(");
        let result = check_letter(letter);
    }
}

fn check_letter(letter : String) {
    let vowels = vec!["a", "e", "i", "o", "u"];

    for vowel in &vowels{
        if vowel == letter{
            return false;
        }
    }
    else{
        return true;
    }

}