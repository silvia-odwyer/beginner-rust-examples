use std::io;

fn main() {
            let mut vec = vec!["a"];
    loop { 

        let mut string = String::new();

        println!("Enter a string: ");
        io::stdin().read_line(&mut string).expect("Could not read line :(");

        string = string.trim().to_string();

        vec.push(string)

    }
}