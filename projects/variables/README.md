# 変数

> 参考：https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

## 変数と可変性

Rustの変数はデフォルトで不変です。これはRustが安全性と簡単な同時実効性を行うためです。

変数が不変の場合、値が名前にバインドされると、その値を変更することはできません。

まず以下のコードを記述します。

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

そして、コンパイルすると以下のようなエラーを吐きます。

```txt
cargo build
   Compiling variables v0.1.0 (/Users/ittoku/Documents/learning-rust/projects/variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     println!("The value of x is: {x}");
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` due to previous error
```

コンパイルエラーは苛立たしいものですが、実際はプログラムがやりたいことを安全に実行できていないことを意味するだけです。これは「あなたが優れたプログラマーではない」という意味ではありません！

このエラーメッセージで見るべき箇所は、「`cannot assign twice to immutable variable`」です。そこで`x`変数を指しているので、そこがエラーの原因であることを示しています。

不変として指定された値を変更しようとすると、コンパイル時にエラーが発生することが重要です。これはまさにこの状況がバグにつながる可能性があるためです。Rustコンパイラは、値が変更されないと宣言した場合、実際には変更されないことを保証するため、自分で追跡する必要がなくなります。したがって、コードは簡単に推論できます。

しかし、可変性は非常に便利で、コードを書きやすくすることができます。`mut`を追加すると、変数の値を変更することができます。

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

プログラムを実行すると、次のようになります。

```shell
cargo run
   Compiling variables v0.1.0 (/Users/ittoku/Documents/learning-rust/projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/variables`
The value of x is: 5
The value of x is: 6
```

## 定数

不変変数と同様に、定数は名前にバインドされ変更できない値ですが、定数と変数にはいくつかの違いがあります。

まず、定数に`mut`を使うことができません。定数はデフォルトで不変というだけではなく、常に不変です。`const`キーワードの代わりに`let`キーワードを使用して定数を宣言し、値の型に注釈をつける必要があります。

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

定数に対するRustの命名規則は、全て大文字を使用し、単語間でアンダースコアを使用することです。コンパイラは、コンパイル時に限られた一連の操作を評価できます。

定数は、宣言されたスコープ内でプログラムが実行されている間ずっと有効です。

プログラム全体で定数として使用されるハードコーディングされた値に名前を付けると、その値の意味をコードの将来の管理者に伝えるのに役立ちます。また、ハードコーディングされた値を将来更新する必要が生じた場合に変更する必要があるコードないの場所を1つだけにしておくと役立ちます。

## シャドーイング

Rustでは変数と同じ名前の新しい変数を宣言することができます。Rustaceansは、最初の変数は2番目の変数によって隠されているといいます。つまり、2番目の変数は、変数の名前を使用した時にコンパイラーが認識するものです。実際には、2番目の変数は最初の変数を覆い隠し、それ自体が影にされるか、スコープが終了するまで、変数名の仕様をそれ自体に引き継いでいきます。

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

プログラムを実行すると以下のようになります。

```shell
cargo run
   Compiling variables v0.1.0 (/Users/ittoku/Documents/learning-rust/projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/variables`
The value of x in the inner scope is: 12
The value of x is: 6
```

シャドウイングは、`let`を使わずにこの変数に再代入しようとすると、コンパイル時エラーが発生します。`let`を使うことで、値に対して変換を行うことができ、その変換が完了した後にその変数をimmutableにすることができます。

`mut`とシャドウイングのもう1つの違いは、`let`を使う時に新しい変数を作ることになるので、値の型を変えても同じ名前を利用することができる点です。

```rust
let spaces = "   ";
let spaces = spaces.len();
```

しかし以下のコードで`mut`を使おうとすると、

```rust
let mut spaces = "   ";
spaces = spaces.len();
```

以下のエラーを吐きます。

```shell
cargo run
   Compiling variables v0.1.0 (/Users/ittoku/Documents/learning-rust/projects/variables)
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
2 |     let mut spaces = "   ";
  |                      ----- expected due to this value
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected `&str`, found `usize`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `variables` due to previous error
```

これは、「変数の型を変えてはいけませんよ」というエラーです。

次は、データ型について詳しくみていきます。
