# ファイルを読み込む

> Ref: https://doc.rust-jp.rs/book-ja/ch12-02-reading-a-file.html

では、`filename`コマンドライン引数で指定されたファイルを読み込む機能を追加しましょう。
まずテスト実行するためのサンプルファイルが必要です。
`minigrep`が動作するか確認するために最適なファイルは、複数行にわたって同じ単語のあるテキストです。
プロジェクトのルート階層に`poem.txt`というファイルを作成し、以下のテキストをそのファイルに書き込みます。
このテキストは、エミリー・ディキンソン(Emily Dickinson)の詩です。

```txt
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!

私は誰でもない！あなたは誰？
あなたも誰でもないの？
なら、私たちは組だね、何も言わないで！
あの人たちは、私たちを追放するでしょう。わかりますよね？

誰かでいるなんて侘しいじゃない！
カエルみたいで公すぎるじゃない。
自分の名を長い1日に告げるのなんて。
感服するような沼地にね！
```

次に、`src/main.rs`を編集し、引数を処理するコードの下にファイルを開くコードを追加します。

```rust
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // --snip--
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    
   println!("With text: \n{}", contents); 
}
```

`std::fs::File`はファイルを扱うのに使用し、`std::io::prelude::*`はファイル入出力を含む処理をするのに有効なトレイトを含んでいます。
言語が一般的な初期化処理で特定の方や関数を自動的にスコープに導入するように、`std::io`モジュールには独自の共通の型や関数の初期化処理があり、入出力を行う際に必要になります。
標準の初期化処理とは異なり、`std::io`の初期化処理には明示的に`use`分を加える必要があります。

`main`関数を見ていきましょう。ここで行なっている処理は次のとおりです。

1. `File::open`関数を読んで`filename`の値を渡して、ファイルへの可変なハンドルを得る処理です。
2. `contents`という名の変数を生成して、可変で空の`String`を割り当てる処理です。ファイル読み込み後に中身を保持します。
3. ファイルハンドルに対して`read_to_string`を呼び出し、引数として`contents`への可変参照を渡す処理です。

これらの行の後に、`contents`の値を出力する一時的な`println!`分を追記して、動作を確認します。

第1コマンドライン引数には適当な文字列を、第2引数に`poem.txt`ファイルを入れて、このコードを実行すると次のような出力になります。

```bash
cargo run frog poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep frog poem.txt`
Searching for frog
In file poem.txt
With text:
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!

私は誰でもない！あなたは誰？
あなたも誰でもないの？
なら、私たちは組だね、何も言わないで！
あの人たちは、私たちを追放するでしょう。わかりますよね？

誰かでいるなんて侘しいじゃない！
カエルみたいで公すぎるじゃない。
自分の名を長い1日に告げるのなんて。
感服するような沼地にね！
```

コードがファイルの中身を読み取り出力するようになりました。
しかし、このコードにはいくつか欠陥があります。
`main`関数が複数の責任を持っていることです。一般的に各関数がただ1つの責任だけを持つようになれば、関数は明確かつ管理しやすくなります。
もう1つの問題点は、できる限りのエラー処理を怠っている点です。
プログラムが小規模なうちは問題にはなりませんが、大規模になるにつれ、これを綺麗に解消するのは困難になっていきます。
プログラムを開発する際に、早い段階でリファクタリングを行うのは良い戦術です。
リファクタリングするコードの量が少なければ、はるかに簡単になります。
