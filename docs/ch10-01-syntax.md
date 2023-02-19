# ジェネリックなデータ型

> Ref: https://doc.rust-jp.rs/book-ja/ch10-01-syntax.html

関数シグネチャや構造体などの要素の定義を生成するのに、ジェネリクスを使用することが出来ます。
それはさらに他の多くの具体的なデータ型と使用することもできます。
まずはジェネリクスで関数、構造体、列挙型、メソッドを定義する方法を見てみましょう。
それから、ジェネリクスがコードのパフォーマンスに与える影響を説明します。

## 関数定義

ジェネリクスを使用する関数を定義するとき、通常は引数や戻り値のデータ型を指定する関数のシグネチャにジェネリクスを配置します。
そうすることでコードがより柔軟になり、コードの重複を阻止しつつ、関数の呼び出し基により多くの機能を提供します。

```rust
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q', 'z'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}
```

`largest_i32`関数はスライスから最大値の`i32`を探す関数で、`largest_char`関数はスライスから最大値の`char`を探す関数です。
関数本体には同じコードがあるので、単独の関数にジェネリックな型引数を導入してこの重複を排除しましょう。

これから定義する新しい関数の型を引数にするには、ちょうど関数の値引数のように型引数に名前をつける必要があります。
型引数の名前にはどんな識別子も使用できますが、`T`を使用します。
これは慣習で、Rustの引数は短く（たった1文字で）、Rustの型の命名規則がキャメルケースだからです。
"type"の省略形なので、`T`が多くのRustプログラマの規定の選択になります。

関数本体で引数を使用するとき、コンパイラがその名前を把握できるようにシグネチャでその引数名を宣言しなければなりません。同様に型引数名を関数シグネチャで使用する際にも、型弾き数名を宣言します。
ジェネリックな`largest`関数を定義するために、型名宣言を山カッコ(`<>`)内、関数名と引数リストの間に配置します。

```rust
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_v2(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q', 'z'];

    let result = largest_v2(&char_list);
    println!("The largest char is {}", result);
}
```

上のコードをコンパイルしようとすると、以下のようなエラーが出ます。

```rust
error[E0369]: binary operation `>` cannot be applied to type `T`
  --> src/main.rs:95:21
   |
95 |             if item > largest {
   |                ---- ^ ------- T
   |                |
   |                T
   |
help: consider restricting type parameter `T`
   |
91 |     fn largest_v2<T: std::cmp::PartialOrd>(list: &[T]) -> T {
   |                    ++++++++++++++++++++++

For more information about this error, try `rustc --explain E0369`.
error: could not compile `generics` due to previous error
```

注釈では`std::cmp::PartialOrd`に触れています。これは、トレイトです。トレイトについては次節で説明します。
とりあえずこのエラーは、`largest`の本体は、`T`がなりうる全ての可能性のある型に対して動作しないと述べています。
本体で型`T`の値を比較するために、値が順序づけ可能な型のみしか使用できないのです。
比較を可能にするために、標準ライブラリには型に実装できる`std::cmp::PartialOrd`トレイトがあります。
ジェネリックな型が特定のトレイトを持つと指定する方法はまだ説明しませんが、先にジェネリックな型引数を使用する他の方法を見ていきましょう。

## 構造体定義

構造体を定義して`<>`記法で1つ以上のフィールドにジェネリックな型引数を使用することもできます。

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

構造体定義でジェネリクスを使用する記法は、関数定義のものと似ています。
山カッコ内に型引数の名前を構造体名の直後に宣言することで、ジェネリックな型を使用できます。

ジェネリックな型を1つだけ使用して`Point<T>`を定義したので、この定義は、`Point<T>`構造体が何らかの型`T`に関して、ジェネリックであると述べています。
その型が何であれ、`x, y`のフィールドは両方とも同じ型である必要があることに注意してください。

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
}
```

上記の例をコンパイルしようとするとエラーを吐きます。
内容は、integerとfloatの型不一致によるエラーです。

```bash
error[E0308]: mismatched types
  --> src/main.rs:98:42
   |
98 |         let wont_work = Point { x: 5, y: 4.0 };
   |                                          ^^^ expected integer, found floating-point number

