// 文字列をビッグラタンに変換する関数の作成。
// 各単語の最初の子音を語尾に移動し、"ay"を追加。"first"は"irst-fay"となる。母音で始まる単語は、代わりに"hay"が末尾に追加される("apple"は"apple-hay")。
// UTF-8エンコーディングの詳細について覚えておくこと

use std::fmt::format;

pub fn piglatinize(s: &str) -> String {
    let mut s = String::from(s);
    let initial_letter = s.chars().next().unwrap();
    if !is_vowel(initial_letter) {
        s.remove(0);
        String::from(format!("{s}-{initial_letter}ay"))
    } else {
        String::from(format!("{s}-hay"))
    }
}

fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pig_latin() {
        assert_eq!(String::from("irst-fay"), piglatinize("first"));
        assert_eq!(String::from("apple-hay"), piglatinize("apple"));
        assert_eq!(String::from("ess-may"), piglatinize("mess"));
        assert_eq!(String::from("ather-fay"), piglatinize("father"));
        assert_eq!(String::from("itch-bay"), piglatinize("bitch"));
    }

    #[test]
    fn voel() {
        assert!(is_vowel('a'));
        assert!(!is_vowel('z'));
    }
}
