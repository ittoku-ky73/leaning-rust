# トレイト

> Ref: https://doc.rust-jp.rs/book-ja/ch10-02-traits.html

トレイトはRustコンパイラに、特定の型に存在し他の型と共有できる機能について知らせます。
トレイトを使用すると、共通の振る舞いを抽象的に定義できます。
トレイト境界を使用すると、あるジェネリックが特定の振る舞いを持つあらゆる型になり得ることを指定できます。

> 注釈、トレイトは他の言語でよくインターフェイスと呼ばれる機能に類似しています。

## トレイトを定義する

型の振る舞いは、その型に対して呼び寄せるメソッドから構成されます。
異なる型は、それらの型全てに対して同じメソッドを呼び出せるなら、同じ振る舞いを共有することになります。
トレイト定義は、メソッドシグネチャをあるグループにまとめ、なんらかの目的を達成するのに必要な一連の振る舞いを定義する手段です。

例えば、以下のような色々な種類や量のテキストを保持する複数の構造体があるとします。

- `NewArticle`、特定の場所から送られる新しいニュースを保持する構造体
- `Tweet`、新規ツイート、リツイート、その他のツイートへのリプライなのかを示すメタデータを伴う最大280文字の構造体

例として、`NewArticle, Tweet`インスタンスに保存されているデータの概要を表示できるメディアアグリゲータライブラリを作成します。
これを実装するには、各型の概要が必要で、インスタンスに`summarize`メソッドを呼び出して概要を要求する必要があります。
以下のコードではこの振る舞いを表現する`Summary`トレイトを定義しています。

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

ここでは`trait`キーワードを使用してトレイトを定義しています。
波カッコの中にこのトレイトを実装する型の振る舞いを記述するメソッドシグネチャを定義しています。

トレイトには、本体に複数のメソッドを含むことが出来ます。

## トレイトを型に実装する

`Summary`トレイトを定義できたので、メディアアグリゲータでこれを型に実装できます。
以下のコードは`src/lib.rs`に、`Summary`トレイトを`NewArticle, Tweet`構造体上に実装しています。

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

型にトレイトを実装することは、普通のメソッドを実装することに似ています。
違いは`impl`の後に、実装したいとレイトの名前を置き、`for`キーワード、トレイトの実装対象の方の名前を指定することです。
`impl`ブロック内に、トレイと定義で定義したメソッドシグニチャを置き、各シグネチャの後に波カッコを書き、メソッド本体に処理を記述します。

トレイトを実装した構造体のメソッドの呼び出しは次のように書きます。

```rust
use traits::{self, Summary, NewsArticle, Tweet};

let news_article = NewsArticle {
    headline: String::from("Trait example"),
    location: String::from("Japan"),
    author: String::from("ittokun"),
    content: String::from("Hello, World !!!"),
};
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new article: {}", news_article.summarize());
println!("1 new tweet: {}", tweet.summarize());
```

このコードを実行すると以下のような出力がされます。

```bash
1 new article: Trait example, by ittokun (Japan)
1 new tweet: horse_ebooks: of course, as you probably already know, people
```

上のコードでは、`Summary`トレイトと、`NewsArticle, Tweet`を構造体を`lib.rs`に定義し、それを`main.rs`で使用しています。
この`lib.rs`では、クレート専用(`aggregator`)にしています。

トレイト実装で注意すべきことの1つに、トレイトの対象の型が自分のクレートに固有である時のみ、型に対してトレイトを実装できるということです。
例えば、`Display`のような標準ライブラリのトレイトを`agregator`クレートの機能の一部として、`Tweet`のような独自の型に実装したりです。
これは`Tweet`が`aggregator`クレート固有だから可能です。
また、`Summary`を`aggregator`クレートで`Vec<T>`に対して実装することもできます。

