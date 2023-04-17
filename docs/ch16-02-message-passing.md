# メッセージ受け渡し

> Ref: https://doc.rust-jp.rs/book-ja/ch16-02-message-passing.html

人気度を増してきている安全な並行性を保証する1つのアプローチがメッセージ受け渡しです。
これはスレッドやアクターがデータを含むメッセージを相互に送り合うことでやり取りします。

[Go言語のドキュメンテーション](http://golang.org/doc/effective_go.html)のスローガンの1つに「メモリを共有することでやりとりするな。代わりにやりとりすることでメモリを共有しろ」というものがあります。

Rustにはチャンネルというメッセージ送信並行性を実現する道具があり、Rustの標準ライブラリが実装を提供しているプログラミング概念です。
プログラミングのチャンネルは、水の流れのように考えることができます。
アヒルのおもちゃやボートみたいなものを流れに置いたら、水路の終端まで下流に流れていきます。

プログラミングにおけるチャンネルは転送機と受信機の2つに分けることができます。
転送機はアヒルのおもちゃを川に置く上流になり、受信機はアヒルのおもちゃが行き着く下流になります。

コード内で送信したいデータとともに転送機のメソッドを呼び出し、別の部分がメッセージが到着していないか受信側を調べます。
転送機と受信機のどちらかがドロップされるとチャンネルは閉じられます。

ここで1つのスレッドが値を生成し、それをチャンネルに送信し、別のスレッドがその値を受け取り、出力するプログラムを書いてみましょう。
チャンネルを使用してスレッド間に単純な値を送り、機能の説明を行います。
一度このテクニックに慣れてしまえば、チャンネルを使用したチャットシステムや、多くのスレッドが計算の一部を担い、結果をまとめる1つのスレッドにその部分を送るようなシステムを実装できるようになります。

ではまずはチャンネルを生成してみましょう。このコードはまだコンパイルできません。

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();
```

`mpsc::channel`関数で新しいチャンネルを生成しています。
`mpsc`は**multiple producer, single consumer**を表しています。

簡潔に言えば、Rustの標準ライブラリがチャンネルを実装している方法は、1つのチャンネルが値を生成する複数の送信側と、その値を消費するたった1つの受信側を持つことを意味します。
複数の小川が合わさって1つの大きな川になると、どの小川を通っても送られたものが最終的に1つの川に行き着きます。

`mpsc::channel`関数はタプルを返し、1つ目の要素は送信側、2つ目の要素は受信側になります。
`tx, rx`という略称は、多くの分野で伝統的に転送機と受信機に使用されているもので、各終端を示しています。

立ち上げたスレッドがメインスレッドとやりとりするように、転送機を立ち上げたスレッドに移動し1文字列を送ってみましょう。
これは川の上流にアヒルのおもちゃを置いたり、チャットのメッセージをあるスレッドから別のスレッドに送るみたいなものです。

```rust
use std::thread;
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
});
```

`thread::spawn`を使用して新しいスレッドを生成し、`move`を使用して、`tx`を所有するようにクロージャにムーブしています。
立ち上げたスレッドはメッセージをチャンネルを通して送信できるように、チャンネルの送信側を所有する必要があります。

転送側には送信したい値を取る`send`メソッドがあります。
これは`Result<T, E>`型を返すので、すでに受信側がドロップされ値を送信する場所がなければ送信処理はエラーを返します。

以下のコードではメインスレッドのチャンネルの受信側から値を得ます。
アヒルのおもちゃを川の終端で水から回収したり、チャットメッセージを取得するようなものです。

```rust
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
});

let received = rx.recv().unwrap();
println!("Got: {}", received);
```

チャンネルの受信側には有用なメソッドが2つあります。
`recv`メソッドはメインスレッドの実行をブロックし、値がチャンネルを流れてくるまで待機します。
一旦値が送信されたら、`recv`はそれを`Result<T, E>`に含んで返します。
チャンネルの送信側が閉じたら`recv`はエラーを返し、値が来ないことを通知します。

`try_recv`メソッドはブロックせず即座に`Result<T, E>`を返します。
メッセージがあれば`Ok`値を、なければ`Err`値を返します。
メッセージを待つ間にこのスレッドにすることが他にあれば、`try_recv`は有用です。
`try_recv`を頻繁に呼び出しメッセージがあったら処理し、なければ再度チェックするまでの間、他の作業をするループを書くことができます。

この例では簡潔性のために`recv`を使用しています。
メッセージを待つこと以外にメインスレッドがすべき作業はないので、メインスレッドをブロックするのは適切です。

上記のコードを実行すると以下のような出力になります。

```
Got: hi
```

## 所有権の転送

借用規則は安全な並行コードを書く手助けをしてくれるので、メッセージ送信では重要な役割を担っています。
並行プログラミングでエラーを回避することは、Rustプログラム全体で所有権について考える利点です。
試しにチャンネルと所有権がともに動いて、どう問題を回避するかをみてみましょう。

以下のコードはコンパイルできません。なぜ許容されないのか考えてみてください。

```rust
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
    println!("val is {}", val);
});

