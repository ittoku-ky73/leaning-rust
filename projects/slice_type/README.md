# スライス

> 参考：https://doc.rust-lang.org/book/ch04-03-slices.html

スライスを使うと、コレクションの中の一部の要素を参照することができます。スライスは参照の一種であり、所有権を持ちません。

スライスを学ぶために、空白で区切られた単語の文字列を受け取り、その文字列の中で最初に見つかった単語を返す関数を書いていきます。もし文字列の中にスペースがなければ、文字列全体を返します。

以下の`first_word`関数は、パラメータに`&String`を指定しています。所有権は使わないのでこれで良いのです。戻り値には、文字列のスペースがあった地点のインデックスを返しています。

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
```

Stringを要素ごとに調べて、値がスペースかどうかチェックする必要があるため、`as_byte`メソッドを使用して`String`をバイトの配列に変換しています。

次に、バイト配列に対するイテレータを`iter`メソッドで作成します。イテレータについては、第13章で詳しく説明します。今のところ`iter`はコレクションの各要素を返すメソッドであり、`enumerate`は`iter`の結果をラップして、書く要素をタプルの一部として返すということだけ覚えておいてください。

`enumerate`から返されるタプルの最初の要素はインデックスで、2番目の要素はその要素の参照です。これは、インデックスを自分で計算する必要がなくなるので便利です。

`enumerate`メソッドはタプルを返すので、パターンを使ってそのタプルを再構築することができます。パターンは第6章で説明します。`for`ループでは、タプルのインデックスを`i`、タプルの1バイトを`&item`とするパターンを指定しています。`.iter().enumerate()`から要素への参照を取得しているので、パターンには`&`を使用しています。

`for`ループ内では、バイトリテラル構文を使用して、スペースを表すバイトを探しています。スペースが見つかったら、その位置を返します。スペースがなければ、文字列の長さを返します。

これで、文字列の最初の単語の終わりのインデックスを見つけることができましたが、問題があります。ただ`usize`を単体で返しているだけなので、`&String`の中でのみ意味のある数値になっていることです。つまり、文字列とは別の値なので、将来的に有効である保証がありません。以下のコードを見て見ましょう。

```rust
let mut s = String::from("hello");

let word = first_word(&s);

s.clear();
```

このプログラムは正常にコンパイルされ、`s.clear()`を読んだ後に`word`を使っても同じようにコンパイルされます。`word, s`の関係は全くないので、`s`を書き換えても`word`の値はそのまま残ります。

`word`のインデックスが`s`のデータと同期していないことを機にするのは面倒で、エラーも起こりやすいです。もし`second_word`関数を作るとなれば、インデックスの管理はさらに脆くなります。またシグネチャは次のようにしなければなりません。

```rust
fn second_word(s: &String) -> (usize, usize)
```

今の時点では、変数同士は無関係で浮遊しており、これらを同期させる必要があるのです。幸運なことにRustには文字列スライスを使うことでこの問題を解決することができます。

## 文字列スライス

文字列スライスは、Stringの一部を参照するものです。

```rust
let s = String::from("hello");

let hello = &s[0..5];
let world = &s[6..11];
```

`hello`はString全体への参照ではなく、`[0..5]`ビットで指定されたStringの一部の参照です。`start_index`はスライスの最初の位置で、`end_index`はスライスの最後の位置より1つ多い位置です。内部的には、スライスデータ構造には開始位置とスライスの長さが格納されています。つまり、`&s[6..11]`だと、`s`のインデックス`6`のバイトへのポインタを含むスライスとなり、長さは`5`になります。

![trpl04-06](https://doc.rust-lang.org/book/img/trpl04-06.svg)

Rustの、`... range`構文では、インデックスを`0`から始める場合、インデックスを文字列の最後で終わらせる場合、文字列全体のスライスを取得する場合、次のように書くことができます。

```rust
let slice = &s[..2];
let slice = &s[3..];
let slice = &s[..];
```

> 文字列スライス範囲のインデックスは、有効なUTF-8文字の教会で発生する必要があります。マルチバイト文字の途中で文字列スライスを作成しようとするとプログラムはエラーで終了します。UTF-8の扱いについては第8章で説明します。

これらを踏まえて、`first_word`にスライスを返すように書き換えて見ましょう。

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    &s[..]
}
```

