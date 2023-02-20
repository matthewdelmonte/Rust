// fn main() {
//     // move s1 into s2
//     // let s1 = String::from("hello");
//     // let s2 = s1;

//     // clone s1 into s2
//     let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("s1 = {}, s2 = {}", s1, s2);
// }
// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{}' is {}.", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a String

//     (s, length)
// }

fn printer(value: String) -> String {
    return value
    }
    
fn main() {
    let x = String::from("value");
    println!("The result of printing x is that it prints a {}.", printer(x)); 

    // Trying to use the variable after ownership of its value has been transferred
    // println!("The Result of Printing y is that it also prints a {}.", x);
}