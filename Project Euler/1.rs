fn main() {
    let end = 10;
    let mut sum = 0;

    for i in 1..end {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }

        if i == end - 1 {
            println!("Sum is: {:?}", sum);
        }
    }
}