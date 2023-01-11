#[derive(Debug)]
struct Ipv4Addr {
    _address: String,
}

impl Ipv4Addr {
    fn new(_address: String) -> Self {
        Self {
            _address
        }
    }
}

#[derive(Debug)]
struct Ipv6Addr {
    _address: String,
}

impl Ipv6Addr {
    fn new(_address: String) -> Self {
        Self {
            _address
        }
    }
}

#[derive(Debug)]
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { _x: i32, _y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message call: {:?}", self);
    }
}

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

fn dice_roll(dice_roll: u8) {
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

fn add_fancy_hat() { println!("Add fancy hat!!!") }
fn remove_fancy_hat() { println!("Remove fancy hat...") }

fn main() {
    // defining an Enum

    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(String::from("127.0.0.1")));
    let localhost_v6 = IpAddr::V6(Ipv6Addr::new(String::from("::1")));
    let m1 = Message::Quit;
    let m2 = Message::Move { _x: 12, _y: 34 };
    let m3 = Message::Write(String::from("hello"));
    let m4 = Message::ChangeColor(12, 34, 56);
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y;

    println!("v4 address is: {:?}", localhost_v4);
    println!("v6 address is: {:?}", localhost_v6);
    m1.call();
    m2.call();
    m3.call();
    m4.call();
    println!("some_number is: {:?}", some_number);
    println!("some_char is: {:?}", some_char);
    println!("absent_number is: {:?}", absent_number);

    // match control flow

    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let alabama_quarter = Coin::Quarter(UsState::Alabama);
    let alaska_quarter = Coin::Quarter(UsState::Alaska);
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    dice_roll(3);

    println!("Coin Penny is: {}", value_in_cents(penny));
    println!("Coin Nickel is: {}", value_in_cents(nickel));
    println!("Coin Dime is: {}", value_in_cents(dime));
    println!("Coin Quarter is: {}", value_in_cents(alabama_quarter));
    println!("Coin Quarter is: {}", value_in_cents(alaska_quarter));
    println!("six is: {:?}", six);
    println!("none is: {:?}", none);

    // if let

    let config_max = Some(3u8);
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);

    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } 

    match &coin {
        Coin::Quarter(state) => println!("State quarter from {:?}", state),
        _ => { 
            count += 1;
            println!("count is: {}", count);
        }
    }

    if let Coin::Quarter(state) = &coin {
        println!("State quarter form {:?}", state);
    } else {
        count += 1;
        println!("count is: {}", count);
    }

    if let Coin::Penny = &coin {
        println!("Coin penny!!!");
    } else {
        count += 1;
        println!("count is: {}", count);
    }
}
