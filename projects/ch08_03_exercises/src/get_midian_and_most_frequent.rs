// 整数のリストが与えられたときにベクターを使って、
// リストの中央値(ソースされたときに中の位置にある値)と最頻値（最も頻繁に出現する値）を返す関数の作成

use std::collections::HashMap;

// 数値のリストから中央値を求めて返す
pub fn get_midian(vec: &Vec<i32>) -> i32 {
    if vec.is_empty() {
        return -1;
    }

    let mut vec = vec.clone();
    let mid = vec.len() / 2;

    vec.sort();

    match vec.len() % 2 {
        0 => -1,
        _ => vec[mid],
    }
}

// 数値のリストから最も頻出する値を返す
pub fn get_most_frequent(vec: &Vec<i32>) -> i32 {
    if vec.is_empty() {
        return -1;
    }

    let mut map = HashMap::new();

    // 以下の形式で挿入
    // [1, 2, 3, 4, 5, 4] -> {1: 1, 2: 1, 3: 1, 4: 2, 5: 1}
    for &number in vec {
        *map.entry(number).or_insert(0) += 1;
    }

    let values = map.values().collect::<Vec<&i32>>();
    if values.iter().all(|&value| value == values[0]) {
        return -1;
    }

    let mut max_value = 0;
    let mut max_frequency = 0;

    // mapを順番に読み取り、前の値よりも大きい値であれば、その都度更新していく
    for (&value, &frequency) in map.iter() {
        if frequency > max_frequency {
            max_frequency = frequency;
            max_value = value;
        }
    }
    max_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn midian() {
        assert_eq!(2, get_midian(&vec![1, 2, 3]));
        assert_eq!(3, get_midian(&vec![1, 2, 3, 4, 5]));
        assert_eq!(-1, get_midian(&vec![]));
        assert_eq!(-1, get_midian(&vec![1, 2, 3, 3]));
    }

    #[test]
    fn most_frequent() {
        assert_eq!(1, get_most_frequent(&vec![1, 2, 1]));
        assert_eq!(2, get_most_frequent(&vec![1, 2, 3, 2]));
        assert_eq!(-1, get_most_frequent(&vec![]));
        assert_eq!(-1, get_most_frequent(&vec![1, 1, 2, 2]));
    }
}
