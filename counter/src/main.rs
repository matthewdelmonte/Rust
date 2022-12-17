// counter program
fn main() {
    let mut counter = 0;

    for _i in 0..10 {
        counter += 1;

        if counter == 5 {
            println!("I'm {}!", counter);
            break
        }
        
        println!("{}", counter);
    }
}
