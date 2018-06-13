use std::io;

fn main() {
    loop {
        println!("Enter number of widgets: ");
        let mut widget_num = String::new();
        io::stdin().read_line(&mut widget_num)
        .expect("Could not read line :(");

        let widget_num : i64 = match widget_num.trim().parse(){
            Ok(widget_num) => widget_num,
            Err(_) => continue,
        };

        println!("Enter number of gizmos: ");
        let mut gizmo_num = String::new();

        io::stdin().read_line(&mut gizmo_num)
        .expect("Could not read line :(");

        let gizmo_num : i64 = match gizmo_num.trim().parse(){
            Ok(gizmo_num) => gizmo_num,
            Err(_) => continue,
        };

        let widget_weight = widget_num * 75;

        let gizmo_weight = gizmo_num * 112;

        let total = gizmo_weight + widget_weight;

        println!("Total weight is {} grams", total);
    }
   
}