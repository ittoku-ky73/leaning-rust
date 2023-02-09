mod get_midian_and_most_frequent;
mod pig_latin;
mod text_interface;

pub use crate::get_midian_and_most_frequent::{get_midian, get_most_frequent};
pub use crate::pig_latin::piglatinize;
pub use crate::text_interface::text_interface;

fn main() {
    midian_and_most_frequent();

    pig_latin();

    text_interface();
}

fn midian_and_most_frequent() {
    let vec = vec![1, 2, 3, 4, 3];
    let midian = get_midian(&vec);
    let most_frequent = get_most_frequent(&vec);

    println!("midian: {}", midian);
    println!("most_frequent: {}", most_frequent);
}

fn pig_latin() {
    println!("apple: {}", piglatinize("apple"));
    println!("bitch: {}", piglatinize("bitch"));
    println!("curl: {}", piglatinize("curl"));
    println!("first: {}", piglatinize("first"));
    println!("second: {}", piglatinize("second"));
}
