fn main() {
    // move s1 into s2
    // let s1 = String::from("hello");
    // let s2 = s1;

    // clone s1 into s2
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
