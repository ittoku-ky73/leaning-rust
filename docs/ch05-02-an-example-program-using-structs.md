# 構造体を用いたプログラム

> 参考：https://doc.rust-lang.org/book/ch05-02-example-structs.html

構造体を学ぶために、長方形の面積を計算するプログラムを書いて見ましょう。まずは単一変数を使用して、構造体を使うようになるまでリファクタリングしていきます。

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

プログラムの出力は次のとおりです。

```none
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/struct_rectangle`
The area of the rectangle is 1500 square pixels
```

このプログラムをもっとわかりやすく読みやすくしていきます。

`area`関数は、1つの長方形の面積を計算することになっていますが、関数には2つのパラメータがあり、パラメータが関連しているということはどこにも書かれていません。ここは幅と高さを一緒にグループ化したほうが読みやすく、管理しやすいでしょう。これにはタプルを使用します。

## taple型でリファクタリング

タプルを使用したプログラムです。

```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

引数を1つにしたことで、コードが少しわかりやすくなりました。が、タプルの各要素にインデックスをつけなければならず、計算がわかりにくくなっています。

タプルのインデックスが、幅が0、高さが1で、とてもわかりにくいです。コードの中でデータの意味を伝えていないため、エラーが発生しやすくなっているのです。

## Structsを使ったリファクタリング

構造体を使用して、データにラベルをつけることで意味を付加します。

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

ここでは構造体を定義して、`Rectangle`という名前にしています。括弧の中では、`width, height`を定義しています。

この短形は、`Rectangle`構造体のインスタンスを借用したもので、その型は不変です。

`area`関数は`Rectangle`インスタンスの`width, height`フィールドにアクセスします。こうすることで、値が関連していることが伝わり、タプルのインデックス値を使用せず、値に説明的な名前をつけることができます。

## 派生形質で便利な機能を追加する

プログラムのデバッグ中に`Rectangle`のインスタンスを表示して、フィールドの値を見ることができれば便利です。

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1);
}
```

しかし、このプログラムはエラーを引き起こします。

```none
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```

`println!`マクロは、多くの種類のフォーマットを行うことができますが、デフォルトではDisplayフォーマットを使用するように指示しています。これまで見てきたプリミティブ型は、デフォルトでDisplayを実装しています。しかし構造体は、表示が増える可能性があり、またどのように出力をフォーマットすべきかあまり明確でないため、Displayの実装が用意されていません。

エラーを読み進めていくと、こんな親切な注意書きがある。

```none
= help: the trait `std::fmt::Display` is not implemented for `Rectangle`
= note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

試して見ましょう。`println!`マクロの括弧の中に指定子`:?`をおくことで、`Debug`という出力形式を使用するように指示します。`Debug trait`を使用すると、開発者にとって便利な方法で構造体を表示することができ、デバッグ中にその値を確認することができます。

しかしこれだけではうまくいきません。

Rustにはデバッグ情報を出力する機能がありますが、構造体でこの機能を利用できるようにするには、明示的に選択する必要があります。そのためには構造体の定義の直前に外部属性`#[devide(Debug)]`を追加します。

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

こうすることでやっと、構造体を出力することができます。

```none
cargo run
   Compiling struct_rectangle v0.1.0 (/Users/ittoku/Documents/learning-rust/projects/struct_rectangle)
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/struct_rectangle`
rect1 is Rectangle { width: 321, height: 543 }
```

また、大きな構造体を作成する場合、もう少し読みやすい出力があると便利です。この場合、指定子に`:#?`を使用することで次のように出力することができます。

```none
cargo run
   Compiling struct_rectangle v0.1.0 (/Users/ittoku/Documents/learning-rust/projects/struct_rectangle)
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/struct_rectangle`
rect1 is Rectangle {
    width: 321,
    height: 543,
}
```

`println!`とは別に、`dbg!`というマクロがあり、こちらも値を出力することができます。

`dbg!`マクロは、式の所有権を取得し、コード内に`dbg!`マクロを呼び出したファイルと行番号を、式の結果値とともに表示し、値の所有権を返します。

> `dbg!`マクロを呼び出すと、標準出力のコンソールストリーム(stdout)に出力します。`stdout, stderr`については、第12章で説明します。

では、`dbg!`マクロを使用した、出力を確認しましょう。

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
```

`dbg!`は、式の値の所有権を返すので、`width`フィールドの中でも使うことができます。この例の出力は次のようになります。

```none
cargo run
   Compiling struct_rectangle v0.1.0 (/Users/ittoku/Documents/learning-rust/projects/struct_rectangle)
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/struct_rectangle`
[src/main.rs:38] 30 * scale = 60
[src/main.rs:41] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```

最初の出力は、`src/main.rs`の10行目で`30 * scale`という式をデバッグしており、その結果の値は60となっています。次の出力は、`src/main.rs`の41行目でRectangle構造体である`&rect1`の値を出力しています。`dbg!`マクロは、コードが何をおこなっているかを把握しようとするときに、本当に役に立ちます。

Debug特性に加えて、`derive`属性と一緒に使うことで、カスタムタイプに便利な動作を追加できる特性を提供しています。これらは付録Cに書かれています。第10章ではこれらの特質をカスタム動作で実装する方法と、独自の特質を作成する方法を説明します。また、`derive`以外にも多くの属性があります。詳しくは、Rustリファレンスの「属性」のセクションを参照してください。

`area`関数は、長方形の面積だけを計算します。他の型では動作しないので、この動作をRectangle構造体にもっと密接に結びつけておくと便利です。次のセクションで、`area`関数をRectangle型に定義された`area`メソッドにする方法を見ていきましょう。
