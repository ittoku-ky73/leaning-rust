// 文字列をビッグラタンに変換する関数の作成。
// 各単語の最初の子音を語尾に移動し、"ay"を追加。"first"は"irst-fay"となる。母音で始まる単語は、代わりに"hay"が末尾に追加される("apple"は"apple-hay")。
// UTF-8エンコーディングの詳細について覚えておくこと

// ハッシュマップとベクターを使って、ユーザーが従業員名を会社の部署に追加できるようなテキストインターフェイスを作成。
// 例えば、"サリーを技術部に"や"アミールを営業部に"といった感じです。そしてユーザーに部門に属するすべての人、または部門別に会社に属するすべての人をアルファベット順に並べたリストを取得する

mod get_midian_and_most_frequent;

pub use crate::get_midian_and_most_frequent::{
    get_midian,
    get_most_frequent
};

fn main() {
    let vec = vec![1, 2, 3, 4, 3];
    let midian = get_midian(&vec);
    let most_frequent = get_most_frequent(&vec);

    println!("midian: {}", midian);
    println!("most_frequent: {}", most_frequent);
}
