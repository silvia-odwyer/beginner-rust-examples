use std::io

fn main() {
    println!("Enter number of large containers: ");
    let num_large_containers = String::new();

    io::stdin().read_line(&num_large_containers)
    .expect("Could not read line :(");

    let num_large_containers : u32 = num_large_containers.trim().parse(){
        Ok(num) => num_large_containers,
        Err(_) => println!("Please enter a number!")
    }

    let num_small_containers = String::new();

    io::stdin().read_line(&num_small_containers)
    .expect("Could not read line :(");

    let num_small_containers : u32 = num_small_containers.trim().parse(){
        Ok(num) => num_small_containers,
        Err(_) => println!("You did not enter a number. :(")
    }

    let large_cont_deposit = num_large_containers * 0.25;

    let small_cont_deposit = num_small_containers * 0.10;
    
    let total = large_cont_deposit + small_cont_deposit;
    println!("Total deposit earned: {}", total);

}