use std::io;

fn main() {
    let mut square = String::new();

    println!("Please enter a chess board position (e.g, A6)");
    io::stdin().read_line(&mut square)
        .expect("Failed to read line.");

    let column = &square[0..1];
    let row = &square[1..2];

    let columns_starting_with_black = String::from("ACEG");
    let column_starts_on_black;
    
    if columns_starting_with_black.contains(column) {
        column_starts_on_black = true;
    }
    else {
        column_starts_on_black = false;
    }

    let row: u32 = match row.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Could not convert to row to u32.")
    };
    
    // Nesting if statements works as expected. Nothing fancy.
    //
    // row % 2 == 0 checks if the row is a multiple of 2. With this 
    // information, all we need to check is what colour the column starts on
    // and we can tell which colour it is. Have a look at a chess board with the 
    // columns and rows labelled, and it should all make sense.
    if row % 2 == 0 {
        if column_starts_on_black {
            println!("It's white.");
        }
        else {
            println!("It's black.");
        }
    }
    else {
        if column_starts_on_black {
            println!("It's black.");
        }
        else {
            println!("It's white.");
        }
    }
}

