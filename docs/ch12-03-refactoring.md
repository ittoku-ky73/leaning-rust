# リファクタリング

> Ref: https://doc.rust-jp.rs/book-ja/ch12-03-improving-error-handling-and-modularity.html

プログラムを改善するために、構造と可能性のあるエラーに対処する方法に関連する4つの問題を修正していきます。

1つ目は、`main`関数が2つの役割を受け持っていることです。
この問題を解決しないと、プログラムが巨大化したときに以下のような問題が発生します。

- 動作確認が困難になる
- テストの実行が難しくなる
- 機能を壊さずに変更することが難しくなる。

機能を小分けにすることで、各関数が1つの仕事のみに責任を持つようにしましょう。

2つ目は、変数名の解決です。
`query, filename`はプログラムの設定用変数ですが、`f, contents`といった変数は、プログラムのロジックを担っています。
設定用変数を1つの構造に押し込め、目的を明瞭化するようにしましょう。

3つ目は、ファイルを開くのが失敗したときに、`expect`のエラーメッセージが正しく表示されないことです。
ファイルを開く行為は、ファイルが存在しない以外にも色々な理由で失敗することがあります。
それぞれの理由で失敗したときに表示するエラーメッセージを分けて出力するようにしましょう。

4つ目は、ユーザーが引数を渡さなかったときのエラー処理です。
現在は範囲外アクセスというエラーが表示され、これでは意味がわかりません。
なぜ失敗したかをユーザーに伝えるために、理解できる出力にしましょう。

ここではプロジェクトをリファクタリングして、これら4つの問題を扱います。

## バイナリプロジェクトの責任の分離

`main`関数に複数の仕事の責任を割り当てるという構造の問題は、多くのバイナリプロジェクトでありふれています。
結果として、`main`が肥大化し始めた際にバイナリプログラムの個別の責任を分割するためにガイドラインとして活用できる工程をRustコミュニティは開発しました。
この工程は以下のような手順になっています。

- プログラムを`main.rs, lib.rs`に分け、ロジックを`lib.rs`に移動する。
- コマンドライン引数の解析ロジックが小規模な限り、`main.rs`においても良い。
- コマンドライン引数の解析ロジックが複雑かの様相を呈し始めたら、`main.rs`から抽出して`lib.rs`に移動する。

この工程の後に`main`関数に残る責任は以下に限定される。

- 引数の値でコマンドライン引数の解析ロジック
- あらゆる設定
- `lib.rs`の`run`関数
- `run`の返り値のエラー処理

このパターンは責任の分離です。`main.rs`はプログラムの実行をし、`lib.rs`は全てのロジックを扱うということです。
これに従って、プログラムのリファクタを行いましょう。

### 引数解析器を抽出

引数解析の機能を`main`関数から抽出して、`lib.rs`に移動します。
まずは`parse_config`関数を作成します。

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    // --snip--
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
```

これでもまだ不十分です。使用する引数の変数をベクターに集結させただけです。

この変更後、プログラムを実行して、引数解析がまだ動作しているか確認してください。
問題が発生したときに、原因を特定する助けにするために頻繁に進捗を確認するのは良いことです。

### 設定値をまとめる

もう少し`parse_config`を改善しましょう。
現時点ではタプルを返しますが、即座にタプルを分解して再度個別の値にしています。
これは正しい抽象化ができていない兆候です。

改善の余地があるのは、`parse_config`の`config`の部分です。
返り値のタプルを構造体に置き換え、`Config`構造体を作成し、フィールドに意味のある名前をつけましょう。
こうすることで、将来このコードのメンテナンス者が、異なる値が相互に関係する方法や目的を理解しやすくなります。

> 注釈、この複雑型(conplex type)がより適切なときに組み込みの値を使うアンチパターンを
> **primitive obsession（組み込み型強迫観念）** と呼ぶ人もいます。

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let mut f = File::open(config.filename).expect("file not found");

    // --snip--
}
struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
```

`query, filename`を`Config`構造体に追加することで、コードが良くなりました。

`String`のデータは、色々の手法で管理できますが、そのなかで`clone`メソッドで管理する手法は最も単純で非効率です。
`clone`メソッドを呼び出すことで、`Config`インスタンスが所有するデータの総コピーが生成され、文字列データへの参照を保持する要理も、時間とメモリを消費します。
ですがデータをクローンすることで、コードがとても素直になります。
この場面において、少々のパフォーマンスを犠牲にして単純性を得るのは間違いではありません。

> **`clone`の代償**
>
> 実行時のコストを下げるために、Rustaceanは`clone`を使用せず所有権問題を解消する傾向にあります。
> 第13章で、この種の問題に有用なメソッドなどを学びます。
> 今は、コピーは一回限りで、ファイル名とクエリ文字列は非常に小さなものなので、`clone`で問題はないです。
> 初めは究極的に効率化しようとするより、ちょっと非効率的でも動くプログラムを書く方が良いです。
> もっとRustの経験を積めば、最も効率的な解決法から始めることもできるようになりますが、今は`clone`でいきましょう。

