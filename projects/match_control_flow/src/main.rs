#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
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

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let alabama_quarter = Coin::Quarter(UsState::Alabama);
    let alaska_quarter = Coin::Quarter(UsState::Alaska);
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Coin Penny is: {}", value_in_cents(penny));
    println!("Coin Nickel is: {}", value_in_cents(nickel));
    println!("Coin Dime is: {}", value_in_cents(dime));
    println!("Coin Quarter is: {}", value_in_cents(alabama_quarter));
    println!("Coin Quarter is: {}", value_in_cents(alaska_quarter));
    println!("six is: {:?}", six);
    println!("none is: {:?}", none);
}
