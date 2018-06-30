use std::io;

fn main() {
    loop {
        let mut n = String::new();

        println!("Enter a number: {}", n);

        io::stdin().read_line(&mut n).expect("Could not read line");
        
        let n = n.trim();

        let mut split_n: Vec<&str> = n.split("").collect();
        println!("{:?}", split_n);

        let mut vec = Vec::new();

        for num in &split_n {
            println!("Hi");
            let num : f64 = match num.trim().parse() {
                 Ok(num) => num,
                 Err(_) => continue,
            };
            vec.push(num);

        }
        println!("{:?}", vec);

        let mut sum = 0.0;
        for index in 0..vec.len() - 1 {
            let num = vec[index];
            
            sum = sum + num;
            println!("Num: {}\nSum: {}", num, sum);
            if index == vec.len() - 2 {
                println!("Total: {}", sum + vec[index + 1]);
            }
        }

    }
}