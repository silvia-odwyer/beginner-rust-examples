

fn main() {
    let limit = 10;
    let mut vec = vec![];

    for i in 0..limit {
        vec.push(i);
    }

    println!("{:?}", vec);

    let p = 2;
    while p < limit {
        let multiples = vec![];

        let index = 2 * p;
        while index < limit + 1 {
            multiples.push(index)
            index += p;
        }

        println!("Multiples {:?}", multiples);

        for num in 
    }
}
