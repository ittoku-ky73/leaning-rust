# `match` Control Flowの構成

> 参考: https://doc.rust-lang.org/stable/book/ch06-02-match.html

Rustには`match`という強力な制御フロー構造があります。値を一連のパターンと比較して、どのパターンとマッチしたかに基づいてコードを実行することができます。
パターンはリテラル値、変数名、ワイルドカードなど、様々なもので構成できます。第18章では、様々な種類のパターンとその機能について説明します。
`match`の威力は、パターンの表現力とコンパイラが、すべての可能なケースが処理されていることを確認することから生まれます。

値はmatchの各パターンを通過し、値が最初のパターンで適合するコードブロックを実行します。

matchを使ったコインの例を見ていきましょう。以下のコードは、未知のアメリカ合衆国硬貨を受け取って、計数機と同じようにそれがどの硬貨かを判断してその値をセントで返す関数について書いています。

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

上記の関数について説明します。まず、`match`キーワードに続いて、式をリストアップします。これは`if`で使用される式と似ていますが、大きな違いがあります。
`if`では、式は`bool`値を返す必要がありますが、ここでは任意の型を返すことができます。ここでは`Coin enum`型を返しています。

`match`では、パターンとコードの2つからできています。上記を例にすると、最初のパターンは`Coin::Penny`で、コードは`=>`演算子の後ろです。この場合のコードは値1だけです。各アームと次のアームはコンマで区切ります。

`match`式が実行されると、結果として得られる値を上から順番にパターンを比較します。
あるパターンが値に一致すれば、そのパターンに関連するコードが実行されます。マッチしなければ、次のパターンと比較します。上記のコードだと`match`は4つのアームを持っています。

各アームに関連するコードは式であり、一致するアームの式の結果の値は、一致する式全体に対して返される値です。

各アームが値を返すだけの短いコードであれば通常、括弧は使いません。アームで複数行のコードを実行したい場合は、括弧を使用する必要があります。

```rust
match coin {
    Coin::Penny => {
        println!("Lucky penny!");
        1
    }
    // --snip--
}
```

## 値にバインドされるパターン

`match`アームのもう1つの便利な機能は、パターンにマッチする値の部分にバインドできる点です。これは、`enum`のバリアントから値を抽出する方法です。

例として、`enum`を少し変更して、その内部にデータを保持するようにしてみましょう。
1999年から2008年まで、アメリカ合衆国は片面に50州それぞれの異なるデザインを施したクォーターを鋳造していました。他の硬貨には州のデザインはないので、クォーターだけがこの特別な価値を持っています。
これを、UsState値を内部に保存するためにQuarterバリアントを変更することで、この情報を列挙に追加できます。

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

例えば、ある友人が50州全てのクォーターを集めようとしているとします。小銭をコインの種類ごとに分類しながら、それぞれのクォーターに関連する州を当てはめて、もしその友人が持っていないものがあれば、コレクションに加えることができます。

下記のコードでは、バリアントである`Coin::Quarter`の値にマッチするパターンに`state`という変数を追加しています。`Coin::Quarter`がマッチすると、`state`変数はそのクォーターの`state`の値にバインドされ、コードが実行されます。

```rust
match coin {
    // --snip--
    Coin::Quarter(state) => {
        println!("State quarter from {:?}!", state);
        25
    }
}
```

`value_in_cents(Coin::Quarter(UsState::Alaska))`を呼び出すと、`Coin::Quarter(UsState::Alaska)`となります。この値を各マッチアームと比較すると、`Coin::Quarter(state)`に到達するまでマッチしません。

## `match`と`Option<T>`

前のセクションでは、`Option<T>`を使う時に`Some case`から内部の`T`値を取り出したいと言いました。コイン列挙型でやったように`match`を使って`Option<T>`を扱うこともできます。
コインを比較する代わりに`Option<T>`の変種を比較しますが、`match`式が動作する方法は変わりません。

例として、`Option<i32>`を受け取って、中に値があればその値に1を足し、なければNone値を返す関数を書きます。

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

