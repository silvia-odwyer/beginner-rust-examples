use std::io;

fn main(){
    
    println!("This program prints until a certain number!");

    let end = String::new();
    let num = 0;
    io::stdin().read_line(&mut end)
    .expect("Could not read line! :/");

    let end : u32 = end.trim().parse(){
        Ok(num) => end, 
        Err(_) => continue,
    };

    println!("You entered {}", num);

    loop {
        let res = inc_num(num);
        match res.cmp(&end){
            Ordering::Less => println!("{}", res),
            Ordering::Greater => break;
        }
    } 

}

fn inc_num(i32: num) => i32{
    num + 1
}