しかし、外部のトレイトを外部の型に対して実装することはできません。
例えば、`aggregator`クレート内で`Vec<T>`に対して`Display`トレイトを実装することはできません。
`Display, Vec<T>`は標準ライブラリで定義され、`aggregator`クレート固有でないからです。
この制限は、コヒーレンス(coherence)、孤児のルール(orphan rule)と呼ばれるプログラムの特性の一部で、親の型が存在しないためにそう命名されました。
この規則により、他の人のコードが自分のコードを壊したり、その逆が起きないことを保証してくれます。
この規則がなければ、2つのクレートが同じ型に対して同じトレイトを実装できてしまし、コンパイラはどちらの実装を使うべきかわからなくなってしまいます。

## デフォルト実装

時として、すべての型の全メソッドに対して実装を要求するのではなく、トレイトの全てあるいは一部のメソッドに対してデフォルトの振る舞いがあると便利です。
そうすれば、特定の型にトレイトを実装する際、各メソッドのデフォルト実装を保持するかオーバーライドするか選べます。

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

独自の実装を定義するのではなく、デフォルト実装を使用して`NewsArticle`のインスタンスをまとめるには、空の`impl`ブロックを指定します。

```rust
let news_article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
        "The Pittsburgh Penguins once again are the best \
         hockey team in the NHL.",
    ),
};

println!("New article available! {}", news_article.summarize());
```

`impl Summary for NewsArticle`の`summarize`メソッドが定義されていなければ、次のような出力になります。

```bash
New article available! (Read more...)
```

`Summary`トレイトの`summarize`に変更を加えても、`Tweet`の`Summary`を変更する必要はありません。
理由は、デフォルト実装をオーバーライドするからです。

デフォルト実装は、自らのトレイトのデフォルト実装を持たない他のメソッドを呼び出すことが出来ます。
このようにすれば、トレイとは多くの有用な機能を提供しつつ、実装者は僅から部分しか指定しなくて済むようになります。
例えば、`Summary`トレイトを、実装する必要がある`summarize_author`メソッドを持つように定義し、それから`summarize_author`メソッドを呼び出すデフォルト実装を持つ`summarize`メソッドを定義することもできます。

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

コードを実行すると次のような出力が得られます。`main.rs`は変更しません。

```bash
New article available! (Read more from @Iceburgh...)
```

デフォルト実装は、そのメソッドをオーバーライドしている実装からは呼び出すことはできません。

## 引数としてのトレイト

トレイトを定義し実装する方法を学んだので、次は色々な種類の型を受け付ける関数を定義する方法を学んでいきましょう。

例として、`NewsArticle, Tweet`型に引数の`item`に`summarize`メソッドを呼ぶ`notify`関数を定義してみましょう。

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

引数の`item`には、具体的な型の代わりに、`impl`キーワードとトレイト名を指定します。
この引数は指定されたトレイトを実装しているあらゆる型を受け付けます。
`notify`関数の中身では、`summarize`のような`Summary`トレイトに由来するメソッドを呼び出すことが出来ます。
またこの関数を呼び出すときに、`String, i32`のような他の型を渡すようなコードはコンパイルできません。理由は引数の型である`Summary`を実装していないからです。

### トレイト境界構文

`impl trait`構文は単純なケースは解決できますが、複雑な状態を表現したい場合は向いていません。
トレイト境界(trait bound)と呼ばれる糖衣構文(Syntax sugar)を使用することで、その問題を解決できます。

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

トレイト境界は、山カッコの中にジェネリックな型引数の宣言をおき、型引数の後ろにコロンを挟んでトレイト境界をおくことで定義します。

普通の`impl trait`構文と違い、トレイト境界構文は冗長になる傾向があります。
しかし、以下の例をご覧ください。上の例は`impl trait`構文、下の例はトレイト境界を使った構文の例です。
どちらのコードの方が見やすいでしょうか。

```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {}

pub fn notify<T: Summary>(item1: &T, item2: &T) {}
```

このように、引数に複雑な型を1つ以上指定する場合、トレイト境界構文は有用です。`T`に型を指定し、`item1, item2`の引数の型が同一であることが一目でわかると思います。

