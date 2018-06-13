use std::io;

fn main() {

    loop {
    println!("Enter human years: ");
    let mut human_years = String::new();

    io::stdin().read_line(&mut human_years)
    .expect("Could not read line :(");

    let human_years : i32 = match human_years.trim().parse(){
        Ok(human_years) => human_years,
        Err(_) => continue,
    };

    if human_years < 3 {
        let dog_years = human_years * 10;
            println!("Dog years: {}", dog_years)
    }
    else {
        let dog_years = 20 + ((human_years - 2) * 4);
            println!("Dog years: {}", dog_years)
    }



    }
  

}