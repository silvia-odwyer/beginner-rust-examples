fn main() {
    let v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("Element: {}", i)
    }

    // Once you have iterated over an array once, you cannot iterate over
    // it again. 
    // You can however iterate over a reference of the array.

    // EXAMPLE OF MULTIPLE ITERATIONS OVER AN IDENTICAL VECTOR

    let vec1 = vec![1, 2, 3, 4, 5];

    for i in &vec1 {
        println!("Element: {}", i)
    }

    for i in &vec1 {
        println!("This is the second iteration: {}", i)
    }
}