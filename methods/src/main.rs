#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32, 
}

impl Rectangle {
    fn area_of_rectangle(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

//Constructor to build square rectangle. "Actually in Rust constructor is what in which you do not pass self instance as parameter so it will considered as a parameter"
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle { 
        width: 30, 
        height: 50
    };
    
    let rect2 = Rectangle { 
        width: 10, 
        height: 10
    };

    let rect3 = Rectangle { 
        width: 40, 
        height: 51
    };
    
    println! (
        "The area of rectangle is {} square pixels", rect1.area_of_rectangle()
    );
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    //calling constructor
    let sq = Rectangle::square(3);
    
    println! (
        "Rectangle {:#?}", sq
    );
}