### 複数のトレイト境界を`+`構文で指定する

複数のトレイト境界も指定できます。例として、`notify`に`summarize`メソッドを加えて`item`の画面出力形式を使用したいとします。
その場合、`notify`の定義に`item`は`Display, Summary`の両方を実装しなくてはなりません。
このような状況の場合、`+`構文を使用します。以下の例は、`+`構文の使用方法を示しています。両方とも有効です。

```rust
pub fn notify(item: &(impl Summary + Display)) {}

pub fn notify<T: Summary + Display>(item: &T) {}
```

これらの2つのトレイと教会が指定されていれば、`notify`の中で`summarize`を呼び出したり、`{}`を使って`item`をフォーマットすることもできます。

### where句を使ったトレイト境界

たくさんのトレイと境界を使うことによる欠点というものがあります。
それは、関数名と引数リストの間に大量のトレイト教会に関する情報を含む場合です。
Rustではトレイト境界を関数シグネチャの後の`where`句の中で指定することで、シグネチャを読みやすくする機能を持っています。
以下の例では、`where`句を使った場合と、使わなかった場合の2つのコードの比較です。

```rust
pub fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

pub fn some_function<T, U>(_t: &T, _u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    1
}
```

## トレイトを実装している型を返す

`impl trait`構文を戻り値型で指定することで、トレイトの型を返すことが出来ます。

```rust
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

戻り値の型として`impl Summary`を指定することで、具体的にこの関数が何を返すのか明確になります。

この機能は13章で学ぶ「クロージャとイテレータ」を扱う際にとても便利です。
これらの作り出す型は、コンパイラだけが知るものであったり、指定するには長いものもあります。
`impl trait`構文を使えば、非常に長い型を指定することなく、`Iterator`トレイトを実装する型を簡潔に指定することが出来ます。

ただし、`impl trait`は1種類の型を返す場合のみ使えます。
例えば以下のように、戻り値の型に`impl Summary`を指定しつつ、`NewsArticle, Tweet`のどちらかを返すようなコードは失敗します。

```rust
pub fn returns_summarizable_v2(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                    hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}
```

`NewsArticle, Tweet`のどちらかを返すというのは、コンパイラの`impl trait`構文の実装周りの制約により許されていません。
このような振る舞いをする関数を書く方法は、第17章で説明します。

## トレイト境界でlargest関数を修正する

前節で実装した`largest`関数を改良して、ジェネリックな型引数を使用する`largest`関数を定義してみましょう。

おさらいとして、前節ではコードを実行しようとしたとき、このようなエラーが出ていました。

```bash
error[E0369]: binary operation `>` cannot be applied to type `T`
   --> src/lib.rs:102:17
    |
