use std::str;
use std::io;

fn main() {
    let mut letter : char = ' ';  // should convert to char instead.
    loop {
        println!("Enter a single letter: ");
        io::stdin().read_line(&mut letter).expect("Could not read line :(");
        check_letter(&letter);
    }
}

fn check_letter(letter : &char) {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    if vowels.contains(&letter){
        println!("contains");
    }
}