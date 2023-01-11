fn main() {
    {                     // s is not valid here, it's not yet declared
        let s = "hello";  // s is valid from this point forward
        println!("{s}");
    }                     // this scope is now over, and s is no loger valid

    let mut s = String::from("hello");
    s.push_str(", world!");  // push_str() appends a literal to a String
    println!("{s}");  // This will print `hello, world!`

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    {
        let x = 5;
        let y = x;
        println!("x = {}, y = {}", x, y);

        let s = String::from("hello");  // s comes into scope
        takes_ownership(s);             // s's value moves into the function
                                        // ... and so is no longer valid here
        let x = 5;                      // x comes into scope
        makes_copy(x);                  // x would move into the function
                                        // but i32 is Copy, so it's okay to still
                                        // use x afterward
        fn takes_ownership(some_string: String) {  // some_string comes into scope
            println!("{some_string}");
        }  // Here, some_string goes out of scope and `drop` is called. The backing
           // memory is freed.

        fn makes_copy(some_integer: i32) {  // some_integer comes into scope
            println!("{some_integer}");
        }  // Here, some_integer goes out of scope. Nothing special happens.

    }  // Here, x goes out of scope, then s. But because s's value was moved, nothing
       // special happens.

    {
        let _s1 = gives_ownership();         // gives_owner moves its return
                                             // value into s1
        let s2 = String::from("hello");      // s2 comes into scope
        let _s3 = takes_and_gives_back(s2);  // s2 is moved into
                                             // takes_and_gives_back, which also
                                             // moves its return value into s3
        fn gives_ownership() -> String {  // gives_ownership will move its
                                          // return value into the function
                                          // that calls it
            let some_string = String::from("yours");  // some_string comes into scope
            some_string                               // some_string is returned and
                                                      // moves out to the calling
                                                      // function
        }

        // This function takes a String and returns one
        fn takes_and_gives_back(a_string: String) -> String {  // a_string comes into
                                                               // scope
            a_string  // a_string is returned and moves out to the calling function
        }
    }  // Here, s3 goes out of scope and is dropped. s2 was moved, nothing
       // happens. s1 goes out of scope and is dropped.

    {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);

        println!("The length of '{}' is {}.", s2, len);

        fn calculate_length(s: String) -> (String, usize) {
            let length = s.len();

            (s, length)
        }
    }

    // References and Borrowing

    {
        let mut s = String::from("hello");
        change(&mut s);
        println!("{s}");

        fn change(some_string: &mut String) {
            some_string.push_str(", world");
        }
    }

    {
        let mut s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        println!("{r1} and {r2}");
        let r3 = &mut s;
        println!("{r3}");
    }

    {
        let reference_to_nothing = no_dangle();
        println!("{reference_to_nothing}");

        // fn dangle() -> &String {
        //     let s = String::from("hello");
        //     &s
        // }

        fn no_dangle() -> String {
            String::from("hello")
        }
    }

    // Slice Type

    let s = String::from("hello world");
    let word = first_word(&s);
    println!("{} first word index is: {}", &s, first_word(&s));
    // s.clear();  // error!
    println!("The value of s is: {word}");

    let my_string = String::from("hello world");

    let _word = first_word(&my_string[0..6]);
    let _word = first_word(&my_string[..]);
    let _word = first_word(&my_string);

    let my_string_literal = "hello world";

    let _word = first_word(&my_string_literal[0..6]);
    let _word = first_word(&my_string_literal[..]);
    let _word = first_word(&my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    &s[..]
}
