// use std::fmt::Display;
// &str is a reference to an immutable string and is stored in the binary
// String is an owned string that is mutable and stored on the heap

// fn main() {
//     let _s1: &str = "Hello, world!";
//     let _s2: String = String::from("Hello, Matthew");
//     let _s3: String = "Hello, dev".to_string();
//     let _s4: String = "Hello, Engineeer".to_owned();
//     let s5: &str = &_s3[..];
//     println!("{}", s5);
// }

// fn main() {
//     let mut s: String = String::from("foo");
//     s.push_str("bar");
//     println!("{}", s);

//     s.replace_range(.., "cadence");
//     println!("{}", s);
// }

fn main() {
    // let b: String = ["first ", "second"].concat();
    // let s1: String = String::from("tic");
    // let s2: String = String::from("tac");
    // let s3: String = String::from("toe");

    // let s: String = format!("{}-{}-{}", s1, s2, s3);

    // println!("{}", s);
    // println!("This is the value of b: {}", b);

    // let v: Vec<&str> = vec![&s1, &s2, &s3];
    // let concatenated: String = v.join("-");
    // println!("This is the value of v: {}", concatenated);

    // for c in "Matthew".bytes() {
    //     println!("{}", c);
    // };

    for c in "Matthew".chars() {
        println!("{}", c);
    };
}
