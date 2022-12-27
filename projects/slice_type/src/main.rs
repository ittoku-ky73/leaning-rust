fn main() {
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
