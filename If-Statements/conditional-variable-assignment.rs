fn main() {
    let condition = true;
    let month = if condition {
        "May"
    } else {
        "January"
    };

    println!("The month is: {}", month);
}