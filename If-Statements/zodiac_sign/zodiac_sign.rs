use std::io;

fn main() {
    let mut month = String::new();
    let mut day = String::new();

    println!("Please enter the month you were born in.");
    io::stdin().read_line(&mut month)
        .expect("Failed to read line.");

    println!("Please enter the day you were born on.");
    io::stdin().read_line(&mut day)
        .expect("Failed to read line.");
    
    // day is converted to a number for use below.
    let day: u32 = match day.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Could not process input.")
    };
    
    // We use the to_lowercase() method so that the user can type in the month
    // in any permutation of upper and lower cases and the program will still be
    // able to process it in the if statements.
    //
    // The trim() method is used to remove the "\n" character which the read_line() function
    // above appends to all Strings passed in.
    let month = month.to_lowercase().trim();
    
    // Rust checks to see if there is a possibility that sign mightn't be initialized.
    // The key is that an else branch is also included, which panics, because the
    // month entered is, by that stage, invalid, since it did not match any of the string literals.
    // 
    // The compiler understands that by the stage the variable is used, in the println! macro
    // below, it must have been initialized, or that line of code will not be executed, because
    // the panic! macro was executed, which terminates the program.
    let sign;
    

    if month == "december" {
        if day >= 22 {
            sign = "Capricorn";
        }
        else {
            sign = "Sagittarius";
        }
    }
    else if month == "january" {
        if day >= 20 {
            sign = "Aquarius";
        }
        else {
            sign = "Capricorn";
        }
    }
    else if month == "february" {
        if day >= 19 {
            sign = "Pisces";
        }
        else {
            sign = "Aquarius";
        }
    }

    else if month == "march" {
        if day >= 21 {
            sign = "Aries";
        } 
        else {
            sign = "Pisces";
        }
    }
    else if month == "april" {
        if day >= 20 {
            sign = "Taurus";
        }
        else {
            sign = "Aries";
        }
    }
    else if month == "may" {
        if day >= 21 {
            sign = "Gemini";
        }
        else {
            sign = "Taurus";
        }
    }
    else if month == "june" {
        if day >= 21 {
            sign = "Cancer";
        }
        else {
            sign = "Gemini";
        }
    }
    else if month == "july" {
        if day >= 23 {
            sign = "Leo";
        }
        else {
            sign = "Cancer";
        }
    }
    else if month == "august" {
        if day >= 23 {
            sign = "Virgo";
        }
        else {
            sign = "Leo";
        }
    }
    else if month == "september" {
        if day >= 23 {
            sign = "Libra";
        }
        else {
            sign = "Virgo";
        }
    }
    else if month == "october" {
        if day >= 23 {
            sign = "Scorpio";
        }
        else {
            sign = "Libra";
        }
    }
    else if month == "november" {
        if day >= 22 {
            sign = "Sagittarius";
        }
        else {
            sign = "Scorpio";
        }
    }

    else {
        panic!("Invalid month entered.");
    }

    println!("You are a {}!", sign);

}
