// ハッシュマップとベクターを使って、ユーザーが従業員名を会社の部署に追加できるようなテキストインターフェイスを作成。
// 例えば、"サリーを技術部に"や"アミールを営業部に"といった感じです。そしてユーザーに部門に属するすべての人、または部門別に会社に属するすべての人をアルファベット順に並べたリストを取得する

mod get_midian_and_most_frequent;
mod pig_latin;

pub use crate::get_midian_and_most_frequent::{
    get_midian,
    get_most_frequent
};
pub use crate::pig_latin::piglatinize;


fn main() {
    midian_and_most_frequent();

    pig_latin();
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
