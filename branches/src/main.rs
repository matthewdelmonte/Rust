// fn main() {
//     let value = 4;

//     match value {
//         5 => println!("the number is 5"),
//         _ => println!("the number is something else")
//     }
// }

fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}


