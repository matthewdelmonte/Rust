// use std::io::{self, Read};

// // Array
// fn main() {
//     println!("Hello, world!");

//     // Rust array with 10 elements intialized to 0
//     let mut a = [0; 10];

//     // Read characters from keyboard in a
//     for i in 0..10 {
//         a[i] = read!();
//     }

//     // print array
//     for i in 0..10 {
//         print!("{}", a[i]);
//     }
// }
use std::io::{self, Read};

fn main() {
    println!("Hello, world!");

    let mut a = [0; 10];

    for i in 0..10 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        a[i] = input.trim().parse().unwrap();
    }

    for i in 0..10 {
        print!("{}", a[i]);
    }
}
