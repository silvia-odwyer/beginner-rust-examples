fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // You can also reverse your iter, should you need it:
    for number in (1..10).rev() {
        println!("{}!", number);
    }
    
}