For more information about this error, try `rustc --explain E0308`.
```

解決策は、`Point`構造体に複数のジェネリックな型引数を使用することです。

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

ジェネリクスは数個以上使用すると、コードが読みづらくなります。コードで多くのジェネリックな型が必要なときは、コードの小分けが必要なサインかもしれません。

## 列挙型定義

構造体のように、列挙子にジェネリックなデータ型を保持する列挙型を定義することが出来ます。
標準ライブラリが提供している`Option<T>`列挙型を見てみましょう。

```rust
enum Option<T> {
    Some(T),
    None,
}
```

`Option<T>`列挙型を使用することで、オプショナルな値があるという抽象的な概念を表現でき、`Option<T>`はジェネリックなので、オプショナルな値の型に関わらず、この抽象を使用できます。

列挙型でも複数のジェネリックな型を使用できます。`Result`列挙型の定義を見てみましょう。

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

例えばファイルを開くコードの場合、成功したときは`T`に`std::fs::File`が入り、失敗したときは`E`に`std::io::Error`が入ります。

自分のコード内で、保持している値の型のみが異なる構造体や列挙型定義の場面を認識したら、代わりにジェネリックな型を使用することで重複を避けることが出来ます。

## メソッド定義

定義にジェネリックな型を使うメソッドを、構造体や列挙型に実装することもできます。

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

`impl`の直後に`T`を宣言することで、型`Point<T>`にメソッドを実装していることを指定するために、`T`を使用することが出来ます。
`impl`の後に`T`をジェネリックな型として宣言することで、コンパイラは`Point`の山カッコの型が、具体的な型ではなくジェネリックな型であると認識することが出来ます。

例えば、`Point<f32>`と指定することもできます。

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

このコードは、`Point<f32>`に`distance_from_origin`というメソッドを定義し、`T`が`f32`でない`Point<T>`の他のインスタンスにはこのメソッドが定義されないことを意味します。
このメソッドは、座標点からどれだけ離れているかを測定し、浮動小数点数にのみ利用可能な数学的処理を使用します。

構造体定義のジェネリックな型引数は、必ずしもその構造体のメソッドシグネチャで使用するものと同じにはなりません。

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("the p3 is: x = {}, y = {}", p3.x, p3.y);
}
```

上記のコードは、`Point<T, U>`に`mixup`メソッドを定義し、他の`Point`を引数として取っており、この引数は`mixup`を呼び出している`self`の`Point`とは異なる型の可能性があります。

上記の出力は、"the p3 is: x = 5, y = c"となります。これは`p1.x`と`p2.y`の値が`p3`に代入されたことを意味しています。

この例でわかることは、一部のジェネリックな引数は`impl`で宣言され、他の一部はメソッド定義で宣言される場面をデモすることです。
ここで、ジェネリックな引数`T, U`は`impl`の後に宣言されています。
これは構造体定義に当てはまるからであり、ジェネリックな引数`V, W`は`fn mixup`の後に宣言されています。なぜならこのメソッドにしか関係ないからです。

## ジェネリクスを使用したコードのパフォーマンス

ジェネリックな型引数を使用すると、実行時にコストが発生するのか思うかもしれません。
Rustのジェネリクスは具体的な型があるコードよりもジェネリックな型を使用したコードを実行するのが遅くならないように実装されています。

コンパイラはこれを、ジェネリクスを使用しているコードの単相化をコンパイル時に行うことで達成しています。
単相化(monomorphization)は、コンパイル時に使用されている具体的な型を入れて、ジェネリックなコードを特定のコードに変換する家庭のことです。

この過程においてコンパイラは、ジェネリックな関数を作成するために使用した手順と真逆のことをしています。
コンパイラはジェネリックなコードが呼び出されている箇所全てを見て、ジェネリックなコードが呼び出されている具体的な型のコードを生成するのです。

標準ライブラリの`Option<T>`列挙型を使用する例でこれが動作するか見てみましょう。

```rust
let integer = Some(5);
let float = Some(5.0);
```

このコードをコンパイルすると、単相化を行います。その過程でコンパイラは、`Option<T>`のインスタンスに使用された値を読み取り、2種類の`Option<T>`を識別します。
コンパイラは、`Option<T>`のジェネリックな定義を`Option_i32`と`Option_f64`に展開し、これによりジェネリックな定義を特定の定義と置き換えます。

単相化されたバージョンのコードは、以下のようになります。ジェネリックな`Option<T>`が、コンパイラが生成した特定の定義に置き換えられています。

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

Rustでは、ジェネリックなコードを書くインスタンスで型を指定したコードにコンパイルするので、ジェネリクスを使用することに対して実行時にコストを払うことはありません。
コードを実行すると、それぞれの定義を手作業で複製したときのように振舞います。
単相化の過程により、Rustのジェネリクスは実行時に究極的に効率的になるのです。
