# テストの書き方

> Ref: https://doc.rust-jp.rs/book-ja/ch11-01-writing-tests.html

テストは、コードが想定された方法で機能していることを実証するRustの関数です。
テスト関数の本体は、以下の3つの典型的な動作を行います。

1. 必要なデータや状態をセットアップする
2. テスト対象コードを走らせる
3. 結果が想定通りであることを断定（アサーション）する

Rustが、特にこれらの動作を行うテストを書くために用意している機能を見ていきましょう。
これには、`test`属性、マクロ、`should_panic`属性が含まれます。

## テスト関数の構成

最も単純には、Rustにおけるテストは`test`属性で注釈された関数のことです。
属性とは、Rustコードの部品に関するメタデータです。メタデータとは構造体で使用した`derive`属性のことです。
関数をテスト関数に変えるには、`fn`の前に`#[test]`を加えます。
`cargo test`コマンドでテストを実行したら、コンパイラは`test`属性で注釈された関数を走らせるテスト用バイナリをビルドし、各テスト関数が通過したか失敗したか報告します。

新しいライブラリプロジェクトをCargoで作ると、テスト関数付きのテストモジュールが自動的に生成されます。
このモジュールのおかげで、新しいプロジェクトを始めるたびにテスト関数の正しい構造や文法をいちいち検索しなくて済みます。
ここには好きな数だけテスト関数やテストモジュールを追加すれば良いというわけです。

では実際にテストを書くために、以下のプロジェクトを作成しましょう。

```bash
cargo new testing --lib
```

プロジェクトを作成したら、`src/lib.rs`の中身は以下のようになっているはずです。

```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

では上記のコードで何が行われているか見ていきましょう。
`fn`行の`#[test]`は属性で、テストを実行するとこの関数をテストとして扱います。
さらに、`tests`モジュール内にはテスト関数以外の関数を入れ、一般的なシナリオをセットアップしたり、共有の処理を行う手助けをしたりできるので、`#[test]`属性でどの関数がテストを行うかを示す必要があるのです。

関数本体は、`assert_eq!`マクロを使用して、2+2が4に等しいことをアサーションしています。
このアサーションは、典型的なテストのフォーマット例をなしているわけです。
走らせてこのテストが通るかどうか確認してみましょう。

```bash
cargo test

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests testing

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Cargoがテストをコンパイルし実行しました。出力されたテキストの内容は次のとおりです。

- `running 1 test`、走らせたテストの総数
- `test tests::it_works ... ok`、it_worksテストが成功したことを示しています
- `test result: ok.`、全テストが通ったことを意味します
  - `0 passed;`、テストが通った数
  - `0 failed;`、テストが失敗した数
  - `0 ignored;`、テストを無視した数
  - `0 measured;`、ベンチマークテスト用
  - `0 filtered out;`、フィルタをかけた数
  - `finished in 0.00s`、テストを実行するのにかかった時間
- `Doc-tests testing`、ドキュメンテーションテストの結果用

まだドキュメンテーションテストは何もないものの、コンパイラはAPIドキュメントに現れるどんなコードレイもコンパイルできます。
この機能により、ドキュメントとコードを同期することができます。ドキュメンテーションテストについては第14章で接絵名します。

テストの名前を変更してどうテスト出力が変わるかみてみましょう。

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

ではテストを実行して出力を確認してみましょう。`it_works`から`exploration`に変わっています。

```bash
cargo test

running 1 test
test tests::exploration ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests testing

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

次は失敗するテストを書いてみましょう。テスト関数内の何かがパニックするとテストは失敗します。
各テストは新規スレッドで実行され、メインスレッドがテストスレッドが死んだと確認した場合、テストは失敗となります。
`panic!`マクロを呼び出して、テストを失敗させるコードを書いてみましょう。

```rust
#[cfg(test)]
mod tests {
    // ...
    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
```

`cargo test`でテストを走らせると、`another`は失敗したと表示されます。

```bash
cargo test

running 2 tests
test tests::exploration ... ok
test tests::another ... FAILED

failures:

---- tests::another stdout ----
thread 'tests::another' panicked at 'Make this test fail', src/lib.rs:17:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`test tests::another ... FAILED`と表示され、その下に、なぜテストが失敗したか具体的な理由も表示しています。
今回の場合、`Make this test fail`のため失敗し、`src/lib.rs:17:9`で起きたと書かれています。
この表示方法は、テストがたくさんあり、失敗したテストの詳細がたくさん表示される時に有用になります。
これを失敗したテストの名前を使用して、そのテストのみを実行することで簡単にデバッグすることができます。

テスト結果は`FAILED`で、1つのテストが通り、1つが失敗したと書かれています。

次はテストを行う際に有用になる`panic!`以外のマクロについてみていきましょう。

## `assert!`マクロ

標準ライブラリで提供されている`assert!`マクロは、テスト内の何らかの条件が`true`と評価されることを確かめたい時に使用します。
`assert!`マクロには、論理値に評価される引数を与えます。
その値が`true`なら`assert!`は何もせずテストが通り、`false`なら`assert!`マクロは`panic!`マクロを呼び出しテストを失敗させます。
`assert!`マクロを使用することで、コードが意図した通りに機能していることを確認する助けになるわけです。

ではテストを書くために、第5章で書いた`Rectangle`構造体と`can_hold`メソッドを定義しましょう。

```rust
#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle ) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

