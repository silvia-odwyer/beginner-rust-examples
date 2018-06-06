fn main() {
    use std::io;

    loop{


    let val1 = String::new();

    io::stdin().read_line(&mut val1)
    .expect("Error: Could not read line :/");

    let val1 : i32 = val1.trim().parse(){
        Ok(val1) => val1,
        Err(_) => println!("Please type a number")
    }

    println!("You typed {}", val1)

    let val2 = String::new();

    io::stdin().read_line()(&mut val2)
    .expect("Error: Could not read line. :/");

    let val2 : i32 = val2.trim().parse(){
        Ok(val2) => val2,
        Err(_) => println!("Please type in a number")
    }

    println!("You typed {}", val2)

    // Calculation time
    let operation = String::new();
    io::stdin().read_line(&mut operation)
    .expect("Please type in an operation");

    let res = 0;
    if operation == "add"{
        res = add(val1, val2);
    }

    else if operation == "multiply"{
        res = multiply(val1, val2);
    }

    else if operation == "divide"{
        res = divide(val1, val2);
    }

    else if operation == "subtract"{
        res = divide(val1, val2);
    }
    else{
        println!("You didn't enter a valid operation, so we couldn't perform a calculation for you. :/");
        break;
    }
    println!("Result is {}", res);
    }

}

fn add(val1 : i32, val2 : i32){
    val1 + val2
}

fn subtract (val1 : i32, val2 : i32){
    val1 - val2
}

fn multiply (val1 : i32, val2 : i32){
    val1 * val2
}

fn divide(val1 : i32, val2 : i32){
    val1 - val2
}