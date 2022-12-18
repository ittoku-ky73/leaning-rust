# Guessing Game

この章では、実際のプログラムでそれらを使用する方法を示すことにより、いくつかの一般的なRustの概念を紹介します。`let, match`、メソッド、関連する関数、外部クレートの使用などを学びます！つまり基礎の練習です。

古典的な初心者向けプログラミングの問題である推測ゲームを実装します。仕組みは、1から100までのランダムな整数を生成し、プレイヤーに推測を入力するように求めます。推測を入力すると、プログラムは推測が低いか高いかを示します。推測が正しければ、ゲームはお祝いメッセージを出力して終了します。

## 新しいプロジェクトのセットアップ

以下のコマンドを実行します。

```shell
cargo new guessing_game
cd guessing_game
```

では早速、ビルド＆実行をしてみましょう。

```shell
cargo run
   Compiling guessing_game v0.1.0 (/Users/ittoku/Documents/learning-rust/projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.37s
     Running `target/debug/guessing_game`
Hello, world!
```

## 推測の処理

推測ゲームプログラムの最初の部分では、ユーザー入力を求め、その入力を処理し、入力が期待通りの形式であることを確認します。まず、プレイヤーが推測を入力できるようにします。

ユーザー入力を取得し、結果を出力として出力するには、`io`入出力ライブラリをスコープに入れる必要があります。

```rust
use std::io;
```

