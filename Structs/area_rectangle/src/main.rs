struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
fn area(&self) -> u32 {
    self.width * self.height 
}

fn can_hold(&self, other : &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
}

}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50};

    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("The area of rectangle1 is {} square(**2) metres", rect1.area());
    println!("The area of rectangle2 is {} square metres", rect2.area());
    println!("Can rect3 fit inside rect1? {}", rect1.can_hold(&rect3));
}


