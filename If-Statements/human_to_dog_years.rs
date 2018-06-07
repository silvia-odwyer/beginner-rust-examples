use stdin::io;

fn main() {
    let human_years = String::new();

    io::stdin().read_line(&mut human_years)
    .expect("Could not read line :(");

    let human_years : i32 = human_years.trim().parse(){
        Ok(num) => human_years,
        Err(_) => println!("Please enter a number!")
    }

    let dog_years = 0;
    if human_years < 3 && human_years > 0 {
        dog_years = human_years * 10.5
    }
    else {
        dog_years = 20.5 + ((human_years - 2) * 4)
    }

}