use std::io;

fn main() {

    let mut pi : f64 = 3.0;
    let mut denom_one : f64 = 0.0;
    let mut denom_two : f64 = 1.0;
    let mut denom_three : f64 = 2.0;
    for i in 1..150 {
        denom_one = denom_one + 2.0;
        denom_two = denom_two + 2.0;
        denom_three = denom_three + 2.0;
        let mut div_res = (4.0 / (denom_one * denom_two * denom_three));

        let div = i % 2;
        pi = match div {
            0 => pi - div_res,
            _ => pi + div_res,
        };
        println!("{}", pi);
    }
}

// fn add_them(&pi : &f64, &denom_one : &f64, &denom_two : &f64, &denom_three : &f64) {
    
//         println!("1: {}, 2: {}, 3: {}", denom_one, denom_two, denom_three);
//         let mut div_res = (4.0 / (denom_one * denom_two * denom_three));
//         println!("DIV RES: {}", div_res);
//         pi = pi + (4.0 / (denom_one * denom_two * denom_three));
//         println!("{}", pi);
// }

// fn sub_them(&pi : &f64, &denom_one : &f64, &denom_two : &f64, &denom_three : &f64){
//         println!("1: {}, 2: {}, 3: {}", denom_one, denom_two, denom_three);
//         let mut div_res = (4.0 / (denom_one * denom_two * denom_three));
//         println!("DIV RES: {}", div_res);
//         pi = pi + (4.0 / (denom_one * denom_two * denom_three));
//         println!("{}", pi);
// }