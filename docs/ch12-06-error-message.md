# 標準エラーにメッセージを書き込む

> Ref: https://doc.rust-jp.rs/book-ja/ch12-06-writing-to-stderr-instead-of-stdout.html

今まで出力は`println!`関数を使用して端末に書き込んでいました。
多くの端末では2種類の出力を提供しています。
1つは普通の情報ようの標準出力(`stdout`)、もう1つはエラーメッセージようの標準エラー出力(`stderr`)です。
この差異のおかげで、ユーザーはエラーメッセージを画面に表示しつつ、プログラムの成功した出力をファイルにリダイレクトするといったこともできるようになります。

`println!`関数は標準出力しかできないので、標準エラーを出力するには他の関数を使用しなければなりません

## エラーが書き込まれる場所を確認

まずは`minigrep`で出力されるものが結果によるものか、エラーによるものかをみていきます。

コマンドラインプログラムは、エラーメッセージを標準エラー出力に送信していると期待しています。
標準出力ストリームをファイルにリダイレクトしても、画面にエラーメッセージを表示します。
現在の`minigrep`ではそうなってはいません。

この動作をでもする方法は、以下のコマンドを実行します。

```bash
cargo run > output.txt
```

`>`記法により、標準出力の中身を画面ではなく`output.txt`に書き込むようにシェルに指示することができます。
実行するとプログラムの出力は何も表示しません。ということは`output.txt`に出力が書き込まれているということです。

```txt
Problem parsing arguments: not enough arguments
```

エラーメッセージは標準出力として出力されています。
このようなエラーメッセージは標準エラーに出力され、成功した状態のデータのみがファイルに残るとはるかに有用です。
こちらを変更していきます。

## エラーを標準エラーに出力

エラーメッセージを出力するには、`eprintln!`マクロを使用します。

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
```

では以下のコマンドを実行してみましょう。エラーが端末に表示されるはずです。
`output.txt`の中身は何も入っていないはずです。

```bash
cargo run > output.txt

Problem parsing arguments: not enough arguments
```

次は、エラーを起こさないようにコマンドを実行してみましょう。

```bash
cargo run to poem > output.txt
```

ターミナルには出力は見られず、`output.txt`に結果が含まれます。

```txt
Are you nobody, too?
How dreary to be somebody!
```

これで、`minigrep`を標準出力と標準エラー出力に分けることができました。

## まとめ

この章では、これまでに学んできた主要な概念の一部を念押しし、Rustで入出力処理を行う方法を見ていきました。
コマンドライン引数、ファイル、環境変数、エラー出力が理解できればコマンドラインアプリケーションを書くことができます。
それと、コードの体系化、データ構造に効率的にデータを保存、エラー処理などのテストもされていることでしょう。

次は、関数型言語に影響されたRust機能を見ていきます。クロージャとイテレータについてです。
