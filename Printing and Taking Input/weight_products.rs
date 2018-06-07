use stdin::io;

fn main() {
    println!("Enter number of widgets: ")
    let widget_num = String::new();
    io::stdin().read_line(&mut widget_num)
    .expect("Could not read line :(");

    let widget_num = widget_num.trim().parse(){
        Ok(num) => widget_num,
        Err(_) => println!("You did not enter a number!")
    }

    println!("Enter number of gizmos.")
    let gizmo_num = String::new();

    io::stdin().read_line(&mut gizmo_num)
    .expect("Could not read line :(");

    let gizmo_num = gizmo_num.trim().parse(){
        Ok(num) => gizmo_num,
        Err(_) => println!("You did not enter a number!")
    }

    let widget_weight = widget_num * 75;

    let gizmo_weight = gizmo_num * 112;

    let total = gizmo_weight + widget_weight;

    println!("Total weight is {} grams", total) 
}