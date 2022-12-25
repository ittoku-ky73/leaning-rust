# 参照と借用

> 参考：https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

前の章で書いたタプルコードです。

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

このタプルコードの問題は、`s`の所有権が`calculate_length`関数に移動されたため、もう一度変数を定義しなければならない点です。これは参照を使用することで解決できます。参照はポインタと同じで、そのアドレスに格納されているデータにアクセスするために追跡することができます。ただポインタとは異なり、その参照が存在する限り、型の有効な値を指すことが保証されます。

```rust
{
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
  s.len()
}
```

上記のコードは、`s1`に所有権を持たせたまま関数に値を渡しています。`&`は参照を表し、所有権を持たずに参照することができます。

![trpl04-05](https://doc.rust-lang.org/book/img/trpl04-05.svg)

> 注意：`&`の反対は非参照で、参照解除演算子`*`です。第8章でこれの使い方を、第15章で参照解除の詳細について説明します。

以下のコードの部分では、`s1`の動きを追っていきます。まず`s1`変数を生成し、その変数の値のみを`calculate_length`関数の引数として渡しています。つまり、`s1`は値のみを渡して所有権は渡していないので、その後もこの変数を使用することができます。

```rust
let s1 = String::from("hello");

let len = calculate_length(&s1);
```

以下のコードの部分では、関数の引数に参照を渡す方法を書いています。注目すべき箇所は、`&String`ですね。型の前に`&`をつける必要があります。

```rust
fn calculate_length(s: &String) -> usize {  // s is reference to a String
  s.len()
}  // Here, s goes out of scope. But because it does not have ownership of what
   // it refers to, it is not dropped
```

Rustでは、参照を作る行為を借用（Borrowing）と呼んでいます。実生活と同じように、人が何かを所有している場合、その人から何かを借りることができます。使い終わったら、自分のものではないので返さなければいけません。

では、借りているものを修正しようとするとどうなるでしょうか。

```rust
{
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world!");
}
```

コードをコンパイルすると、以下のようなエラーを吐きます。変数が不変であるのと同様に参照も不変です。

```shell
cargo run
   Compiling references_and_borrowing v0.1.0 (/learning-rust/projects/references_and_borrowing)
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src/main.rs:7:13
  |
6 |         fn change(some_string: &String) {
  |                                ------- help: consider changing this to be a mutable reference: `&mut String`
7 |             some_string.push_str(", world");
  |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable

For more information about this error, try `rustc --explain E0596`.
error: could not compile `references_and_borrowing` due to previous error
```

解決方法は、`mut`を加えてミュータブル参照を作成することです。ただし、ミュータブル参照を複数作ることはできません。

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```

以下のエラーを吐きます。

```rust
cargo run
   Compiling references_and_borrowing v0.1.0 (learning-rust/projects/references_and_borrowing)
error[E0499]: cannot borrow `s` as mutable more than once at a time
  --> src/main.rs:14:18
   |
13 |         let r1 = &mut s;
   |                  ------ first mutable borrow occurs here
14 |         let r2 = &mut s;
   |                  ^^^^^^ second mutable borrow occurs here
15 |         println!("{r1}, {r2}");
   |                    -- first borrow later used here

For more information about this error, try `rustc --explain E0499`.
error: could not compile `references_and_borrowing` due to previous error
```

エラーでは、`s`を`mutable`として一度に複数回借りることができないので、このコードは無効ですと言っています。

同じデータに対して同時に複数の変異可能な参照ができないようにすることで、変異は可能ですが、非常に制御された方法で行われます。他の多くの言語では、好きなようにできるので、Rustでは苦労するところです。しかしこの制限を設けることで、コンパイル時にデータ競合を防ぐことができます。データ競合は、レースコンディションと似ていて、これら3つの動作が発生した時に起こります。

- 2つ以上のポインタが同時に同じデータにアクセスしている
- 1つ以上のポインタがデータへの書き込みに使用されている
- データへのアクセスを同期させるためのメカニズムがない

データ競合は未定義の動作を引き起こし、実行時にそれを追跡しようとすると、診断や修正が困難になります。Rustはデータ競合を含むコードのコンパイルを拒否することで、この問題を防いでいます。

```rust
let mut s = String::from("hello");
{
    let r1 = &mut s;
}  // r1 goes out of scope here, so we can make a new reference with no problems.
let r2 = &mut s;
```

上記のコードは動きますが、下記のコードは動きません。

```rust
let mut s = String::from("hello");
let r1 = &s;  // no problem
let r2 = &s;  // no problem
let r3 = &mut s;  // BIG problem
println!("{}, {}, and {}", r1, r2, r3);
```

エラーは次のとおりです。

```shell
cargo run
   Compiling references_and_borrowing v0.1.0 (learning-rust/projects/references_and_borrowing)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/main.rs:15:18
   |
13 |         let r1 = &s;
   |                  -- immutable borrow occurs here
14 |         let r2 = &s;
15 |         let r3 = &mut s;
   |                  ^^^^^^ mutable borrow occurs here
16 |         println!("{r1}, {r2}, and {r3}");
   |                    -- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `references_and_borrowing` due to previous error
```

同じ値への不変の参照を持ちながら、変更可能な参照を持つことはできません。

不変参照を使用するユーザは、値が突然変更されることは期待していません。しかし、データを読んでいるだけの人は、他の人のデータをいじることはできないので、複数の不変参照を持つことが可能です。

参照のスコープは、それが導入されたところから始まり、その参照が最後に使われた時まで続きます。

```rust
let mut s = String::from("hello");
let r1 = &s;  // no problem
let r2 = &s;  // no problem
println!("{r1} and {r2}");
let r3 = &mut s;  // no problem
println!("{r3}");
```

不変変数`r1, r2`のスコープは、それらが最後に使われた`println!`の後で終わります。スコープの外に出たことにより`r3`が使えるということです。

`Borrowing Error`は面倒ですが、Rustコンパイラが潜在的なバグを早期に指摘し、問題のある箇所を正確に示してくれます。そのおかげで、データが思った通りにいかないときに原因を突き止める必要がなくなります。

## ダングリングポインタ

ポインタを使用する言語では、誤ってダングリングポインタ（他の誰かに渡されたかもしれないメモリ上の位置を参照するポインタ）を作成してしまうことがよくあります。あるデータへの参照がある場合、コンパイラはそのデータへの参照より先にそのデータがスコープの外に行かないようにします。

```rust
let reference_to_nothing = dangle();

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

このコードを実行すると、以下のエラーを吐きます。

```shell
cargo run
   Compiling references_and_borrowing v0.1.0 (learning-rust/projects/references_and_borrowing)
error[E0106]: missing lifetime specifier
  --> src/main.rs:23:24
   |
23 |         fn dangle() -> &String {
   |                        ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
23 |         fn dangle() -> &'static String {
   |                         +++++++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `references_and_borrowing` due to previous error
```

このエラーは、ライフタイムについて言及しています。ライフタイムは第10章で説明します。このエラーの見るべきところは次の箇所です。

```shell
= help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
```

`s`は`dangle`の内部で生成されているので、`dangle`が終了すると、`s`は`deallocated`されます。そこで関数に参照を返そうとしたので、この参照は無効な`String`を指すことになるので、Rustがそれを止めているというわけです。

ここは単純にこう書きましょう。これで所有権が移動され、何も割り当て解除されません。

```rust
fn no_dangle() -> String {
  String::from("hello");
}
```

## 参照のルール

参照について、これまで見てきたことのおさらいをしておきましょう。

- 変更可能な参照は1つ、不変な参照は幾つでも持つことができる
- 参照は常に有効でなければならない

次は、異なる種類の参照である分割について見ていきます。
