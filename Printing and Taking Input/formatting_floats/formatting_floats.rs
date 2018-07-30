fn main() {
    // Notice that we use two floats in the division to get a 
    // float for our approximation of pi.
    let pi: f32 = 22.0/7.0;
    
    // To format a float we can specify a number of decimal places
    // using a precision format specifier in our string.
    // e.g, {:.3} => 3 decimal places
    // Notice how rust does round the number instead of just truncating.
    // However, there are limitations to this rounding, because of the 
    // fact that variables are represented in binary. Sometimes, you
    // may see something unexpected, such as 1.015 rounding to 1.01, etc.
    println!("{:.0}", pi); // 3
    println!("{:.1}", pi); // 3.1
    println!("{:.2}", pi); // 3.14
    println!("{:.3}", pi); // 3.143
    println!("{:.4}", pi); // 3.1429
    println!("{:.5}", pi); // 3.14286
    println!("{:.7}", pi); // 3.1428571
    println!("{:.9}", pi); // 3.142857075
    println!("{:.10}", pi);// 3.1428570747


}
