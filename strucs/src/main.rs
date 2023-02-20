// the Rectangle struct has two fields: height and width. 

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

// The impl block contains methods that can be called on an instance of the Rectangle struct.

// The new method is a constructor that creates a new Rectangle object and returns it. The other two methods, get_area and get_perimeter, calculate the area and perimeter of the rectangle, respectively.

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }

    fn height(&self) -> bool {
        self.height > 0
    }

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

// To create a new instance of the Rectangle struct, we call the new method using the syntax Rectangle::new(10, 5). This creates a new Rectangle object with a height of 10 and a width of 5.

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64
}

fn main() {
    let scale = 2;
    let rect = Rectangle::new(10, 5);
    let rect2 = Rectangle {
        height: dbg!(10 * scale),
        width: 5
    };

    if rect.width() && rect.height() {
        println!("Area: {}", rect.get_area());
    } else {
        println!("You entered a height of {} and a width of {}. Both values must be greater than zero to calculate the area of a rectangle.", rect.height, rect.width);
    }

    println!("Perimeter: {}", rect.get_perimeter());
    dbg!(&rect);

    let user1: User = User {
        email: String::from("matthew@twitter.com"),
        username: String::from("matthewd"),
        sign_in_count: 1,
        active: true
    };

    println!("user1: {:#?}", user1);

    let user2: User = build_user(
        String::from("caleb@twitter.com"),
        String::from("calebd"),
    );

    println!("user2 {:#?}", user2);

    let user3: User = User {
        email: String::from("fastmonkey@twitter.com"),
        username: String::from("calebd"),
        ..user1
    };

    println!("user3 {:#?}", user3);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}