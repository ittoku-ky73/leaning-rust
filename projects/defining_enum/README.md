# 列挙型の定義

> 参考: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html

構造体では、短径の幅と高さのように関連するフィールドやデータをグループ化することができますが、列挙型では、ある値が可能な値の集合の1つであることを示すことができます。例えば、RectangleはCircle, Triangleを含む一連の可能な形状のうちの1つであるとしたいことがあります。Rustでは、これらの可能性を`enum`としてエンコードすることができます。

なぜこのような場合に構造体よりも列挙型の方が便利で適切なのかを見ていきましょう。例えば、IPアドレスを扱う必要があるとします。現在IPアドレスには、バージョン4と6があります。IPアドレスにはこの2つしかないので、可能な限りすべての種類を列挙することができ、これが列挙の名前の由来になっています。

どのIPアドレスの、バージョン4か6のアドレスになることはできますが、同時にその両方になることはできません。このIPアドレスの特性は、`enum`データ構造を適切なものにします。なぜなら`enum`の値はそのバリエーションのうちの1つでしかないからです。4と6のアドレスも基本的にはIPアドレスなので、コードがどの種類のIPアドレスにも適用される状況を扱う場合は、同じ型として扱われるべきです。

IPAddrKind列挙型を定義し、IPアドレスの種類として`V4, V6`をリストアップすることで、この概念をコードで表現することができます。

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

これでIpAddrKindはカスタムデータ型になり、コードの他の場所で使用できるようになりました。

## 列挙型の値

このように2種類のIpAddrKindのインスタンスを作成することができます。

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

`enum`のバリアンとはその識別子の下に名前空間があり、ダブルコロンで区切る必要があります。これは`IpAddrKind::V4, IpAddrKind::V6`の両方の値が同じ型であるため便利です。これにより任意のIpAddrKindを受け取る関数を定義することができます。

```rust
fn route(ip_kind: IpAddrKind) {}

route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

列挙型にはさらに多くの利点があります。IPアドレスの方についてもう少し考えて見ましょう。現時点では、実際のIPアドレスデータを格納する方法がなく、IPアドレスの種類だけがわかっています。構造体を学んでいるのであれば、この問題に取り組むことができるでしょう。

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
};

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
}
```

ここでは、IpAddrKind型の`kind`フィールドとString型の`address`フィールドの2つのフィールドを持つIpAddr構造体を定義しています。この構造体のインスタンスは2つあり、1つは`home`で、`kind`にIpAddrKind::V4の値を持ち、`address`に`127.0.0.1`を持っています。2つ目も同じような感じです。

しかし、同じ概念を`enum`だけで表現する方がより簡潔です。`enum`を構造体の中に入れるのではなく、各バリアントに直接データを入れることで、`V4, V6`の両方のバリアントが関連するString値を保つことができます。

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

`enum`の各バリアントに直接データをアタッチするので、余分な構造体は必要ありません。定義した`enum`バリアントの名前は、`enum`のインスタンスを構築する関数にもなります。つまり、IpAddr::V4()はStringを引数に取るIpAddr型のインスタンスを返す関数でもあります。
`enum`を定義したことにより、コンストラクタ関数が自動的に定義されるようになりました。

構造体ではなく列挙型を使用することにはもう1つの利点があります。それは各バリアントとは異なるタイプと関連するデータの量を持つことができる点です。
バージョン4タイプのIPアドレスは、常に`0~255`の値を持つ4つの数値要素を持っています。V4型のアドレスを4つの`u8`値として格納し、V6型のアドレスを1つのString値として表現したい場合、構造体では不可能です。
列挙型はこのようなケースを簡単に処理します。

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

バージョン4, 6のIPアドレスを格納するためのデータ構造を定義する方法をいくつか見てきました。しかしIPアドレスを格納し、その種類を符号化することは一般的であり、標準ライブラリにそのはその定義があります。標準ライブラリではIpAddrをどのように定義しているのか見て見ましょう。

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

このコードでは、`enum`変数の中に文字列、数値型、構造体など、どのような種類のデータでも入れることができます。
さらに、別の`enum`も含めることもできます。また、標準ライブラリの型は持ったほど複雑にできているわけではありません。

