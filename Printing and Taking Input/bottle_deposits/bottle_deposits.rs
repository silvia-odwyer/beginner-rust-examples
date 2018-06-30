use std::io;

fn main() {

    loop {
        println!("Enter number of large containers: ");
        let mut num_large_containers = String::new();

        io::stdin().read_line(&mut num_large_containers)
        .expect("Could not read line :(");

        let num_large_containers : f64 = match num_large_containers.trim().parse(){
            Ok(num_large_containers) => num_large_containers,
            Err(_) => continue,
        };

        println!("Enter number of small containers: ");
        let mut num_small_containers = String::new();

        io::stdin().read_line(&mut num_small_containers)
        .expect("Could not read line :(");

        let num_small_containers : f64 = match num_small_containers.trim().parse(){
            Ok(num_small_containers) => num_small_containers,
            Err(_) => continue,
        };

        let large_cont_deposit = num_large_containers * 0.25;

        let small_cont_deposit = num_small_containers * 0.10;
        
        let total = large_cont_deposit + small_cont_deposit;
        println!("Total deposit earned: {}", total);
    }
   

}