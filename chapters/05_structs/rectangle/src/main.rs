struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!("The area of the rectangle is {} square pixels", area(&rect1));
    println!("The area of the rectangle is {} square pixels", rect1.area());

    if rect1.width() {
        println!("The rectangle has nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); 
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3)); 
    
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
