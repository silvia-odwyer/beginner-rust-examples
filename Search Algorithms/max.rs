use std::io;

fn main() {
    // get random numbers and add them to the vec.
    // for _i in 1..1000 {
    //     let num = 
    // }    
    let vec = vec![987, 732, 3247, 23472, 18023, 3213098, 13209123, 13089, 123, 302];
    let mut max = 0;
    let mut index = 0;

    for num in vec.iter() {

        if num > &max {
            max = *num;
        }
        index += 1;
        if index == vec.len() - 1 {
            println!("Max is: {}", max)
        }
    }
}
