#[derive(Debug)]
enum UsState {
    Alabama,
    // Alaska,
    // --snip--
}

enum Coin {
    _Penny,
    Quarter(UsState),
}

fn main() {
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

    if let Coin::_Penny = &coin {
        println!("Coin penny!!!");
    } else {
        count += 1;
        println!("count is: {}", count);
    }
}