`can_hold`メソッドは論理値を返すので、`assert!`マクロは完璧なユースケースとなります。
ではテストを書いてみましょう。

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}
```

`use super::*`は、第7章で説明した通常の公開ルールに従う普通のモジュールです。
`tests`モジュールは内部モジュールなので、外部モジュール内のテストは以下にあるコードを内部モジュールのスコープに持っていく必要があります。
ここではglobを使用して、外部モジュールで定義したものすべてがこの`tests`モジュールでも使用可能になるようにしています。

このテストは通ります。次は別のテストを追加しましょう。小さい長方形は、大きな長方形を保持できないことをアサーションします。

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
```

`can_hold`関数の正しい結果は`false`なので、その結果を`assert!`マクロに渡す前に反転させる必要があります。
結果として`can_hold`が`false`を返せばテストは通ります。

```bash
cargo test

running 3 tests
test tests::exploration ... ok
test tests::larger_can_hold_smaller ... ok
test tests::smaller_cannot_hold_larger ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests testing

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

次は、コードにバグを仕込んだらテスト結果がどうなるか確認してみましょう。
幅を比較する記号を置き換えてバグを仕込みましょう。

```rust
impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle ) -> bool {
        self.width < other.width && self.height > other.height
    }
}
```

テストを実行すると、以下のような出力になります。

```bash
cargo test

running 3 tests
test tests::exploration ... ok
test tests::smaller_cannot_hold_larger ... ok
test tests::larger_can_hold_smaller ... FAILED

failures:

---- tests::larger_can_hold_smaller stdout ----
thread 'tests::larger_can_hold_smaller' panicked at 'assertion failed: larger.can_hold(&smaller)', src/lib.rs:43:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::larger_can_hold_smaller

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

テストによりバグが捕捉されました。
`larger.width`が8、`smaller.width`が5なので、`can_hold`内の幅の比較が今は`false`を返すようになったのです。

## `assert_eq!, assert_ne!`マクロ

機能をテストする一般的な方法に、コードが返す値と期待する値を比較して等しいどうかか確かめる方法があります。
これは、`assert`マクロで`==`演算子を使用した式を渡すことで行うこともできます。
しかし標準ライブラリには1組のマクロ(`assert_eq!, assert_ne!`)が提供されており、こちらの方がよく使われます。
これらのマクロはそれぞれ、2つの引数を比較し等しいか等しくないか確かめます。
また、アサーションが失敗時に2つの値の出力を行うため、テストが失敗した原因を確認しやすくなります。

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2))
    }
}
```

実行結果は次のとおりです。

```bash
cargo test

running 4 tests
test tests::it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests testing

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

`assert_eq!`マクロに与えた第1引数の4は、`add_two(2)`の呼び出し結果と等しいです。

次はコードにバグを仕込んで、`assert_eq!`のテストが失敗した時、どのような見た目になるか確認してみましょう。

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}
```

テストを再度実行します。

```bash
cargo test

running 4 tests
test tests::it_adds_two ... FAILED

failures:

---- tests::it_adds_two stdout ----
thread 'tests::it_adds_two' panicked at 'assertion failed: `(left == right)`
  left: `4`,
 right: `5`', src/lib.rs:66:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::it_adds_two

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

テストがバグを補足しました！`it_adds_two`のテストは失敗し、なぜ失敗したのかが書かれています。

2つの値が等しいとアサーションを行う関数の引数を`expected, actual`と呼び、引数を指定する順序が問題になる言語やテストフレームワークもあることに注意してください。
Rustでは、`left, right`と呼ばれ、期待する値とテスト下のコードが生成する値を指定する順序は問題になりません。
今回のテストのアサーションを`assert_eq!(add_two(2), 4)`と書くこともできます。

`assert_ne!`マクロは、与えられた2つの値が等しくなければ通り、ひとしければ失敗します。
このマクロは、値が何になるのか確信が持てないが、コードが意図した通りに動いていれば、確実にこの値になならないとわかっている場合に最も有用になります。

