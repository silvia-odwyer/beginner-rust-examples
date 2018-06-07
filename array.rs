fn main(){
    let array = [1, 2, 3, 4, 5];
    let array_len = array.len();
    let item: u32 = 0;
    for x in 0..array_len{
        item = array[x];
        println!(item);
    }
}