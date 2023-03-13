# 入出力プログラムの改善

> Ref: https://doc.rust-jp.rs/book-ja/ch13-03-improving-our-io-project.html

イテレータに関する新しい知識があれば、イテレータを使用してコードのいろいろな場所をより明確で簡潔に書くことができます。
イテレータを活用して、第12章の入出力プロジェクトの`Config::new, search`関数を改善していきましょう。

## `clone`を取り除く

第12章の`Config::new`では、
スライスに添え字アクセスして値をクローンすることで、
`Config`構造体の値を所有させつつ、
`String`値のスライスをとり、
`Config`構造体のインスタンスを作成するコードを作成しました。

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

この実装の際、非効率な`clone`の呼び出しを将来的に除去すると述べました。ここでそれを行います。

引数`args`に`String`要素のスライスがあるために`clone`が必要だったのですが、`new`関数は`args`を所有していません。
`Config`インスタンスの所有権を返すには、`Config`インスタンスがその値を所有できるように、`query, filename`フィールドから値をクローンしなければいけませんでした。

イテレータの知識があれば、`new`関数でスライスを借用する代わりに、引数としてイテレータの所有権を奪うように変更することができます。
スライスの長さを確認し、特定の場所に添え字アクセスするコードの代わりにイテレータの機能を使います。
これによりイテレータは値にアクセスするので、`Config::new`関数が明確化します。

`Config::new`がイテレータの所有権を奪い、借用する添え字アクセス処理を終えたら、`clone`でメモリを確保するのではなく、イテレータから`String`値を`Config`にムーブできます。

## 返却されるイテレータを直接使う

現在の入出力プロジェクトの`src.main.rs`は以下のようになっています。

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
}
```

これを以下のコードに変更します。まだコンパイルできません。

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
}
```

`env::args`関数はイテレータを返します！
イテレータの値をベクターに集結させ、それからスライスを`Config::new`に渡すのではなく、`env::args`から帰ってくるイテレータの所有権を直接`Config::new`に渡しています。

次に、`Config::new`の定義を更新します。
`src/lib.rs`の`Config::new`のシグネチャを以下のコードにしましょう。まだコンパイルはできません。

```rust
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // --snip--
```

`env::args`関数の標準ライブラリドキュメントは、自身が返すイテレータの型は`std::env::Args`であると書かれています。
これで引数`args`の型は、`&[String]`から`std::env::Args`になりました。
`args`の所有権を奪い、繰り返しを行うことで`args`を可変化する予定なので、`mut`キーワードを追記しています。

## 添え字の代わりに`Iterator`トレイトメソッドを使用する

次に`Config::new`の本体を修正しましょう。
標準ライブラリのドキュメントは、`std::env::Args`が`Iterator`トレイトを実装していると書かれているので、`next`メソッドを呼び出すことができます。
以下のコードは、`next`メソッドを使用するように更新したものです。

```rust
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}
```

`env::args`の戻り値の1番目の値はプログラム名です。この関数で必要なのは次からの値なので無視します。
2番目に`next`を呼び出して`Config`の`query`フィールドの置きたい値を得ます。
`next`が`Some`を返したら値を抜き出し、`None`であれば`Err`値で早期リターンします。
`filename`値に対しても同じことを行います。

## イテレータアダプタでコードを明確にする

入出力プロジェクトの`search`関数でも、イテレータを活用することができます。

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

イテレータアダプタメソッドを使用すると、このコードを簡潔に書くことができます。
そうすれば`results`ベクターをなくすことも可能です。
関数型プログラミングスタイルは、可変な状態の量を最小化することを好み、コードを明瞭化します。
可変な状態を除去すると、検索を同時並行に行うという将来的な改善も可能になります。
なぜなら`results`ベクターへの同時アクセスを管理する必要がなくなるからです。

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
     contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

`search`関数の目的は、`query`を含む`contents`の行全てを返すことです。
これは`filter`に酷似しており、`line.contains(query)`で実装が可能です。
それから合致した行を別のベクターに`collect`で集結させます。
この変更は`search_case_insensitive`関数でも使用することができます。

```rust
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
```

次の論理的な疑問は、自身のコードでどちらのスタイルを選び方と理由です。
スタイルは前の実装と、イテレータを使用する現在の実装です。
多くのRustプログラマはイテレータスタイルを好みます。
初めは難しいですが、いろいろなイテレータアダプタと使用感覚を一度掴めば、イテレータの方が理解しやすいこともあります。
いろいろなループを少しずつ弄んだり、新しいベクターを構築する代わりに、コードはループの高難度の目的に集中できるのです。
これはありふれたコードの一部を抽象化するので、イテレータの各要素が通過しなければならない古い条件など、このコードの独特の概念を理解しやすくなります。

ですが本当に2つの実装は等価なのでしょうか？
直感的な仮説は、より低レベルのループの方がより高速ということかもしれません。
次節ではパフォーマンスについて見ていきます。
