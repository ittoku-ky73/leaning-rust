# 環境変数を取り扱う

> Ref: https://doc.rust-jp.rs/book-ja/ch12-05-working-with-environment-variables.html

`minigrep`に環境変数を導入して、ユーザーが大文字小文字無視の検索ができるように選べる機能を追加します。
この機能はコマンドラインオプションで指定することでもできますが、環境変数を用いることで、オプションなしでその機能を行うことができるようになります。

## 大文字小文字を区別しない

環境変数がオンの場合に呼び出す`search_case_insensitive`関数を新しく追加します。
テスト駆動開発で実装するので、初めは失敗するテストを書きます。

```rust
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
```

古いテストの`contents`も少し変更を加えています。
"Duct tape."を最後の行に追加することで、大文字小文字を区別することを確かめています。

大文字小文字を区別しない新しいテストは、クエリに"rUsT"を使用しています。
これは、大文字小文字を区別しない検索をしないと一致しないようなクエリとなっています。

### search_case_insensitive関数

この関数は、`search`関数とほぼ同じです。違いは、`query`と`line`を小文字化していることです。

```rust
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```

まずは`query`を小文字化します。こうすることで`rust, RUST, Rust, RuSt`などであっても`rust`となります。
あとは`query`は`to_lowercase`を呼び出すことで、文字列スライスではなく`String`になります。
そのため`contains`メソッドの引数には`query`の参照を渡す必要があります。

次に、`line`が`query`を含むか確かめる前に`to_lowercase`を呼び出し小文字化します。
大文字小文字を区別しない検索がしたければ、クエリも原文も全て小文字にする必要があります。

ではテストを試してみましょう。成功するはずです。

```bash
cargo test

running 2 tests
test test::case_insensitive ... ok
test test::case_sensitive ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

テストが通ったら、`run`関数に`search_case_insensitive`関数呼び出しましょう。
まずは大文字小文字を切り替えられるように`Config`構造体に設定オプションを追加します。

```rust
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
} 
```

次に`case_sensitive`フィールドの値を確認し、`search, search_case_insensitive`関数のどちらを呼ぶか決定するようにします。

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
```

最後に環境変数を確認する処理を加えます。環境変数を扱う関数は、`std::env`です。
では`std::env`モジュールから`var`関数を使用して`CASE_INSENSITIVE`という環境変数のチェックを行います。

```rust
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}
```

ここでは`case_sensitive`という新しい変数を生成しています。
`env::var`関数を呼び出し、`CASE_INSENSITIVE`環境変数の名前を渡しています。
この関数は、環境変数がセットされていたら`Ok`を返し、されていなければ`Err`を返します。

`Result`の`is_err`メソッドを使用して、環境変数がセットされているか確認しています。
`CASE_INSENSITIVE`環境変数がセットされていれば、falseを返し、プログラムは大文字小文字を区別しない検索を実行します。
環境変数の値はなんでもよく、セットされているかどうかだけを確認します。
そのため`unwrap, expect`メソッドではなく`is_err`でチェックを行っています。

`case_sensitive`変数の値は`Config`インスタンスに渡しているので、`search, search_case_insensitive`のどちらを呼び出すのか決定することができます。

まずは試してみましょう。最初は環境変数をセットせずにプログラムを実行します。

```bash
cargo run to poem.txt

Are you nobody, too?
How dreary to be somebody!
```

機能しています。次は`CASE_INSENSITIVE`を1にして、プログラムを実行してみましょう。

```bash
CASE_INSENSITIVE=1 cargo run to poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

いいですね！"To"を含む行も検索することができました。
これで`minigrep`は、環境変数によって制御できる大文字小文字を区別しない検索ができるようになりました。
そしてコマンドライン引数、環境変数を使ってオプションを管理する方法も知りました。

引数と環境変数で同じ設定を行うことができるプログラムもあります。
その場合、プログラムはどちらが優先されるか決定します。
コマンドライン引数と環境変数のどちらの優先度が高くなるかというとコマンドライン引数です。

`std::env`モジュールは、環境変数を扱うもっと多くの有用な機能を持っています。
ドキュメントを読んでみるのもいいかもしれませんね。
