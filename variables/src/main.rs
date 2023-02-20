// use std::fmt::{ self, Formatter };
use std::env;
use std::str::FromStr;

fn main() {
    // println!("variables functions");
    // // original variable declaration
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The new value of x is {}", x);
    // // shadow copy of original variable
    // let x = x + 1;
    // println!("The value of x is: {x}");

    // let arr = ["10", "20", "30", "40", "50"];

    // for element in arr.iter() {
    //     println!("The value is {}!", element);
    // };

    // let PI = 3.141592;

    // println!("Pi is roughly {:.2}", PI);

    // for number in 1..4 {
    //     println!("{}!", number);
    // };


    // let mut number: i32 = 3;

    // while number != 0 {
    //     println!("{}!", number);

    //     number -= 1;
    // }

    // println!("Lift off!!!");

    env::set_var("RUST_BACKTRACE", "1");

    let input = "1 2 3 4 G";
    let mut numbers: [i32; 5] = [0; 5];

    let mut index = 0;
    for value in input.split_whitespace() {
        numbers[index] = i32::from_str(value).expect("Failed to parse value");
        index += 1;
    }

    println!("{:?}", numbers);
}

