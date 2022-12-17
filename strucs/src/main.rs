// the Rectangle struct has two fields: height and width. The impl block contains methods that can be called on an instance of the Rectangle struct.

struct Rectangle {
    height: u32,
    width: u32,
}

// The new method is a constructor that creates a new Rectangle object and returns it. The other two methods, get_area and get_perimeter, calculate the area and perimeter of the rectangle, respectively.

// To create a new instance of the Rectangle struct, we call the new method using the syntax Rectangle::new(10, 5). This creates a new Rectangle object with a height of 10 and a width of 5.

impl Rectangle {
    fn new(height: u32, width: u32) -> Rectangle {
        Rectangle { height, width }
    }

    fn get_area(&self) -> u32 {
        self.height * self.width
    }

    fn get_perimeter(&self) -> u32 {
        2 * (self.height + self.width)
    }
}

fn main() {
    let rect = Rectangle::new(10, 5);
    println!("Area: {}", rect.get_area());
    println!("Perimeter: {}", rect.get_perimeter());
}