ここで`first_word`を呼び出すと、基礎データに結びついた1つの値が返されます。この値は、スライスの開始点への参照と、スライス内の要素数から構成されています。

`second_word`関数でも有効です。

```rust
fn second_word(s: &String) -> &str
```

コンパイラは文字列への参照が有効であることを保証するので、混乱しにくい簡単なAPIができました。1つ前の`first_word`関数では、文字列とインデックスと別々の変数に入っており、文字列がクリアされて、インデックスはそのままでしたね。このコードは論理的に正しくありませんが、すぐにエラーにはなりません。エラーはそれらの変数を使った時に発生します。スライスはこのバグを検知し、コードに問題があることを知らせてくれます。以下の例で、コンパイル時エラーが発生します。

```rust
let mut s = String::from("hello world");

let word = first_word(&s);

s.clear();  // error!

println!("The first word is: {}", word);
```

エラーは次のとおりです。

```shell
cargo run
   Compiling slice_type v0.1.0 (learning-rust/projects/slice_type)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:5:5
  |
4 |     let word = first_word(&s);
  |                           -- immutable borrow occurs here
5 |     s.clear();
  |     ^^^^^^^^^ mutable borrow occurs here
6 |     println!("The value of s is: {word}");
  |                                   ---- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `slice_type` due to previous error
```

借用規則から、もし不変の参照がある場合、不変の参照を取ることはできなくなっています。`clear`はStringを変更させるものなので、`mutable`な参照を取得する必要があります。Rustは、`clear`の`mutable`参照と、`word`の`immutable`参照が同時に存在することを認めず、コンパイルに失敗します。RustはAPIを使いやすくしただけでなく、コンパイル時のエラーも一掃してくれました。

### 文字列リテラルのスライス

文字列リテラルはバイナリ内部（スタック）に格納されます。なので型は`&str`であり、バイナリのその特定のポイントを指すスライスである。これは文字列リテラルが不変である理由でもあり、`&str`は普遍の参照である。

```rust
let s = "hello world";
```

### 文字列スライスのパラメータ

リテラルとString値のスライスを取ることができるなら、`first_word`を改良するとなるとシグネチャになります。

経験豊富なRustaceanなら、下記のような署名を代わりに書くでしょう。こうすれば、`&String, &str`の両方で同じ関数を使用できるからです。

```rust
fn first_word(s: &str) -> &str
```

Stringがある場合は、StringのスライスかStringの参照を渡すことができます。これについては第15章で説明します。Stringの参照ではなく文字列のスライスを受け取る関数を定義することで、機能を失うことなくAPIをより一般的で便利にすることができます。

```rust
let my_string = String::from("hello world");

let word = first_word(&my_string[0..6]);
let word = first_word(&my_string[..]);
let word = first_word(&my_string);

let my_string_literal = "hello world";

let word = first_word(&my_string_literal[0..6]);
let word = first_word(&my_string_literal[..]);
let word = first_word(&my_string_literal);
```

### その他のスライス

文字列スライスは、文字列に特化したものですが、もっと一般的なスライスタイプもあります。配列を見ていきましょう。文字列の時と同じように、配列も参照することができます。

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

このスライスは、`&[i32]`という型を持っています。これは文字列のスライスと同じように動作し、最初の要素への参照と長さを格納します。この種のスライスは、他のあらゆる種類のコレクションに使用されます。これらのコレクションについては、第8章で説明します。

## 概要

`ownership, borrowing, slices`のコンセプトは、Rustプログラムのコンパイル時のメモリ安全性を確保します。Rust言語では、他のシステムプログラミング言語と同様にメモリの使用量を制御できますが、データの所有者がスコープの外に出た時に自動的にクリーンアップするため、余計なコードを書いたり、デバッグしたりする必要がありません。

`ownership`はRustの他の多くの部分の動作に影響を与えるので、この本の残りの部分を通して、この概念についてさらに説明していきます。第5章では、`struct`でデータの断片をグループ化することについて見ていきます。

