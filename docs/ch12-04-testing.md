# テスト駆動開発

> Ref: https://doc.rust-jp.rs/book-ja/ch12-04-testing-the-librarys-functionality.html

前節でロジックを`src/lib.rs`に、引数処理とエラー処理を`src/main.rs`に分けたので、テストを書くのが容易になりました。
色々な引数で関数を直接呼び出し、コマンドラインからバイナリを呼び出す必要なく戻り値を確認できます。

この説では、テスト駆動開発(TDD)過程を活用して`minigrep`プログラムに検索ロジックを追加します。
このソフトウェア開発テクニックは、以下の手順に従います。

1. 失敗するテストを書き、想定通りの理由で失敗することを確認する
2. コードを追加、変更を行い、新しいテストを通過するようにする
3. 追加、変更したコードをリファクタし、テストが通るか確認する。
4. 手順1から繰り返す

この過程は、ソフトウェアを書く方法の1つですが、TDDによりコードデザインも駆動することができます。
テストを通過させるコードを書く前にテストを書くことで、過程を通して高いテストカバー率を保つ助けとなります。

では実際にクエリ文字列の検索を行う機能の実装をテスト駆動し、クエリに合致する行のリストを作成します。
この機能を`serch`という関数に追加しましょう。

## 失敗するテストを書く

まずは必要のなくなった`plintln!`を`main.rs, lib.rs`から削除しましょう。
そして`lib.rs`で、`test`モジュールを追加します。その中に`search`関数のテストを書きます。

```rust
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
rust:
safe, fast, productive.
pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
```

このテストは、"duct"という文字列を検索します。
検索対象の文字列は3行で、その中に"duct"を含んでいます。
`search`関数から返る値が想定している行だけを含むことをアサーションします。

今テストを走らせても、`search`関数を作成していないので失敗どころかコンパイルできません。
なので、からのベクターを常に返す`search`関数の定義を追加することで、コンパイルはできるようになります。しかしテストは失敗します。

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}
```

`search`関数はの引数は文字列スライスを受け取るので、ライフタイムの`'a`を使用する必要があります。

コンパイラに`search`関数に返されるデータは`contents`引数で渡されているデータと同期間生きることを教えています。
これは重要なことで、スライスに参照されるデータは参照が有効である必要があるのです。
コンパイラが`contents`ではなく`query`の文字列スライスを生成すると想定すると、安全性チェックを間違って行うことになります。

ライフタイム注釈なしでコンパイルすると以下のようなエラーが出ます。

```bash
cargo test
   Compiling minigrep v0.1.0 (/Users/ittoku/Documents/learning/rust/projects/minigrep)
error[E0106]: missing lifetime specifier
  --> src/lib.rs:34:52
   |
34 | pub fn search(query: &str, contents: & str) -> Vec<& str> {
   |                      ----            -----         ^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `query` or `contents`
help: consider introducing a named lifetime parameter
   |
34 | pub fn search<'a>(query: &'a str, contents: &'a  str) -> Vec<&'a  str> {
   |              ++++         ++                 ++               ++

For more information about this error, try `rustc --explain E0106`.
error: could not compile `minigrep` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `minigrep` due to previous error
```

コンパイラには2つの引数のどちらが必要なのか教える必要があります。

他のプログラミング言語では、シグネチャで引数と戻り値を関連づける必要はありません。

コードを元に戻してテストを実行してみましょう。

```bash
cargo test
   Compiling minigrep v0.1.0 (/Users/ittoku/Documents/learning/rust/projects/minigrep)

running 1 test
test test::one_result ... FAILED

failures:

---- test::one_result stdout ----
thread 'test::one_result' panicked at 'assertion failed: `(left == right)`
  left: `["safe, fast, productive."]`,
 right: `[]`', src/lib.rs:50:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    test::one_result

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
```

テストは想定通り失敗しています。次はテストが通るようにしましょう。

## テストを通過するコードを書く

からのベクターを常に返しているためテストは失敗します。
これを修正するために`search`を実装するには以下の手順を踏みます。

- 文字列を各行ごとに繰り返す
- 行にクエリ文字列が含まれるか確認する
- あれば、返す値のリストに追加する
- なければ何もしない
- 一致する結果のリストを返す

### `lines`メソッドを繰り返す

Rustには`lines`という文字列を行ごとに繰り返すメソッドがあります。
このメソッドを`search`関数内で使用してみましょう。

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        // do something with line
    }
}
```

`lines`メソッドはイテレータを返します。イテレータについては第13章で話します。
ここでは、イテレータに`for`ループを使用してコレクションの各要素に繰り返し処理を行います。

### クエリを求めて検索する

次に現在の行がクエリ文字列を含んでいるか確認します。
ここでも`contains`という文字列を検索してくれるメソッドがあります。
このメソッドを`search`関数内で使用してみましょう。

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        if line.contains(query) {
            // do something with line
        }
    }
}
```

### 合致した行を保存する

次はクエリ文字列を含む行を保存する必要があります。
`for`ループの前に可変なベクターを生成し、`push`メソッドを呼び出して`line`をベクターに保存します。

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

これでテストが通るはずです。実行してみましょう。

```bash
cargo test

running 1 test
test test::one_result ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

テストが通ったことで、この機能を維持しながら`search`関数をリファクタすることも考えることができます。
今の`search`関数のコードは悪くないですが、イテレータの有用な機能の一部をうまく活用していません。
この例は第13章で再度触れ、イテレータについて詳しくみていきます。

### `run`関数で`search`関数を使用する

`search`関数のテストができたので、`run`関数から`search`を呼び出してみましょう。

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}
```

これでプログラム全体が動くはずです。試してみましょう。

```bash
cargo run frog poem.txt

Searching for frog
In file poem.txt
How public, like a frog
```

次は複数行にマッチするであろう言葉で試してみましょう。

```bash
cargo run body poem.txt

Searching for body
In file poem.txt
I'm nobody! Who are you?
Are you nobody, too?
How dreary to be somebody!
```

最後に、マッチしない言葉で試してみましょう。

```bash
cargo run monomorphization poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep monomorphization poem.txt`
Searching for monomorphization
In file poem.txt
```

できました！`grep`の独自のミニバージョンを構築し、アプリケーションを構造化する方法を学びました。
また、ファイル入出力、ライフタイム、テスト、コマンドライン引数解析について学ぶことができました。

このプロジェクトをまとめ上げるために、環境変数を扱う方法と標準エラー出力する方法を少しみていきます。
これらはコマンドラインプログラムを書く際に有用なものです。