102 |         if item > largest {
    |            ---- ^ ------- T
    |            |
    |            T
    |
help: consider restricting type parameter `T`
    |
98  | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
    |             ++++++++++++++++++++++

For more information about this error, try `rustc --explain E0369`.
```

内容は、`>`を使用して型`T`の2つの値を比較しようとしてエラーになっていました。
大なり演算子(`>`)は、標準ライブラリトレイトの`std::cmp::PartialOrd`でデフォルトメソッドとして定義されています。
この関数を動作させるためには、`T`のトレイト境界に`PartialOrd`を指定する必要があります。
`PartialOrd`はpreludeに含まれているので、スコープに導入する必要はありません。

```rust
fn largest<T: PartialOrd>(list: &[T]) -> T {}
```

しかしコードをコンパイルしようとすると別のエラーが発生します。

```bash
error[E0508]: cannot move out of type `[T]`, a non-copy slice
  --> src/lib.rs:99:23
   |
99 |     let mut largest = list[0];
   |                       ^^^^^^^
   |                       |
   |                       cannot move out of here
   |                       move occurs because `list[_]` has type `T`, which does not implement the `Copy` trait
   |                       help: consider borrowing here: `&list[0]`

error[E0507]: cannot move out of a shared reference
   --> src/lib.rs:101:18
    |
101 |     for &item in list.iter() {
    |         -----    ^^^^^^^^^^^
    |         ||
    |         |data moved here
    |         |move occurs because `item` has type `T`, which does not implement the `Copy` trait
    |         help: consider removing the `&`: `item`

Some errors have detailed explanations: E0507, E0508.
For more information about an error, try `rustc --explain E0507`.
```

このエラーの鍵となる行は、`cannot move out of type [T], a non-copy slice`です。
ジェネリックでない`largest`関数では、最大の`i32, char`を探そうとするだけでした。
`i32, char`のようなサイズが既知の型はスタックに格納できるので、`Copy`トレイトを実装しています。
しかし`largest`関数をジェネリックにすると、`list`引数が`Copy`トレイトを実装しない型を含む可能性も出てきたのです。
結果として、`list[0]`から値を`largest`にムーブできず、エラーに陥ったのです。

このコードを`Copy`トレイトを実装する型だけを使って呼び出すようにしたい場合、`T`のトレイト境界に`Copy`を追加します。

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q', 'l'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

もし`largest`関数を`Copy`を実装する型だけに制限したくなかったら、`Clone`というトレイと境界を持つと指定することもできます。
そうすると、`largest`関数に所有権が欲しいときにスライスの各値をクローンできます。
`clone`関数を使用するということは、`String`のようなヒープデータを持つ型の場合、より多くのヒープ確保が発生する可能性があることを意味します。
そして大量のデータを取り扱っていたら、ヒープ確保に時間がかかることもあります。

`largest`の別の実装方法は、関数がスライスの`T`値への参照を返すようにすることです。
戻り値の型を`T`でなく`&T`に変え、それにより関数の本体を参照するように変更すると、`Clone, Copy`トレイと境界は必要なくなり、ヒープ確保も避けられます。

## トレイト境界を使ったメソッドの条件分け

ジェネリックな型引数を持つ`impl`ブロックにトレイと境界を与えることで、特定のトレイトを実装する型に対するメソッド実装を条件分けできます。

```rust
use std::fmt::Display;

pub struct Pair<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

また、別のトレイトを実装するあらゆる型に対するトレイト実装を条件分けすることもできます。
トレイと境界を満たすあらゆる型にトレイトを実装することは、ブラケット実装(blanket implementation)と呼ばれ、Rustの標準ライブラリで広く使用されています。
例として、標準ライブラリは、`Display`トレイトを実装するあらゆる型に`ToString`トレイトを実装しています。

```rust
impl<T: Display> ToString for T {}
```

標準ライブラリにはこのブラケット実装があるので、`Display`トレイトを実装する任意の方に対して、`ToString`トレイトで定義された`to_string`メソッドを呼び出せるのです。
例として、整数は`Display`を実装するので、このように整数値を対応する`String`値に変換できます。

```rust
let s = 3.to_string();
```

トレイトとトレイト境界により、ジェネリックな方引数を使用して重複を減らしつつ、コンパイラに対して、そのジェネリックな方に特定の振る舞いが欲しいことを指定するコードを書くことが出来ます。
それからコンパイラは、トレイト境界の情報を活用してコードに使用された具体的な型が正しい振る舞いを提供しているか確認できます。
動的型付き言語では、その方に定義されていないメソッドを呼び出せば、実行時にエラーが出るでしょう。
しかし、Rustはこの種のエラーをコンパイル時に移すので、コードが動かせるようになる以前に問題を修正することを強制します。
加えて、コンパイル時にすでに確認したので、実行時の振る舞いを確認するコードを書かなくても済みます。
こうすることで、ジェネリクスの柔軟性を諦めることなくパフォーマンスを向上させます。

すでに使っている他のジェネリクスに、ライフタイムと呼ばれるものがあります。
ライフタイムは、型が欲しい振る舞いを保持していることではなく、必要な間だけ参照が有効であることを保証します。
これについては次節で説明します。