標準ライブラリにIpAddrの定義が含まれていても、標準ライブラリの定義をスコープに入れていないため、独自の定義を作成し、矛盾なく使用できることに注意してください。
型をスコープに入れることについては、第7章で説明します。

では次に、別の`enum`の例を見ていきましょう。

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

上記の`enum`には次のようなバリアントがあります。

| Variant     | Description                        |
| ----------- | ---------------------------------- |
| Quit        | no data associated with it at all. |
| Move        | named fields like a struct does.   |
| Write       | includes a single String.          |
| ChangeColor | includes three `i32` values.       |

これらの`enum`の定義は、様々な種類の構造体の定義と似ていますが、`enum`では`struct`キーワードを使用せず、すべてのバリアントをMessageタイプの下にまとめて定義します。
以下の構造体は、前述の`enum`バリアントが保持するデータと同じデータを保持することができます。

```rust
struct QuitMessage;  // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);  // tuple struct
struct ChangeColorMessage(i32, i32, i32);  // tuple struct
```

しかしそれぞれ独自の型を持つことなる構造体を使用した場合、単一の型であるMessage enumを使用した場合のように、これらの種類のメッセージを受け取る関数を簡単に定義することができません。

`impl`を使って構造体でメソッドを定義できるように、`enum`でもできます。

```rust
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

上記では、`Message::Write`に`String::from("hello")`を入れた変数`m`を作成し、その入れた値を呼び出す`call`メソッドを定義しています。返り値は`Write("hello")`です。

次は`enum`のOptionについて見ていきましょう。

## Option EnumとNullあたいに対する優位性

ここでは標準ライブラリで定義されているもう1つの列挙型であるOptionを見ていきます。Option型は、値が何かである可能性もあれば、何もない可能性もあるということを符号化します。

例えば、項目を含むリストの最初の項目を要求した時は値を得ることができますが、空のリストの最初の項目をリクエストしても何も得られない、みたいなケースです。
このような型システムで表現することで、コンパイラが処理すべきケースをすべて処理したかどうかチェックできます。これによりプログラミング言語でよく見られるバグを防ぐことができます。

プログラミング言語の設計は、どのような機能を盛り込むかという観点で考えられがちですが、どのような機能を排除するかということも重要です。
Rustには、Null機能がありません。`null`とはそこに値がないことを意味する値です。`null`を持つ言語では、`null`か`null`でないかの2つの状態のどちらかになります。

2009年に発表した「Null References:The Billion Dollar Mistake "の中で、nullの発明者であるTony Hoareはこのように言っています。

> 私はこれを「10億ドルの失敗」と呼んでいます。
>
> 当時、私はオブジェクト指向言語において、参照に関する初の包括的な型システムを設計していました。私の目標は、コンパイラが自動的にチェックを行うことで、すべての参照の使用が絶対に安全であることを保証することでした。
>
> しかし、実装が簡単だからということで、NULL参照を入れる誘惑には勝てなかった。その結果、無数のエラーや脆弱性、システムクラッシュが発生し、この40年間でおそらく10億ドルの痛手と損害を被ったことでしょう。

`null`値の問題は、`null`値を`not-null`値として使おうとすると、何らかのエラーになることです。この性質が蔓延しているため、この手のエラーは非常に起こりやすいのです。

しかし、`null`が表現しようとしているコンセプトは、依然として有用なものです。`null`は、現在無効であるか、何らかの理由で存在しない値です。

問題はその概念ではなく、特定の実装にあるのです。Rustには`null`はありませんが、あたいが存在するかしないかの概念をエンコードできる`enum`があります。この`enum`は`Option<T>`で、標準ライブラリで以下のように定義されています。

```rust
enum Option<T> {
    None,
    Some(T),
}
```

`Option<T>`列挙型は非常に便利なので、プレリュードにも含まれています。要は明示的にスコープに取り込む必要はないです。また、`Some, None`を`Option::`という接頭辞なしで直接使用することができます。`Option<T>`列挙型は依然として普通の列挙型であり、`Some<T>, None`は依然として`Option<T>`型の変種です。

`<T>`構文は、Rustの機能としてまだ説明していませんでした。これはジェネリックな型パラメータで、今のところ知っておくべきことは、`<T>`はOption列挙型の`Some`が任意の方のデータを1つ保持できることを意味し、`T`の代わりに使われる各具現型は全体の`Option<T>`型を異なる型にする、ということです。
以下のコードは、数値型と文字列型を保持するためにOption値を使用する例です。

```rust
let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;
```

`some_number`の型は`Option<i32>`、`some_char`の型は`Option<char>`です。RustはSomeバリアントの内部で値を指定しているので、これらの型を推論することができます。
`absent_number`についてRustはOption型全体をアノテートすることを要求します。コンパイラはNone値だけを見て、対応するSomeバリアントが保持する型を推論することはできません。
ここでは、`absent_number`が`Option<i32>`型であることをRustに伝えています。

Someの値である時、値が存在し、その値がSomeの中に保持されていることを知ります。Noneの値である時、ある意味で`null`と同じことを意味し、有効な値を持たないということです。ではなぜ`Option<T>`が`null`よりも優れていると言えるのでしょうか

`Option<T>`とは異なる型なので、コンパイラは`Option<T>`の値をあたかもそれが間違いなく有効な値であるかのように使わせてくれないのです。以下のコードでは、`Option<i8>`に`i8`を追加しようとしている例です。

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

これは以下のようなエラーを吐きます。

```none
cargo run
   Compiling defining_enum v0.1.0 (projects/defining_enum)
