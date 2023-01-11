# 制御フロー

> 参考：https://doc.rust-lang.org/book/ch03-05-control-flow.html

条件が`true`であるかどうかに応じてコードを実行したり、条件が`true`である間コードを繰り返し実行したりする機能は、ほとんどのプログラミング言語の基本的な構成要素です。Rustコードの実行フローを制御できる最も一般的な構造は、`if`式とループです。

## `if`式

`if`式を使用すると、条件に応じてコードを分岐できます。条件を指定して、「この条件が満たされた場合、このコードブロックを実行します。この条件が満たされない場合は、このコードブロックを実行しません。」などができます。

式を調べるために、プロジェクトディレクトリに`control_flow`という名前の新しいプロジェクトを作成します。

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

全ての`if`式は`if`キーワードで始まり、その後に条件が続きます。上記の場合、`number`が5未満かどうかをチェックします。条件がtrueの場合に実行するコードブロックを、条件の直後の括弧の中に配置します。コードブロックはしばしばアームと呼ばれることもあります。

以下のコードは、条件が`bool`値ではないためエラーが発生します。

```rust
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```

エラーは次のとおりです。

```shell
cargo run
   Compiling control_flow v0.1.0 (/Users/ittoku/Documents/learning-rust/projects/control_flow)
error[E0308]: mismatched types
  --> src/main.rs:10:8
   |
10 |     if number {
   |        ^^^^^^ expected `bool`, found integer

For more information about this error, try `rustc --explain E0308`.
error: could not compile `control_flow` due to previous error
```

Rustは、RubyやJavaScriptとは異なり、非ブール型をブール型に自動的に変換しようとしません。`if`式では常に条件としてブール値を指定する必要があります。たとえば数値が`0`と等しくない場合にコードブロックを実行する場合は次のように書きます。

```rust
if number != 0 {
    println!("number was something oter than zero");
}
```

### `else if`で複数の条件を処理する

`if, else`を組み合わせることで複数の条件を使用できます。

```rust
let number = 6;

if number % 4 == 0 {
    println!("number is divisible by 4");
} else if number % 3 == 0 {
    println!("number is divisible by 3");
} else if number % 2 == 0 {
    println!("number is divisible by 2");
} else {
    println!("number is not divisible by 4, 3, or 2");
}
```

このプログラムの出力は次のようになります。

```shell
cargo run
   Compiling control_flow v0.1.0 (/Users/ittoku/Documents/learning-rust/projects/control_flow)
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/control_flow`
number is divisible by 3
```

このプログラムを実行すると、各`if`式が順番にチェックされ、条件がtrueとなる最初の本体が実行されます。つまり、`number % 3 == 0`の条件でtrueになった時点で、それより下の`if`式は実行されないということです。

`else if`式が多すぎるコードは乱雑になる可能性があるため、式が複数ある場合は、コードをリファクタリングすることをお勧めします。第6章では、Rustの`match`分岐構造について説明します。

### `if`での`let`ステートメントの使用

`if`は式であるため、`let`で代入することができます。

```rust
let condition = true;
let number = if condition { 5 } else { 6 };
println!("the value of number is: {number}");
```

このプログラムの出力は次のようになります。

```shell
cargo run
   Compiling control_flow v0.1.0 (/Users/ittoku/Documents/learning-rust/projects/control_flow)
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/control_flow`
The value of number is: 5
```

`if`式全体の値は、実行するコードブロックによって異なります。つまり`if`式のアームの結果が同じ型でなければならないということです。以下の例では、型が一致しないためエラーとなります。

```rust
let condition = true;
let number = if condition { 5 } else { "six" };
println!("the value of number is: {number}");
```

このプログラムを実行しようとすると、エラーになります。

```shell
cargo run
   Compiling control_flow v0.1.0 (/Users/ittoku/Documents/learning-rust/projects/control_flow)
error[E0308]: `if` and `else` have incompatible types
  --> src/main.rs:31:44
   |
31 |     let number = if condition { 5 } else { "six" };
   |                                 -          ^^^^^ expected integer, found `&str`
   |                                 |
   |                                 expected because of this

For more information about this error, try `rustc --explain E0308`.
error: could not compile `control_flow` due to previous error
```

これは、`number`の型を知ることで、コンパイラは`number`が使用する全ての場所でその型が有効であることを確認できます。`number`の型が実行時にのみ決定される場合、Rustはそれを行うことができません。変数の複数の仮想型を追跡する必要がある場合、コンパイラはより複雑になり、コードに関する保証が少なくなります。