`main`関数に`Config`のインスタンスの`config`変数を定義したことにより、この構造体のフィールドを使用するようになりました。

これで`query, filename`の関連と目的が明確になりました。これらの値は、`config`インスタンスでアクセスできます。

### `Config`のコンストラクタの作成

ここまでコマンドライン引数解析のロジックを`main`関数から抽出し、`parse_config`関数に移動しました。
これで`query, filename`が関連し、その関係性がコードを読むことで確認できるようになりました。
それから`Config`構造体に`query, filename`のフィールドを作成し、`parse_config`関数からこの構造体を返すことになりました。

これで`parse_config`関数は`Config`インスタンスを生成こととなったので、これは`Config`構造体に紐づく`new`という関数に置き換えることができます。
この変更を行うことで、コードがより慣用的になります。
`String`などの標準ライブラリの型のインスタンスを、`String::new`を呼び出すことで生成できます。
同様に、`parse_config`を`Config`に紐づく`new`関数に変えれば、`Config::new`を呼び出すことで`Config`のインスタンスを生成できるようになります。

```rust
impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
```

`parse_config`と`Config::new`の違いは名前だけです。処理は変えていません。
再度コードをコンパイルして動作確認してみてください。

## エラー処理を修正

次はエラー処理の修正をやっていきましょう。
現時点ではベクターが2個以下の要素しか含まれていないときに`args`にアクセスしようとするとパニックになります。
内容は次のとおりです。

```bash
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep`
thread 'main' panicked at 'index out of bounds: the len is 1 but the index is 1', src/main.rs:29:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

`'index out of bounds: the len is 1 but the index is 1'`と書かれています。これはプログラマ向けのエラーメッセージです。
エンドユーザーはこれが何を意味しているのかわかりません。これを修正していきます。

### エラーメッセージの改善

`new`関数に、添え字1, 2にアクセスできるかどうかチェックするコードを`new`関数に追加します。
チェックが通らなければ、パニックを起こしわかりやすいエラーメッセージを表示するようにします。

```rust
fn new(args: &[String]) -> Config {
    if args.len() < 3 {
        panic!("not enough arguments");
    }

    // --snip--
}
```

上記のコードでは、コマンドライン引数の長さが3以下であれば、プログラムを終了し「引数の数が足りません」というメッセージを表示します。

では、プログラムを実行してエラーがどのような見た目か確認してみましょう。

```bash
cargo run
   Compiling minigrep v0.1.0 (/Users/ittoku/Documents/learning/rust/projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.62s
     Running `target/debug/minigrep`
thread 'main' panicked at 'not enough arguments', src/main.rs:30:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

よくはなりましたが、この出力ではまだ不十分です。無駄な出力が多すぎます。
`panic!`はプログラミング上の問題の解決に適したマクロです。ユーザーに見せるためのものではありません。
ユーザーに適したメッセージを出力するには`Result`を使用するのが良いでしょう。

### `panic!`の代わりに`Result`

`Result`で、成功したら`Config`インスタンスを返し、失敗したら`Err`列挙子を返すようにします。

```rust
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
```

これで`new`関数は、成功時に`Config`インスタンスを返し、失敗時には`&'static str`を返すようになりました。
エラー時に返される型は`&'static str`ですが、ここにはエラーメッセージが入ります。

`Result`で返す関数の注意点として、成功時は`Ok()`、失敗時は`Err()`で値を囲む必要がある点です。こうしなければプログラムは動きません。

`Config::new`から`Err`値を返すことで、`main`関数内で`Result`値を処理し、エラーだった場合に綺麗にプログラムを終了させることができます。

### `Config::new`でエラー処理

エラーケースを処理し、ユーザーフレンドリーなメッセージを出力するために、`main`関数内で`Config::new`から返される`Result`を処理する必要があります。

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
}
```

`unwrap_or_else`は、標準ライブラリの`Result<T, E>`に定義されています。
このメソッドを使用することで、`panic!`以外の独自のエラー処理を定義することができます。
`Result`が`Ok`値の場合`Ok`が包んでいる中身の値を返し、`Err`値の場合`Err`が包んでいる値をクロージャ内に展開します。

`process::exit`関数は、即座にプログラムを停止させ、渡された数字を終了コードとして返します。
上記のコードでは1を返しています。0以外の終了コードは、プログラムがエラー状態で終了したことを通知する慣習です。

```bash
cargo run
   Compiling minigrep v0.1.0 (/Users/ittoku/Documents/learning/rust/projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/minigrep`
Problem parsing arguments: not enough arguments
```

## `main`からロジックを抽出



### `run`関数からエラーを返す



### `main`で`run`のエラーを処理



## ライブラリクレートに分割




