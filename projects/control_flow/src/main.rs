fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // let number = 3;
    // if number {
    //     println!("number was three");
    // }

    let number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // let condition = true;
    // let number = if condition { 5 } else { "six" };  // error !!!
    // println!("The value of number is: {number}");

    let mut counter = 0;
    let result = loop {
        if counter == 10 { break counter * 2; } else { counter += 1 };
    };
    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is: {element}");
    }

    let fahrenheit = 212;
    let celsius = 0;
    println!(
        "{} degrees fahrenheit is {} degrees celsius.",
        fahrenheit, degf_to_degc(fahrenheit)
    );
    println!(
        "{} degrees celsius is {} degrees fahrenheit.",
        celsius, degc_to_degf(celsius)
    );

    let num = 20;
    println!("The Fibonacci number of {} is {}", num, calculate_fibonacci(num));

    print_twelve_days_of_christmas();
}

fn degf_to_degc(fahrenheit: i32) -> i32 {
    fahrenheit
}

fn degc_to_degf(celsius: i32) -> i32 {
    celsius
}

fn calculate_fibonacci(n: i32) -> i32 {
    if n < 2 {
        return n
    }
    calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2)
}

fn print_twelve_days_of_christmas() {
    println!("\nThe Twelve Days of Christmas");
    println!("----------------------------\n");

    for day in 1..13 {
        println!("{day}.");
        print_day(day);

        for gift_day in (1..(day + 1)).rev() {
            let prefix = if gift_day == 1 && day != 1 { "and " } else { "" };
            print_gift(gift_day, prefix);
        }
    }

    fn print_day(n: u32) {
        let day = match n {
            1  => "first",
            2  => "second",
            3  => "third",
            4  => "fourth",
            5  => "fifth",
            6  => "sixth",
            7  => "seventh",
            8  => "eighth",
            9  => "ninth",
            10 => "tenth",
            11 => "eleventh",
            12 => "twelfth",
            _  => "",
        };

        println!("On the {day} day of Christmas\nmy true love sent to me");
    }

    fn print_gift(n: u32, prefix: &str) {
        let gift = match n {
            1  => "a partridge in a pear tree.\n",
            2  => "two turtle doves,",
            3  => "three French hens,",
            4  => "Four calling birds,",
            5  => "Five golden rings,",
            6  => "six geese a-laying,",
            7  => "seven swans a-swimming,",
            8  => "eight maids a-milking,",
            9  => "nine ladies dancing,",
            10 => "ten lords a-leaping,",
            11 => "eleven pipers piping,",
            12 => "Twelve drummers drumming,",
            _  => "",
        };

        println!("{prefix}{gift}");
    }
}
