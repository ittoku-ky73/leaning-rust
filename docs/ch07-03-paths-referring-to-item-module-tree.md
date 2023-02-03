# モジュールツリー内の項目を参照するパス

> Ref: https://doc.rust-lang.org/stable/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html

Rustはモジュールツリーがどこにあるのか示すのにパスを使います。関数を呼び出すには、そのパスを知っている必要があります。

**パスを知る方法**

- *absolute path*は、クレートルートから始まるフルパスです。外部クレートからでは、クレート名から始まり、現在のクレートからでは、リテラルクレートから始まります。
- *relative path*は、現在のモジュールから始まり、`self, super`、現在のモジュールの識別子を使用します。

両方とも1つ以上の識別子をダブルコロン(`::`)で区切ったものになります。

前回の`restaurant`に戻って、`add_to_waitlist`関数を呼び出したいとします。

ここでは、クレートルートに定義された新しい関数`eat_at_restaurant`から`add_to_waitlist`を呼び出す方法を2つ紹介します。

以下のコードの`eat_at_restaurant`関数は、ライブラリクレートで公開したいところなので、`pub`キーワードを使用しています。

```rust
mod front_of_fouse {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
	crate::front_of_house::hosting::add_to_waitlist();
	// Relative path
	front_of_house::hosting::add_to_waitlist();
}
```

`eat_at_restaurant`の`add_to_waitlist`関数を呼び出すときは、絶対パスを使用します。これは同じクレートで定義されているので、`crate`キーワードを使用して絶対パスを開始することが出来ます。
これはシェルでファイルシステムのルートを`/`で開始することに似ています。

また`eat_at_restaurant`と同じ階層に定義されている関数に対してパスを通す場合、`crate::`を除いた相対パスで指定することが出来ます。

相対パスと絶対パスのどちらを使用するかは、プロジェクトに応じて決定します。それか項目の定義コードを、その項目を使用するコードと別々に移動する可能性が高いか、一緒に移動する可能性が高いかによっても決まります。
一般的に、コード定義とアイテムを別々に移動させる可能性が高い場合は、絶対パスが良いでしょう。

まだ上記のコードを実行してもエラーになります。内容を見て見ましょう。

```bash
cargo build
   Compiling restaurant v0.1.0 (/Users/ittoku/Documents/learning/rust/projects/restaurant)
error[E0603]: module `hosting` is private
  --> src/lib.rs:19:28
   |
19 |     crate::front_of_house::hosting::add_to_waitlist();
   |                            ^^^^^^^ private module
   |
note: the module `hosting` is defined here
  --> src/lib.rs:2:5
   |
2  |     mod hosting {
   |     ^^^^^^^^^^^

error[E0603]: module `hosting` is private
  --> src/lib.rs:21:21
   |
21 |     front_of_house::hosting::add_to_waitlist();
   |                     ^^^^^^^ private module
   |
note: the module `hosting` is defined here
  --> src/lib.rs:2:5
   |
2  |     mod hosting {
   |     ^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restaurant` due to 2 previous errors
```

エラー文にはモジュールの`hosting`がプライベートだよと書かれています。
Rustはプライベートセクションにアクセスできません。そのためエラーになっているということです。
このようにモジュールシステムを機能させることで、外側のコードを壊さずに内側のコードのどの部分を変更できるかを知ることが出来ます。

## `pub`キーワードを使ったパスの公開

上記のエラーの解決のために、`hosting`モジュールに`pub`キーワードをつけて、`add_to_waitlist`にアクセスできるようにして見ましょう。

```rust
mod front_of_fouse {
    pub mod hosting {
        fn add_to_waitlist() {}
	}
}
```

しかしこれでもエラーを吐きます。原因をみていきましょう。

```bash
 cargo build
   Compiling restaurant v0.1.0 (/Users/ittoku/Documents/learning/rust/projects/restaurant)
error[E0603]: function `add_to_waitlist` is private
  --> src/lib.rs:19:37
   |
