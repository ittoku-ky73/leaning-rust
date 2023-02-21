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
