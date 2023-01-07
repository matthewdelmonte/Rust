fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

// practice creating a person struct
struct Person {
    age: u8,
    first_name: String,
    last_name: String,
    occupation: String,
}

fn main() {
    let number_list = vec![34, 28, 16, -110, 51];

    let result = largest(&number_list);
    println!("The largest number is: {}", result);

    let char_list = vec!['Y', 'B', 'Z', '4', 'N'];

    let result = largest(&char_list);
    println!("The largest character is: {}", result);
    
    let integer = Point { x: 1, y: 2 };
    let float = Point { x: 1.0, y: 2.0 };

    let person = Person {
        age: 24,
        first_name: String::from("Matthew"),
        last_name: String::from("Del Monte"),
        occupation: String::from("Software Developer"),
    };

    println!("Hi! My name is {} and I'm {} years old.", person.first_name, person.age);

}
