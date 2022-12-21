# 関数

> 参考：https://doc.rust-lang.org/book/ch03-03-how-functions-work.html

Rustでは関数が一般的です。main関数は、多くのプログラムのエントリポイントです。また、`fn`キーワードで新しい関数を宣言することができます。

Rustは、関数名と変数名の従来のスタイルとしてスネークケースを使用します。このスタイルは、小文字で、単語をアンダースコアで区切ります。

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

このプログラムの出力は次のようになります。

```shell
cargo run
   Compiling functions v0.1.0 (/Users/ittoku/Documents/learning-rust/projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/functions`
Hello, world!
Another function.
```

### パラメーター

関数は、シグネチャの一部である特別な変数である*parameters*を持つことができます。関数にパラメーターがある場合、具体的な値を指定することができます。技術的には、具体的な値は引数と呼ばれますが、カジュアルな会話では、関数の定義内の変数や関数を呼び出す時に渡される具体的な値のいずれかに対して、パラメーターと引数ということがを使うこともあります。

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```

このプログラムの出力は次のようになります。

```shell
cargo run
   Compiling functions v0.1.0 (/Users/ittoku/Documents/learning-rust/projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/functions`
The value of x is: 5
```

関数シグネチャでは、各パラメーターの型を宣言する必要があります。これは、Rustの設計における意図的な決定です。関数定義で型注釈を要求することは、コンパイラがコードの他の場所で型注釈を使用して、意味する型を理解する必要がほとんどないことを意味します。コンパイラは、関数が期待する型を知っていれば、より役立つエラーメッセージを表示することもできます。

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

このプログラムを実行すると次のような出力になります。

```shell
cargo run
   Compiling functions v0.1.0 (/Users/ittoku/Documents/learning-rust/projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/functions`
The measurement is: 5h
```

### ステートメントと式

関数本体は、式で終わる一連のステートメントで構成されています。これまで説明してきた関数には修了式が含まれていませんでしたが、ステートメントの一部として式を見てきました。Rustは式ベースの言語であるため、これは理解すべき重要な違いです。

ステートメントは、何らかのアクションを実行し、値を返さない命令です。以下の例を見てみましょう。`let`キーワードを使用して変数を作成し、変数に値を割り当てることはステートメントです。

```rust
fn main() {
  let y = 6;
}
```

関数定義もステートメントです。前の例は、それ自体がステートメントです。ステートメントは値を返さないため、以下のコードのように別の変数に値を割り当てることはできません。

```rust
fn main() {
  let x = (let y = 6);
}
```

このプログラムを実行すると、次のようなエラーが表示されます。

```shell
cargo run
   Compiling functions v0.1.0 (/Users/ittoku/Documents/learning-rust/projects/functions)
error: expected expression, found `let` statement
 --> src/main.rs:7:14
  |
7 |     let y = (let y = 6);
  |              ^^^

error: expected expression, found statement (`let`)
 --> src/main.rs:7:14
  |
7 |     let y = (let y = 6);
  |              ^^^^^^^^^
  |
  = note: variable declaration using `let` is a statement

error[E0658]: `let` expressions in this position are unstable
 --> src/main.rs:7:14
  |
7 |     let y = (let y = 6);
  |              ^^^^^^^^^
  |
  = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information

warning: unnecessary parentheses around assigned value
 --> src/main.rs:7:13
  |
7 |     let y = (let y = 6);
  |             ^         ^
  |
  = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
  |
7 -     let y = (let y = 6);
7 +     let y = let y = 6;
  |

For more information about this error, try `rustc --explain E0658`.
warning: `functions` (bin "functions") generated 1 warning
error: could not compile `functions` due to 3 previous errors; 1 warning emitted
```

この`let y = 6`ステートメントは値を返さないため、`x`にバインドする値がないためエラーとなります。これは、割り当てが割り当ての値を返すC言語やRubyなどの他の言語とは異なります。

式はステートメントの一部にすることができます。以下の例では、スコープブロックを使用して、変数にバインドしています。

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

  println!("The value of y is: {y}");
}
```

`y`には4が与えられます。ここで注目すべきなのは、`x + 1`で行の最後にセミコロンがないことです。石の最後にセミコロンを追加すると、それをステートメントに変換してしまい、値が返されなくなります。つまり戻り値を設定したい場合、セミコロンを使用しないということです。

### 戻り値を持つ関数

関数は、呼び出したコードに値を返すことができます。戻り値には名前をつけませんが、矢印（`->`）の後にその型を宣言する必要があります。Rustでは、関数の戻り値に関数本体のブロックの最後の式の値を含めます。`return`キーワードも使えますが、ほとんどの場合、式を使います。

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```

上記のコードは正常に動作しますが、下記のコードはエラーになります。

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
  x + 1;
}
```

このプログラムは、次のようなエラーが発生します。

```shell
cargo run
   Compiling functions v0.1.0 (/Users/ittoku/Documents/learning-rust/projects/functions)
error[E0308]: mismatched types
  --> src/main.rs:32:24
   |
32 | fn plus_one(x: i32) -> i32 {
   |    --------            ^^^ expected `i32`, found `()`
   |    |
   |    implicitly returns `()` as its body has no tail or `return` expression
33 |     x + 1;
   |          - help: remove this semicolon

For more information about this error, try `rustc --explain E0308`.
error: could not compile `functions` due to previous error
```

メインのエラーメッセージである「mismatched types」は、このコードの核心的な問題を明らかにしています。`plus_one`関数の定義では、`i32`を返すことになっていますが、ステートメントは値として評価されず、単位型である`()`で表現されています。そのため、何も返されず、関数の定義と矛盾し、エラーとなります。この出力では、Rustがこの問題を修正するためのメッセージを提供しています。セミコロンを削除すれば、エラーが修正されることを示唆しています。