19 |     crate::front_of_house::hosting::add_to_waitlist();
   |                                     ^^^^^^^^^^^^^^^ private function
   |
note: the function `add_to_waitlist` is defined here
  --> src/lib.rs:3:9
   |
3  |         fn add_to_waitlist() {}
   |         ^^^^^^^^^^^^^^^^^^^^

error[E0603]: function `add_to_waitlist` is private
  --> src/lib.rs:21:30
   |
21 |     front_of_house::hosting::add_to_waitlist();
   |                              ^^^^^^^^^^^^^^^ private function
   |
note: the function `add_to_waitlist` is defined here
  --> src/lib.rs:3:9
   |
3  |         fn add_to_waitlist() {}
   |         ^^^^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restaurant` due to 2 previous errors
```

これは、`add_to_waitlist`関数がプライベートだよといっています。`pub`キーワードは祖先のモジュールのコードに対して有効であって、内部のコードは関係ありません。つまり、`add_to_waitlist`関数にアクセスするには、公開する必要があるということです。

では早速修正していきましょう。`fn add_to_waitlist() {}`の前に`pub`キーワードを追加します。

```rust
mod front_of_fouse {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
```

これでコードはコンパイルされます。お疲れ様です。

他のプロジェクトでライブラリクレートを使用できるようする場合、[The Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)が役に立ちます。

> **バイナリとライブラリを含むパッケージのベストプラクティス**
>
> バイナリクレートは`src/main.rs`、ライブラリクレートは`src/lib.rs`にルートを含むことができ、両方ともデフォルトでパッケージ名を持つことが出来ます。

## `super`を使った相対パス

パスの先頭に`super`を使うことで、親モジュールから始まる相対パスを構築することが出来ます。
`super`を使用すると、親モジュールと密接に関連するモジュールで、いつか別の場所に移動するかもしれない時に、簡単にモジュールツリーを変更できます。

では、`src/lib.rs`に以下のコードを追加して動作を確認して見ましょう。

```rust
fn deliver_order() {}

mod back_of_order {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

このコードはコンパイルされます。ここで見るべき点は2つあります。

- モジュール内の関数で、同じモジュール内の関数にアクセスする場合、単純に関数名だけで良い。
- モジュール内の関数で、`super`を使って外部の関数にアクセスする場合、モジュールと同じ階層の関数であれば、`super::deliver_order()`のように書けば良い。

## 構造体と列挙型のパブリック化

`pub`キーワードは、構造体、列挙型に対しても有効です。構造体をパブリックにする場合、構造体自身はパブリックになりますが、フィールドはなりません。フィールドも公開したい場合は、前の例のように`pub`キーワードを使用します。

では`src/lib.rs`を開いて、以下のコードを追加してみましょう

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { 
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

`eat_at_restaurant`の中の3つのコードを1つずつ見ていきます。

`let mut meal = back_of_house::Breakfast::summer("Rye");`では、`meal`という、`backof_house::Breakfast::summer("Rey")`関数の返り値であるBreakfast構造体を、書き込み可能な変数で定義しています。つまり中身は、`{ toast: "Rye", seasonal_fruit: "peaches" }`です。

`meal.toast = String::from("Wheat");`では、Breakfast構造体のtoastフィールドの値を変更しています。

そして最後に出力しています。

下のコメントアウトしてある行を、コメントを外して実行するとエラーになります。理由は、Breakfastのseasonal_fruitフィールドがプライベートであるため、アクセスできないためです。

次は、列挙型の例を見ていきましょう。構造体とは対照的に列挙型ではすべてのバリアントが公開されます。

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

バリアントが公開されていない列挙型はあまり役に立ちません。それにあらゆる場合にすべての列挙バリアントに注釈をつけるのは面倒です。
構造体フィールドは、公開されていなくても役に立つことが多いため、`pub`キーワードをつけますが、列挙型はそうでないため、デフォルトですべてのバリアントが公開されます。

次の節では、`pub, use`キーワードの2つを組み合わせる方法を見ていきます。
