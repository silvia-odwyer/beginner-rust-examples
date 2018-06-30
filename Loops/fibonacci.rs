use std::io;

fn main () {
    loop {
    let mut n = String::new();
    println!("Please enter a number: ");
    io::stdin().read_line(&mut n).expect("Please enter a number.");

    let n : u32 = match n.trim().parse(){
    Ok(n) => n,
    Err(_) => break,
    };

    let mut fib_vec = vec![];

    let mut x = 1;
    let mut prev_num = 2;
    let mut i = 0;

    loop {
        current_num = current_num + prev_num;
        fib_vec.push(x);
        prev_num = x;

        if i == n{
            println!("{:?}", fib_vec);
        }
    }


    }
    }

}