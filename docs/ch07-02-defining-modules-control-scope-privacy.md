# スコープとプライバシーを制御するモジュールの定義

> Ref: https://doc.rust-lang.org/stable/book/ch07-02-defining-modules-to-control-scope-and-privacy.html

この節では、モジュールとモジュールシステムの他の部分、すなわちアイテムに名前をつけることができるパス、パスをスコープに入れる為の`use` キーワード、およびアイテムを公開する為の`pub`キーワードについて見ていきます。
また、`as`キーワード、外部パッケージ、`glob`演算子についても見ていきます。

まず、今後コードを整理する際に参照しやすいように、ルールの一覧表を作成し、その後、各ルールについて詳しく見ていきます。

## モジュールチートシート

モジュール、パス、`use, pub`キーワードがコンパイラでどのように動作するか、多くの開発者がどのようにコードを構成しているのか、などについて簡単なリファレンスを提供します。
この節では、それぞれのルールを見ていきますが、モジュールがどのように機能するか参照するのに最適な場所となっています。

- **Start from the crate root**、クレートをコンパイルする時、コンパイラはまずクレートのルートファイル（ライブラリクレートは`src/lib.rs`、バイナリクレートは`src/main.rs`）を見てコンパイルするコードを探します。
- **Declaring modules**、クレートルートファイルでは、新しいモジュールを宣言することが出来ます。
例えば、`mod garden;`で`garden`モジュールを宣言したとして、コンパイラは以下のような場所でモジュールのコードを探します。
    - インラインで、`mod garden`に続くセミコロンを置き換えるカッコの中
	- `src/garden.rs`ファイルの中
	- `src/garden/mod.rs`ファイルの中
- **Declaring submodules**、クレートルート以外のファイルでもサブモジュールを宣言することが出来ます。
例えば、`src/garden.rs`で`mod vegetables;`を宣言するとして、コンパイラは親モジュールの名前のディレクトリの中の以下のような場所でサブモジュールのコードを探します。
    - インラインで、`mod begetables`に続くセミコロンを置き換えるカッコの中
	- `src/garden/vegetables.rs`ファイルの中
	- `src/garden/vegetables/mod.rs`ファイルの中
- **Paths to code in modules**、一度モジュールがクレートの一部になれば、プライバシー規則が許す限り同じくレートのどこからでも、コードへのパスを使用してそのモジュールのコードを参照することが出来ます。
例えば、`garden vegetables`モジュールの`Asparagus`タイプは、`crate::garden::vegetables::Asparagus`で見つけることが出来ます。
- **Private vs public**、デフォルトでモジュール内のコードは、その親モジュールからみてプライベートです。
モジュールを公開するには、`mod`の代わりに`pub mod`を宣言し、`public`モジュール内の項目を同様に`public`にするには、宣言の前に`pub`キーワードを使用します。
- **The `use` keyword**、スコープ内では、`use`キーワードはアイテムへのショートカットを作成し、長いパスの繰り返しを減らすことが出来ます。
`crate::garden::vegetables::Asparagus`を参照するスコープは、`use crate::garden::vegetables::Asparagus`とショートカットを作成することが出来ます。

ここではこれらのルールを説明するために、`backyard`というプロジェクトを参考に見ていきます。

```bash
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

`src/main.rs`を以下のコードに置き換えます。

```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
```

`src/garden.rs`ファイルを作成し、以下のコードに置き換えます。

```rust
pub mod vegetables;
```

`src/garden/vegetables.rs`ファイルを作成し、以下のコードに置き換えます。

```rust
#[derive(Debug)]
pub struct Asparagus {}
```

そして`cargo run`を実行すると以下のような出力になります。

```bash
cargo run
   Compiling backyard v0.1.0 (/Users/ittoku/Documents/learning/rust/projects/backyard)
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/backyard`
I'm growing Asparagus!
```

## モジュール関連コードのグループ化

モジュールはクレート内のコードを整理し、読みやすく、再利用しやすくすることが出来ます。
また、モジュール内のコードはデフォルトでプライベートなので、モジュールはアイテムのプライバシーをコントロールすることもできます。
プライベートな項目は、外部で使用できない内部実装の詳細です。モジュールとその中の項目は公開することで、それらを使用したり依存したりできるようになります。

例として、レストランの機能を提供するライブラリのクレートを書いて見ましょう。
レストランを実装するのではなく、コードの構成に集中するために、関数のシグネチャは定義に焦点を当てます。

レストランでは、*front of house*、*back of house*があり、*front of house*では、お客様がいる場所でホストがお客様を座らせ、サーバーが注文と支払いを行い、バーテンダーが飲み物を作るといったことを行います。
*back of house*では、シェフやコックがキッチンで働き、皿洗いが皿を洗い、マネージャが管理業務を行ったりします。

このようにクレートを構成するには、関数をネストされたモジュールに整理します。
`cargo new restaurant --lib`という新しいライブラリを作成し、`src/lib.rs`に以下のコードを入力し、モジュールと関数シグネチャをいくつか定義します。

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

モジュールを定義するには、`mod`キーワードの後にモジュール名をつけます。モジュールの本文は`()`で囲みます。
また、モジュールの中にモジュールを入れることもできます。モジュールは構造体、列挙型、定数、トレイト、そして以下のようなアイテムの定義の保持をすることが出来ます。

```bash
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

モジュールツリーは、コンピュータのファイルシステムのディレクトリツリーと似ています。みやすいですね。
