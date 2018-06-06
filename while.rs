fn main() {
    let x = 0;

    // An infinite loop, it is cleaner and much more efficient
    // to use this looping mechanism if you know that you 
    // will need to loop infinitely.
    
    loop {
        x = x + 1;

    }

    let done = false;

    while !done {
        if x == 10 {
            println!("We have reached ten!");
            done = true;
        }
        else {
            continue;
        }
    }

    for x in 1..10 {
        println!("{}", x);
    }

    for (index, value) in (5..10).enumerate(){
        println!("index is {} and value is {}", index, value);
    }
}