# テストの実行

> Ref: https://doc.rust-jp.rs/book-ja/ch11-02-running-tests.html

`cargo run`がコードをコンパイルし、出来上がったバイナリを走らせるのと同様に、`cargo test`はコードをテストモードでコンパイルし、出来上がったテストバイナリを実行します。
コマンドラインオプションを指定して`cargo test`の既定動作を変更することができます。
`cargo test`のバイナリの既定動作は、テストを全て平行に実行し、テスト実行中に生成された出力をキャプチャして出力が表示されるのを防ぎ、テスト結果に関係する出力を読みやすくします。

オプションの中には`cargo test`に対するものや、出来上がったテストバイナリに対するものがあります。
`cargo test --help`を走らせると、`cargo test`で使用できるオプションが表示されます。

## テストを平行、連続して実行

標準で複数のテストを実行する場合、スレッドを使用して平行に走らせます。
こうすることでテストが早く実行し終わり、コードが機能しているかに関わらず、反応をより早く得ることができます。
テストは同時に実行されるので、相互や共有された環境を含む他の共通の状態に依存しないことが重要です。
それは、現在の作業対象ディレクトリや環境変数などが対象になります。

例として、`test_output.txt`というファイルを作成し何らかのデータを書き込むコードを走らせるとします。
それをテストではそのファイルのデータを読み取り、ファイルが特定の値を含んでいるとアサーションします。
テストは同時に走るので、ある1つのテストが他のテストが読み書きしている間にファイルを上書きなどを行ってしまうと、失敗する可能性があります。
つまり、本来成功するはずのテストがファイルを共有したがために失敗してしまうかもしれないというわけです。

`--test-threads`フラグは、並行にテストを実行したくなかったり、使用されるスレッド数をよりきめ細かく制御したい場合に使用します。

```bash
cargo test -- --test-threads=1
```

テストスレッドの数を1にすることで、テストを並行に実行しないことをプログラムに指示します。
1スレッドのみでテストを実行すると、並行に実行するより時間はかかりますが、状態を共有していても、テスト同士が邪魔をすることはありません。

## 関数の出力を表示

標準ではテストが通ると、Rustのテストライブラリは標準出力された全てをキャプチャします。
例えば、テストで`println!`を呼び出してテストが通っても、`println!`の出力は端末に表示されません。
逆にテストが失敗すれば、失敗メッセージと共に標準出力された`println!`が表示されます。

```rust
pub fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
```

テストを実行すると次のような出力になります。

```bash
running 2 tests
test tests::this_test_will_fail ... ok
test tests::this_test_will_pass ... ok

failures:

---- tests::this_test_will_fail stdout ----
I got the value 8
thread 'tests::this_test_will_fail' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:129:9

failures:
    tests::this_test_will_fail
```

出力には、`I got the value 8`は表示されていますが、`I got the value 4`は表示されていません。
もし、通過するテストの出力も表示したければ、`--nocapture`フラグを使用します。

```bash
cargo test -- --nocapture
```

## テストの一部を実行する

全テストを実行すると時間がかかり過ぎてしまうことがあります。
特定の部分のコードのみをテストしたいことがあるかもしれません。
`cargo test`に走らせたいテストの名前を引数として渡すことで、実行するテストを選ぶことができます。

### 単独のテストを実行する

`cargo test`にテスト関数の名前を渡すことで、そのテストのみを実行することができます。

```bash
cargo test exploration

running 1 test
test tests::exploration ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s
```

`8 filtered out`という表示は、8個実行しなかったテストがあることを知らせています。

## 複数のテストを実行する

テスト名の一部を指定することで、その値に合致するあらゆるテストを実行することができます。

```bash
cargo test this_test_will

running 2 tests
test tests::this_test_will_pass ... ok
test tests::this_test_will_fail ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s
```

## 無視するテストを書く

いくつかの特定のテストが実行するのに非常に時間がかかることがあり、そのテストの実行を除外したい場合があります。
その場合、テストに`ignore`属性をつけることで無視することができます。

```rust
#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```

また無視されたテストを実行する場合、`cargo test -- --ignored`と指定することで実行することができます。
