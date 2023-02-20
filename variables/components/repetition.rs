use std::env;
use std::str::FromStr;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let input = "1 2 3 4 5";
    let mut numbers: [i32; 5] = [0; 5];

    let mut index = 0;
    for value in input.split_whitespace() {
        numbers[index] = i32::from_str(value).expect("Failed to parse value");
        index += 1;
    }

    println!("{:?}", numbers);
}