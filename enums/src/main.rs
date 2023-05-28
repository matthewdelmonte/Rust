#[derive(Debug)]
enum States {
    Connecticut,
    Florida,
    Georgia,
    Nevada,
    NewYork,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(States),
}

enum Bill {
    One,
    Five,
    Ten,
    Twenty,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn value_in_dollars(bill: Bill) -> u8{
    match bill {
        Bill::One => 1,
        Bill::Five => 5,
        Bill::Ten => 10,
        Bill::Twenty => 20
    }
}

fn main() {
    // value_in_cents(Coin::Quarter(UsState::Connecticut));
    println!("Coins value = {} cents.", value_in_cents(Coin::Quarter(States::Nevada)));
    println!("Bills value = {} dollars.", value_in_dollars(Bill::Twenty));
}