`plus_one`を詳しくみていきましょう。まずは`five`変数に`Option<T>`型の`Some(5)`を代入します。つぎに`six`変数に`plus_one(five)`を呼び出し、返り値の`Some(6)`を代入します。
なぜ`Some(6)`になるのかというと、`five`変数がパターンの`Some(i)`と一致したため、そのパターンのコードが実行されたためです。これは、`none`変数でも同じことが言えます。

`match`と`enum`の組み合わせは多くの場面で役に立ちます。Rustのコードではこのパターンをよく見かけます。`enum`に対して`match`し、変数にその中のデータをバインドし、それに基づいてコードを実行します。
初めはちょっと難しいですが、一度慣れると、全ての言語でこのパターンを使いたいと思うようになります。常にユーザーのお気に入りです！

## 徹底したマッチング

`plus_one`関数のマッチのNoneの行を削除するとコンパイルできません。

```rust
match x {
    Some(i) => Some(i + 1),
}
```

また、以下のようなエラーを吐きます。

```none
cargo run
   Compiling match_control_flow v0.1.0 (projects/match_control_flow)
error[E0004]: non-exhaustive patterns: `None` not covered
   --> src/main.rs:28:11
    |
28  |     match x {
    |           ^ pattern `None` not covered
    |
note: `Option<i32>` defined here
   --> rustlib/src/rust/library/core/src/option.rs:522:5
    |
518 | pub enum Option<T> {
    | ------------------
...
522 |     None,
    |     ^^^^ not covered
    = note: the matched value is of type `Option<i32>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
    |
30  ~         Some(i) => Some(i + 1),
31  ~         None => todo!(),
    |

For more information about this error, try `rustc --explain E0004`.
error: could not compile `match_control_flow` due to previous error
```

Rustは、`match`で全てのケースをカバーしていないことを知り、どのパターンをカバーすれば良いか知っています。Rustの`match`は網羅的です。コードを有効にするためには、全ての可能性を網羅しなければなりません。
特に、`Option<T>`の場合、Noneのケースを明示的に処理し忘れることをRustが防いでくれるので、NULLがあるかもしれないのに値があると思い込んでしまい、先に述べたような億劫なミスを防いでくれるのです。

## Catch-allパターンと`_`プレースホルダー

列挙型を使うといくつかの特定の値に対しては特別な動作をさせ、それ以外の値はデフォルトで動作させたい時があります。
例えば、サイコロを振って3を出したら、プレイヤーは移動せず、新しい帽子を手に入れるというゲームを実装するとします。7を出したら、派手な帽子を失います。それ以外は、ゲームボード上をその数だけ移動します。
サイコロの結果はランダムな値ではなく、ハードコードします。また他のすべてのロジックは、ボディを持たない関数で表現します。

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
```

このコードは、`u8`が持つすべての値をリストアップしていないにも関わらず、コンパイルされます。なぜなら最後のパターンは具体的にリストアップされていないすべての値にマッチするからです。
このCatch-allパターンは、`match`は網羅的でなければならないという要件を満たしています。パターンが順番に評価されるため、Catch-allパターンは最後に置かなければなりません。

Rustには、Catch-allしたいけどCatch-allパターンに含まれる値を使いたくない場合に使えるパターンもあります。`_`は特殊なパターンで、任意の値にはマッチしますがバインドはされません。これはRustにその値は使わないと伝えるもので、Rustが未使用の変数について警告を発することはありません。

ゲームのルールを変えてみましょう。今度は3か7以外を出したら、もう一度出題しなければならないようにしてみましょう。

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
```

最後にもう一度ゲームのルールを変えて、3か7以外を出したら自分の番には何も起こらないようにします。それを表現するには、`_`に付随するコードとユニット値を使用します。

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
```

パターンマッチングについては、第18章でさらに詳しく説明します。次は、`if let`構文を学びます。これはマッチ式だと少し語弊があるような場合に便利です。