let received = rx.recv().unwrap();
println!("Got: {}", received);
```

ここでは`tx.send`経由でチャンネルに送信後に`val`を出力しようとしています。これを許可するのは悪い考えです。
一度値が他のスレッドに送信されたら、再度値を使用とする前にそのスレッドが変更したりドロップできてしまいます。
可能性としてその別のスレッドの変更により、矛盾していたり存在しないデータのせいでエラーが発生したり、予期しない結果になるでしょう。

上記のコードをコンパイルすると以下のようなエラーを出力します。

```
error[E0382]: borrow of moved value: `val`
  --> src/main.rs:49:31
   |
47 |         let val = String::from("hi");
   |             --- move occurs because `val` has type `String`, which does not implement the `Copy` trait
48 |         tx.send(val).unwrap();
   |                 --- value moved here
49 |         println!("val is {}", val);
   |                               ^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
```

並行性のミスがコンパイルエラーを招きました。
`send`関数は引数の所有権を奪い、値がムーブされると受信側が所有権を得ます。
これにより送信後に誤って再度値を使用するのを防いでくれます。所有権システムのおかげです！

## 複数の値を送信

前の前のコードは動作しましたが2つの個別のスレッドがお互いにチャンネル越しに会話していることは明瞭には示されませんでした。
以下のコードは立ち上げたスレッドで複数のメッセージを送信し各メッセージ間で1秒待機させることで、並行に動作することを証明しています。

```rust
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("Got: {}", received);
}
```

今回はメインスレッドに送信したい文字列のベクターを立ち上げたスレッドが持っています。
それらを繰り返し各々個別に送信し、`thread::sleep`関数を呼び出すことで、1秒間メッセージを停止します。

メインスレッドにおいて、もはや`recv`関数を明示的に呼んではいません。 代わりに`rx`をイテレータとして扱っています。
受信した値それぞれを出力し、チャンネルが閉じられると繰り返しも終わります。

上記のコードを実行すると、各行の間に1秒の待機をしつつ以下のような出力を行います。

```
Got: hi
Got: from
Got: the
Got: thread
```

メインスレッドの`for`ループには停止したり、遅らせたりするコードは何もありません。
なのでメインスレッドが立ち上げたスレッドから値を受け取るのを待機していることが分かります。

## 転送機をクローン

`mpsc`は**multiple producer, single consumer**の略称です。
`mpsc`を使用して、すべての値を同じ受信機に送信する複数のスレッドを生成してみましょう。
チャンネルの転送の片割れをクローンすることで実現できます。

```rust
let (tx, rx) = mpsc::channel();

let tx1 = mpsc::Sender::clone(&tx);
thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

thread::spawn(move || {
    let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("Got: {}", received);
}

```

最初のスレッドを立ち上げる前に、チャンネルの送信側に対して`clone`を呼び出しています。
これにより最初に立ち上げたスレッドに渡せる新しい送信ハンドルが得られます。
元のチャンネルの送信側は、2番目に立ち上げたスレッドに渡します。
これにより2つのスレッドが得られ、それぞれチャンネルの受信側に異なるメッセージを送信します。

このコードを実行すると以下のような出力になります。

```
Got: hi
Got: more
Got: messages
Got: from
Got: for
Got: the
Got: you
Got: thread
```

別の順番で値が出る可能性もあります。これはシステム次第であり並行性は面白いと同時に難しい部分でもあります。
異なるスレッドで色々な値を与えて`thread::sleep`で実験をしたら、走らせるたびにより非決定的になり、毎回異なる出力になります。

チャンネルの動作方法を学んだので、他の並行性に目を向けていきましょう。
