use std::io;

fn main() {
    let mut grade = String::new();

    println!("Please enter an American letter grade. (e.g, B-)");
    io::stdin().read_line(&mut grade)
        .expect("Failed to read line.");

    let grade = grade.trim().to_uppercase();
    let points;

    if grade == "A+" {
        points = 4.0;
    } else if grade == "A" {
        points = 4.0;
    } else if grade == "A-" {
        points = 3.7;
    } else if grade == "B+" {
        points = 3.3;
    } else if grade == "B" {
        points = 3.0;
    } else if grade == "B-" {
        points = 2.7;
    } else if grade == "C+" {
        points = 2.3;
    } else if grade == "C" {
        points = 2.0;
    } else if grade == "C-" {
        points = 1.7;
    } else if grade == "D+" {
        points = 1.3;
    } else if grade == "D" {
        points = 1.0;
    } else if grade == "F" {
        points = 0.0;
    } else {
        panic!("Invalid grade entered.");
    }

    println!("This gets {} points.", points);
 








}