内部的に`assert_eq!, assert_ne!`マクロは、それぞれ`==, !=`演算子を使用しています。
アサーションが失敗すると、これらのマクロは引数をデバッグフォーマットを使用してプリントするので、比較対象の値は`PartialEq, Debug`トレイトを実装していなければなりません。
全ての組み込み型と、ほぼ全ての標準ライブラリの型はこれらのトレイトを実装しています。
自身で定義した構造体、列挙型をアサーションするには、`PartialEq`を実装する必要があります。
実装は定義の上に`#[derive(PartialEq, Debug)]`という注釈を追加するだけで済みます。

## カスタム失敗メッセージ

`assert!, assert_eq!, assert_ne!`の追加引数に、失敗メッセージと共にカスタムメッセージを追加することもできます。
これらのアサーションの必須引数の後に指定された引数は全て`format!`マクロに渡されるので、`{}`プレースホルダーを含むフォーマット文字列とこのプレースホルダーに置き換えられる値を渡すことができます。
カスタムメッセージは、アサーションがどのような意味を持つかドキュメント化するのに役立ちます。
もしテストが失敗した時、コードにどんな問題があるのかをよりしっかり把握するのが目的です。

```rust
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}
```

では`greeting`にバグを仕込み、このテストの失敗がどのようになるのか確認してみましょう。

```rust
pub fn greeting(name: &str) -> String {
    String::from("Hello")
}
```

テストを実行すると以下の出力になります。

```bash
cargo test

...

---- tests::greeting_contains_name stdout ----
thread 'tests::greeting_contains_name' panicked at 'assertion failed: result.contains(\"Carol\")', src/lib.rs:77:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

...
```

この出力結果は、どの行にアサーションがあるかを示すのみとなっています。
もし失敗メッセージが`greeting`関数から得た値を出力していればより有用になります。

```rust
#[cfg(tesg)]
mod tests {
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}
```

このように書くことで、より有益なエラーメッセージを得ることができます。

```bash
cargo test

...

---- tests::greeting_contains_name stdout ----
thread 'tests::greeting_contains_name' panicked at 'Greeting did not contain name, value was `Hello`', src/lib.rs:77:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

...
```

## `should_panic`でパニック

期待する正しい値をコードが返すことを確認することに加えて、想定通りにコードがエラー状態を扱っていることを確認するのも重要なことです。
第9章で生成した`Guess`型を例として、`Guess`を使用する他のコードは、`Guess`のインスタンスは1から100の範囲の値しか含まないという保証に依存しています。
その範囲外の値で`Guess`インスタンスを生成しようとするとパニックすることを確認するテストを書くことができます。

これはテスト関数に`should_panic`という別の属性を追加することで達成できます。
この属性は関数内のコードがパニックしたらテストを通過させ、パニックしなければテストを失敗させます。

```rust
pub struct Guess {
    pub value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

このテストは通ります！では値が100より大きい時にパニックするという女権を取り除くことでコードにバグを導入しましょう

```rust
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess { value }
    }
}
```

テストの結果は次のとおりです。

```bash
cargo test

test tests::greater_than_100 - should panic ... FAILED

failures:

---- tests::greater_than_100 stdout ----
note: test did not panic as expected

failures:
    tests::greater_than_100

...
```

この場合、それほど役に立つメッセージは得られませんが、テスト関数に目を向ければ、`#[should_panic]`で注釈されていることがわかります。
得られた失敗は、テスト関数のコードがパニックを引き起こさなかったことを意味します。

`should_panic`を使用するテストは不正確なこともあります。なぜならコードが何らかのパニックを起こしたことしか示さないからです。
`should_panic`のテストは、起きると想定していたもの以外の理由でテストがパニックしても通ってしまうのです。
`should_panic`のテストの正確を期すために、`should_panic`属性に`expected`引数を追加することもできます。
このテストハーネスは、失敗メッセージに与えられたテキストが含まれていることを確かめてくれます。

```rust
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

`should_panic`属性の`expected`引数においた値が`Guess::new`関数がパニックしたメッセージの一部となっているのでテストは通ります。

`expected`メッセージありの`should_panic`テストが失敗すると出力はこのようになります。

```bash
cargo test

...

test tests::greater_than_100 - should panic ... FAILED

failures:

---- tests::greater_than_100 stdout ----
thread 'tests::greater_than_100' panicked at 'Guess value must be less than or equal to 1, got 200', src/lib.rs:35:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: panic did not contain expected string
      panic message: `"Guess value must be less than or equal to 1, got 200"`,
 expected substring: `"Guess value must be less than or equal to 100"`

failures:
    tests::greater_than_100

...
```

## `Result<T, E>`をテストで使う

これまでは失敗するとパニックするようなテストを書いてきましたが、`Result<T, E>`を使うようなテストも書くことができます。

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

`Result<T, E>`を返すようなテストを書くと、`?`演算子をテストの中で使えるようになります。
これはテスト内で何らかの工程が`Err`を返した時に失敗すべきテストを書くのに便利です。

また`Result<T, E>`を使うテストに`#[should_panic]`注釈は使用することができないことに注意してください。