error[E0277]: cannot add `Option<i8>` to `i8`
  --> src/main.rs:59:17
   |
59 |     let sum = x + y;
   |                 ^ no implementation for `i8 + Option<i8>`
   |
   = help: the trait `Add<Option<i8>>` is not implemented for `i8`
   = help: the following other types implement trait `Add<Rhs>`:
             <&'a f32 as Add<f32>>
             <&'a f64 as Add<f64>>
             <&'a i128 as Add<i128>>
             <&'a i16 as Add<i16>>
             <&'a i32 as Add<i32>>
             <&'a i64 as Add<i64>>
             <&'a i8 as Add<i8>>
             <&'a isize as Add<isize>>
           and 48 others

For more information about this error, try `rustc --explain E0277`.
error: could not compile `defining_enum` due to previous error
```

このエラーメッセージは、型が違うのでRustが追加する方法を理解していないと言っています。コンパイラは常に有効な値を保証してくれます。
つまりRustではNULLチェックをする必要がありません！しかし`Option<T>`の場合、値がない可能性を考慮する必要があり、コンパイラはその値を使う前にその問題を処理することを保証してくれます。

つまり、`Option<T>`を`T`に変換してからでないと、`T`の演算を行うことができないということです。これはNULLでも同じことが言えます。

NULLでない値を誤って仮定してしまうリスクを排除することで、より自信を持ってコードを書くことができます。`null`になる可能性のある値を持つためには、その値の型を`Option<T>`にすることで明示的にオプトインする必要があります。
そして、その値を使用する際には、値がNULLの場合を明示的に処理することが要求されます。値が`Option<T>`ではない型を持っている場合はすべて、その値はNULLでないと仮定することができます。
これは、NULLの普及を抑制し、Rustコードの安全性を高めるために、Rustで意図的に行われた設計上の決定です。

では、`Option<T>`型のあたいがある時に、その値を使えるようにSomeバリアントから`T`値を取り出すにはどうしたら良いのでしょうか。`Option<T>`列挙型には、様々な場面で役立つメソッドが多数用意されており、このドキュメントで見ることができます。
`Option<T>`のメソッドに慣れることは、Rustを使いこなす上で非常に役に立ちます。

`Option<T>`値を使用するためには、それぞれのバリアントを処理するコードを用意したいものです。`Some(T)`が、内部の`T`を使用できる場合にのみ実行されるコードと、Noneが、`T`値を使用できない場合に実行される別のコードも必要になります。
`match`式は、列挙型で使用した場合に実現する制御フロー構造体です。列挙型のどのバリアントを持つかによって異なるコードを実行し、そのコードはマッチング値の内部のデータを使用できます。