デフォルトでは、Rustには標準ライブラリで定義された一連の項目があり、そべてのプログラムのスコープに取り込まれます。このセットは、*prelude*と呼ばれ、その内容はすべて[標準ライブラリのドキュメント](https://doc.rust-lang.org/stable/std/prelude/index.html)で確認できます。

使用したい方がプレリュードにない場合は、`use`ステートメントを使用してその方を明示的にスコープに入れる必要があります。`std::io`ライブラリには、ユーザー入力を受け入れる機能など、多くの便利な機能が提供されます。

あと`println!`は、文字列を画面に出力するマクロです。

### 変数による値の保存

次のようにユーザー入力を格納する変数を作成します。

```rust
let mut guess = String::new();
```

`let`ステートメントを使用して変数を作成することができます。

```rust
let apples = 5;
```

上記は、`apples`という新しい変数を作成し、それを値5にバインドします。Rustの変数はデフォルトで普遍です。つまり、変数に値を与えると、値は変更されません。変数を変更可能にするには、変数名の前に`mut`を追加します。

```rust
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

符号`=`は、変数に何かをバインドしたいことをRustに伝えます。符号の右側は、`guess`にバインドされている値で、`String::new`という`String`の新しいインスタンスを返す関数を呼び出した結果です。`String`は標準ライブラリが提供する文字列型で、UTF-8でエンコードされた拡張可能なテキストビットです。

`new`の`::`構文は、`new`が`String`型の関連関数であることを示しています。関連する関数とは、ある型に対して実装される関数のことです。この`new`関数は、新しい空の文字列を作成します。`new`関数は多くの型に実装されていますが、これは何らかの新しい値を生成する関数の一般的な名前だからです。

つまり、`let mut guess = String::new();`は、新しい空の`String`のインスタンスにバインドされるミュータブル変数を作成したことになります。ふぅ!

### ユーザー入力の受信

次に、`io`モジュールから`stdin`関数を呼び出します。これにより、ユーザー入力を処理できるようになります。

```rust
io::stdin()
    .read_line(&mut guess)
```

プログラムの開始時に`use std::io`でライブラリをインポートしていなかった場合でも、`std::io::stdin`と記述すれば関数を呼び出すこともできます。`stdin`関数は、端末の標準入力へのハンドルを表す型で、`std::io::stdin`のインスタンスを返します。

次に、`.read_line(&mut guess)`は、ユーザーからの入力を得るために、標準入力ハンドルに対して`read_line`メソッドを呼び出しています。また、`read_line`の引数として`&mut guess`を渡し、ユーザ入力をどのような文字列に格納するかを指示しています。`read_line`は、ユーザが標準入力に入力したものを（その内容を上書きすることなく）文字列に追加することなので、その文字列を引数として渡しています。また文字列の引数は、メソッドが文字列の内容を変更できるように、`mutable`である必要があります。

`&`は、この引数が参照であることを示します。コードの複数の部分が1つのデータにアクセスする際に、そのデータを何度もメモリにコピーする必要がないことを示すものです。参照は複雑な機能ですが、Rustの大きな利点の1つは、参照を安全かつ簡単に使用できることです。これは変数と同じように、参照もデフォルトではイミュータブルであるということです。したがって、ミュータブルにするには、&guessではなく、&mut guessと書く必要があります。

### Resultタイプの潜在的な障害の処理

次は、このコードについてみていきます。

```rust
    .expect("Failed to read line");
```

このコードは次のようにも記述することができます。

```rust
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

ただ1行にしてしまうと読みにくいので、分割するのがベストです。

`read_line`では、ユーザーが入力したものをこちらが渡した文字列に入れますが、同時に`Result`の値も返します。`Result`は、`enum`と呼ばれる列挙型で、複数の可能な状態のうちの 1 つになることができる型です。私たちはそれぞれの可能な状態をバリアントと呼んでいます。

`Result`型の目的は、エラー処理情報をエンコードすることです。

`Result`のバリアントには、`Ok, Err`があります。成功したら`Ok`、失敗したら`Err`とそれに関する情報が含まれます。

`Result`のインスタンスには`expect`メソッドがあり、これを呼び出すことによって、`Err`値である場合、`expect`はプログラムをクラッシュさせ、`expect`の引数として渡したメッセージを表示します。`read_line`メソッドが`Err`を返した場合、基盤となるOSからのエラーの結果である可能性があります。また`Ok`値である場合、`expect`は`Ok`が持っている戻り値を受け取り、その値を返します。この場合、その値はユーザーの入力のバイト数です。

もし`expect`を呼び出さずにプログラムをコンパイルすると、次のような警告が出ます。

```shell
cargo build
   Compiling guessing_game v0.1.0 (/Users/ittoku/Documents/learning-rust/projects/guessing_game)
warning: unused `Result` that must be used
  --> src/main.rs:10:5
   |
10 | /     io::stdin()
11 | |         .read_line(&mut guess);
   | |_______________________________^
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: this `Result` may be an `Err` variant, which should be handled

warning: `guessing_game` (bin "guessing_game") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
```

内容は、`read_line`から返された`Result`の値を使用していないことを警告し、プログラムがエラーの可能性を処理していないことを示します。

このエラーの解決方法は、実際にエラー処理を書くことですが、問題が発生したときにこのプログラムをクラッシュさせたいだけなので、`expect`使用しています。

### `println!`プレースホルダー

次に以下のコードについて説明します。

```rust
println!("You guessed: {guess}");
```

今回見るのは、`{}`です。これはプレースホルダーといって、値を所定の位置に保持する小さなカニのハサミみたいなものです。この括弧の中に変数を入れるとその変数の値を出力することができます。また以下のように書くこともできます。

```rust
let x = 5;
let y = 7;

println!("x = {} and y = {}", x, y);
// => x = 5 and y = 7
```

## 暗証番号の生成

次に、ユーザーが推測する秘密の番号を生成する必要があります。暗証番号は、1から100までの乱数を使用します。Rustの標準ライブラリにはまだ乱数機能が含まれていないので、[`rand`クレート](https://crates.io/crates/rand)を使用します。

### クレートを使用してより多くの機能を取得する

クレートはRustソースコードファイルのコレクションです。`rand`クレートはライブラリクレートであり、単独では実行できません。

`rand`を使用するには、クレートを依存関係として含める必要があります。

**Cargo.toml**

```toml
[dependencies]
rand = "0.8.3"
```

Cargoは、バージョン番号を記述するための標準である[セマンティック バージョニング（*SemVer*）](http://semver.org)を理解します。

ビルドすると次のような出力になります。

```shell
cargo build
    Updating crates.io index
  Downloaded ppv-lite86 v0.2.17
  Downloaded libc v0.2.138
  Downloaded getrandom v0.2.8
  Downloaded rand_chacha v0.3.1
  Downloaded rand_core v0.6.4
  Downloaded rand v0.8.5
  Downloaded cfg-if v1.0.0
  Downloaded 7 crates (794.8 KB) in 0.75s
   Compiling libc v0.2.138
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.17
   Compiling getrandom v0.2.8
   Compiling rand_core v0.6.4
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling guessing_game v0.1.0 (/Users/ittoku/Documents/learning-rust/projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 7.28s
```

外部依存関係を含めると、Cargoは依存関係が必要とするすべての最新バージョンをレジストリ（[Crates.io](https://crates.io/)）から取得します。Crates.ioは、Rustエコシステムの人々がオープンソースのRustプロジェクトを投稿して、他のユーザーが使用できるようにする場所です。

Cargoは依存関係をチェックするので、すでにダウンロードしてコンパイルされているクレートはビルドされません。

### Cargo.lockファイルによる再現可能なビルドの確保

Cargoには、他の誰かがコードをビルドするたびに、同じアーティファクトを再構築できるメカニズムがあります。Cargoは、特に指定しない限り、指定した依存関係のバージョンのみを使用します。

初めてプロジェクトをビルドするとき、Cargoは基準に適合する依存関係の全てのバージョンを見つけ出し、*Cargo.lock*ファイルに書き込みます。将来プロジェクトをビルドするとき、*Cargo.lock*ファイルが存在することを確認し、バージョンを再計算するのではなく、そこで指定されたバージョンを使用します。これにより再現可能なビルドが自動的に作成されます。

つまり、*Cargo.lock*ファイルは、明示的にアップグレードするまでプロジェクトはそのままになるということです。また再現可能なビルドは重要であるため、多くの場合、ソース管理にチェックインされます。

### クレートを更新して新しいバージョンを取得する

クレートを更新したい場合、以下のコマンドを使用します。このコマンドは、*Cargo.lock*を無視し、*Cargo.toml*の仕様に適合する全ての最新バージョンを見つけます。その後、*Cargo.lock*ファイルに書き込みます。

```shell
cargo update
```

クレートのマイナーバージョンを上げるには、*Cargo.toml*を次のように更新する必要があります。

```toml
[dependencies]
rand = "0.9.0"
```

### 乱数の生成

では、実際に`rand`を使用して推測する数値を生成しましょう。

```rust
use rand::Rng;

fn main() {
  let secret_number = rand::thread_rng().gen_range(1..=100);

  println!("The secret number is {secret_number}");
}
```

まず、`use rand::Rng`を追加します。`Rng`は乱数ジェネレーターが実装するメソッドを定義します。

次に、`rand::thread_rng`関数を呼び出して、特定の乱数ジェネレーター（現在の実行スレッドにローカルで、OSによってシードされるもの）を生成します。

次に、上記の乱数ジェネレーターに対して、`gen_range`を呼び出します。このメソッドは、`use rand::Rng`ステートメントでスコープに取り込んだ `Rng`トレイトで定義されています。このメソッドは範囲式を引数に取り、その範囲内の乱数を生成します。ここで使用する範囲式は`start..=end`という形式で、1から100までの数を要求するには`1..=100`を指定します。

以下のコマンドは、全ての依存関係によって提供されるドキュメントがローカルにビルドされ、ブラウザで開くことができます。

```shell
cargo doc --open
```

以下のコマンドを実行すると、以下のような出力になります。

```shell
cargo run
   Compiling guessing_game v0.1.0 (/Users/ittoku/Documents/learning-rust/projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 85
Please input your guess.
1234
You guessed: 1234
```

### 推測と暗証番号の比較

次に以下のコードについてみていきます。

```rust
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

まず、`use`文を追加して、標準ライブラリから` std::cmp::ordering`という型をスコープに取り込みます。`Ordering`型は別の`enum`で、`Less, Greater, Equal`というバリアントを持っています。

`cmp`メソッドは2つの値を比較するもので、比較できるものであれば何でも呼び出すことができます。ここでは、`secret_number`と`guess`を比較しています。そして、`use`ステートメントでスコープに取り込んだ順序列挙型の変種を返します。

マッチ式の例を見てみましょう。ユーザーが`50`で、秘密番号は`38`だとしましょう。コードで`50, 38`を比較すると、`50`は、`38`より大きいので、`Ordering::Greater`を返します。match式は上から下へ各アームのパターンをチェックしていきます。ここでは、`Less, Greater, Equal`の順番で見ていきます。例では、`Ordering::Greater`のところにマッチしているので、その下の`Ordering::Equal`は実行されません。

試しにビルドしてみましょう。以下のような出力になりました。

```shell
cargo run
   Compiling guessing_game v0.1.0 (/Users/ittoku/Documents/learning-rust/projects/guessing_game)
error[E0308]: mismatched types
   --> src/main.rs:22:21
    |
22  |     match guess.cmp(&secret_number) {
    |                 --- ^^^^^^^^^^^^^^ expected struct `String`, found integer
    |                 |
    |                 arguments to this function are incorrect
    |
    = note: expected reference `&String`
               found reference `&{integer}`
note: associated function defined here
   --> /Users/ittoku/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/cmp.rs:789:8
    |
789 |     fn cmp(&self, other: &Self) -> Ordering;
    |        ^^^

For more information about this error, try `rustc --explain E0308`.
error: could not compile `guessing_game` due to previous error
```

このエラーのは、型が不一致であることを述べています。Rustは強力な静的型システムを持っています。しかし、型推論も備えています。`let mut guess = String::new()`と書いたているので型推論しなかったのです！一方、`secret_number`は数値型です。Rustの数値型には、1から100までの値を持つものがあります。32ビット数値の`i32`、符号なし32ビット数値の`u32`、64ビット数値の`i64`、そしてその他があります。特に指定しない限り、Rustの数値型のデフォルトは`i32`となります。

つまりエラーの原因は、Rustが文字列と数値の型を比較できないためです。

では、プログラムが入力として読み込んだ文字列を実数型に変換して、秘密の番号と数値的に比較できるようにします。

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

`guess`という名前の変数を作成します。でも待ってください、このプログラムにはすでに`guess`という名前の変数があるのではありませんか？しかし、Rust は便利なことに、`guess`の前の値を新しい値でシャドーイングすることができます。シャドウイングを使えば、たとえば`guess_str`のようなユニークな変数を2つ作る必要がなく、`guess`という変数名を再利用することができます。

そして変数を、`guess.trim().parse()`に結びつけます。この式では、`trim`メソッドで、文字列の先頭と末尾の空白を除去や、エンターを押した時の改行文字を除去し、`parse`メソッドで、文字列を別の方に変換します。この場合、変数名の横にコロンと型の名前を書くことで変数の型にアノテーションをつけることをRustに伝えています。今回は文字列型から数値型（`u32`）に変換しています。

`parse`メソッドは、数字に変換できる文字に対してのみ動作するため、簡単にエラーを起こります。たとえば、文字列に「A👍%」が含まれていた場合、数値に変えることができず、エラーが起こります。`parse`メソッドは`read_line`メソッドと同じように`Result`型を返します。この`Result`を`expect`メソッドを使うことによって、文字列から数値を作成できなかった時にゲームをクラッシュさせ、私たちが与えたメッセージを表示します。

### ループで複数の推測を許可する

`loop`キーワードは無限ループを作成します。

```rust
loop {
  println!("Please input your guess")

  // --snip--
}
```

これで、1度入力すると終わってしまうプログラムから、何度入力しても終わらないプログラムに進化しました！終わらせるには、エラーを吐かせるか、`CTRL-C`を押すとプログラムが終了します。

### 正しい推測の後に止める

`break`ステートメントを追加して、ユーザーが推測を的中させたらゲームを終了させるようにプログラムしましょう。

```rust
match guess.cmp(&secret_number) {
    // --snip--
    Ordering::Equal => {
        println!("You win!");
        break;
    }
}
```

これでプログラムから抜けることができるようになりました！

### 無効な入力の処理

ユーザーが数字以外を入力した時にプログラムをクラッシュさせるのではなく、ユーザーが推測を続けられるようにしてみましょう。

```rust
let guess: u32 = match guess.trim().parse() {
  Ok(num) => num,
  Err(_) => continue,
};
```

これでエラー時のクラッシュからエラー処理に移行しています。`parse`メソッドは`Result`型を返すので、それを`match`式で処理を行っています。

詳しく書くと、`parse()`の`Result`型の返り値が`Ok`だった場合、ユーザーが入力した文字列の数値が代入され、`Err`だった場合、何も代入されません。

### 完成コード

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

### 概要

推測ゲームを正しく構築できました。おめでとう！

このプロジェクトでは、Rustの概念（`let, match, function, crait`など）を紹介し実践していきました。次のいくつかの章では、これらの概念について詳しく学習します。第3章では、変数、データ型、関数など、第4章では、所有権について、第5章では、構造体とメソッドの構文、第6章では列挙型の仕組みを説明します。