## ループによる繰り返し

コードのブロックを複数回実行すると便利なことがよくあります。Rustはいくつかの`loop`を提供しています。ループ本体内のコードを最後まで実行し、すぐに最初に戻ります。

Rustには、`loop, while, for`の3種類のループがあります。

**loop**

`loop`キーワードは、コードのブロックを永遠に、明示的に低すするように指示するまで、コードのブロックを何度も実行するようにRustに指示します。

```rust
loop {
    println!("again!");
}
```

このプログラムを実行すると次のような出力になります。

```shell
cargo run
again!
again!
again!
...
^Cagain!
```

`^C`記号は、`ctrl-c`を押した場所を表します。割り込み信号を受信した時にコードがループ内のどこにあったかによって、`^C`の後に出力される単語が表示される場合とされない場合があります。

幸いなことに、Rustはコードを使用してループから抜け出す方法も提供しています。`break`キーワードをループ内に配置して、ループの実行をいつ停止するかをプログラムに伝えることができます。

また、`continue`はループ内でループの残りのコードをスキップして次のループに進めるようにプログラムに指示します。

**ループから値を返す**

`loop`の用途の1つに、スレッドがジョブを完了したか確認し、失敗する可能性がある操作を再施行することがあります。また、ループの結果を返すこともできます。`break`キーワードには、`return`のように返される値を追加することもできます。

```rust
let mut counter = 0;

let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;
    }
};

println!("The result is {result}");
```

このプログラムの出力は、`The result is 20`となります。

**ループラベル**

ループ内にループがある場合、`break, continue`は最も内側のループに適用します。必要に応じて、ループにループラベルを指定して、`break, continue`が内側のループではなくラベル付きループに適用されるように指定したりできます。

```rust
let mut count = 0;

'counting_up': loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
        println!("remaining = {remaining}");
        if remaining == 9 {
            break;
        }
        if count == 2 {
            break 'counting_up';
        }
        remaining -= 1;
    }
    count += 1;
}

println!("End count = {count}");
```

このプログラムの出力は次のようになります。

```shell
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2
```

**while**

多くの場合プログラムは、ループ内で条件を評価する必要があります。条件がtrueの場合、ループを実行し、falseになると`break`を呼び出しループを停止するなどです。`while`では、このパターンは非常に一般的であるため、すでに組み込まれています。

```rust
let mut number = 3;

while number != 0 {
    println!("{number}!");
    number -= 1;
}

println!("LIFTOFF!!!");
```

この構成により、`if, else`を使った時のネストがなくなり、わかりやすくなります。

**for**

`while`構文で、配列のようなコレクションの要素に対してループを使うことができます。

```rust
let a = [10, 20, 30, 40, 50];
let mut index = 0;

while index < 5 {
    println!("The value is: {}", a[index]);
    index += 1;
}
```

ただ、このコードではエラーが発生しやすく、インデックス、テスト条件が正しくない場合、プログラムがパニックに陥る可能性があります。例えば条件を、`index < 100`にした場合、パニックになります。また、コンパイラはランタイムコードを追加して、ループの反復ごとにインデックスが配列の境界内にあるかチェックを行うので、速度も遅くなります。

`for`ループを使用すれば、これらの問題を解決することができます。

```rust
let a = [10, 20, 30, 40, 50];

for element in a {
    println!("The value is: {element}");
}
```

これでコードの安全性が向上し、配列の末尾を越えたり、少なくなったりすることがなくなり、バグの可能性が排除されます。

`for`ループは、Rustで最も一般的に使用されるループ構造になっています。上記で書かれている`while`ループのコードも、ほとんどのRustaceansは`for`ループを使用します。カウントダウンのプログラムも次のように書くことができます。

```rust
for number in (1..4).rev() {
    println!("{number}!");
}
println!("LIFTOFF!!!");
```

### 概要

これで第3章は終わりです。お疲れ様でした！この章で、変数、スカラー、複合データ型、関数、コメント、`if`式、ループについて学びました。復習も兼ねて次のことを行うプログラムを作成してみましょう。

- 華氏と摂氏の温度を変換する。
- n番目のフィボナッチ数を生成する。
- 歌の繰り返しを利用して、クリスマスキャロル「クリスマスの12日間」の歌詞を印刷する。

次の章では、他のプログラミング言語には一般的に存在しないRustの概念、つまり所有権について説明します。
