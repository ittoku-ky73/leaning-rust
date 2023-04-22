# スレッド

> Ref: https://doc.rust-jp.rs/book-ja/ch16-01-threads.html

多くの現代のOSでは、実行中のプログラムのコードはプロセスで走り、OSは同時に複数のプロセスを管理します。
自分のプログラム内で、独立した部分を同時に実行できます。
これらの独立した部分を走らせる機能をスレッドと言います。

プログラム内の計算を複数のスレッドに分けると、パフォーマンスが改善します。
プログラムが同時に複数の作業を行うからですが、複雑にもなります。
スレッドは同時に走らせることができるので、異なるスレッドのコードが走る順番に関して保証はしません。
これは例えば以下のような問題になる可能性があります。

- スレッドがデータやリソースに矛盾した順番でアクセスする競合状態
- 2つのスレッドがリソースを使い終わるのを待つことによるデッドロック
- 特定の状況で飲み置き、確実な再現や修正が困難なバグ

Rustではスレッドを使用する際の悪影響を軽減します。
しかしそれでもマルチスレッドの文脈でのプログラミングでは、注意深い思考とシングルスレッドで走るプログラムとは異なるコード構造が必要です。

プログラミング言語によってスレッドはいくつかの方法で実装されています。
多くのOSで新規スレッドを生成するAPIが提供されています。
言語がOSのAPIを呼び出してスレッドを生成するこのモデルを時に`1:1`と呼び、1つのOSスレッドに対して1つの言語スレッドを意味します。

多くのプログラミング言語がスレッドの独自の特別な実装を提供しています。
プログラミング言語が提供するスレッドは、グリーンスレッドと呼び、このグリーンスレッドを使用する言語は、それを異なる数のOSスレッドの文脈で実行します。
このためグリーンスレッドのモデルは`M:N`モデルと呼びます。
`M`個のグリーンスレッドに対して、`N`個のOSスレッドがあり、`M, N`は必ずしも同じ数字ではありません。

各モデルにはそれだけの利点と代償があり、Rustにとって最も重要な代償は、ランタイムのサポートです。
ランタイムは混乱しやすい用語で文脈によって意味も変わります。

ここでのランタイムとは、言語によってすべてのバイナリに含まれるコードのことを言います。
言語によってこのコードの大小は決まりますが、非アセンブリ言語はすべてある量の実行時コードを含みます。
そのため口語的に誰かが「ノーランタイム」と言ったら「小さいランタイム」のことを意味することがあります。

ランタイムが小さいと機能も少なくなるのでより多くの機能と引き換えにランタイムのサイズを膨れ上がるのは、受け入れられます。
Rustにはほとんどゼロのランタイムが必要でパフォーマンスを維持するためにCコードを呼び出すなどしたくはありません。

`M:N`グリーンスレッドモデルは、スレッドを管理するのにより大きな言語ランタイムが必要です。
よってRustの標準ライブラリは、`1:1`スレッドの実装のみを提供しています。
Rustはそのような低級言語なので例えばスレッドがいつ走るかの詳細な制御や、より低コストの文脈切り替えなどの一面をオーバーヘッドと引き換えるなら、`M:N`スレッドの実装をしたクレートもあります。

今やRustにおけるスレッドを定義したので、標準ライブラリで提供されているスレッド関連のAPIの使用法を見ていきましょう。

## spawnで新規スレッド生成

新規スレッドを生成するには`thread::spawn`関数を呼び出し、新規スレッドで走らせたいコードを含むクロージャを渡します。

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

この関数では新しいスレッドは実行が終わったかどうかに関わらずメインスレッドが終了したら停止します。
このプログラムからの出力は毎回少々異なる可能性がありますが、以下のような出力になります。

```
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
```

`thread::sleep`を呼び出すと少しの間、スレッドは実行を止め、違うスレッドを走らせることができます。
スレッドはおそらく切り替わるでしょうが保証はありません。 OSがスレッドのスケジュールを行う方法によります。
この実行ではコード上で立ち上げられたスレッドの`print`文が先に現れているのに、メインスレッドが最後に出力しています。
また立ち上げたスレッドには`i`が9になるまで出力するよう指示しているのに、メインスレッドが終了する前の5までしか到達していません。

このコードを実行してメインスレッドの出力しか目の当たりにできたかったり、オーバーラップがなければ範囲の値を増やしてOSがスレッド切り替えを行う機会を増やしてみてください。

## joinハンドル

上記のコードはメインスレッドが終了するためにほとんどの場合、立ち上げたスレッドがすべて実行されないだけでなく、立ち上げたスレッドが実行されるかどうかも保証できません。
原因はスレッドの実行順に保証がないからです。

`thread::spawn`の戻り値を変数に保存することで、立ち上げたスレッドが実行されなかったり、完全には実行されなかったりする問題を修正することができます。
`thread::spawn`の戻り値の型は`JoinHandle`です。

`joinHandle`は、`join`メソッドを呼び出した時にスレッドの終了を待つ所有された値です。
以下のコードは、上記のコードで生成したスレッドの`joinHandle`を使用し、`join`を呼び出し`main`が終了する前に立ち上げたスレッドが確実に完了する方法を示しています。

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

ハンドルに対して`join`を呼び出すと、ハンドルが表すスレッドが終了するまで現在実行中のスレッドをブロックします。
スレッドをブロックするとは、そのスレッドが動いたり終了したりすることを防ぐことです。
`join`の呼び出しをメインスレッドの`for`ループの後に配置したので、このコードを実行すると以下のような出力になります。

```
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```

2つのスレッドが代わる代わる実行されていますが、`handle.join()`呼び出しのためにメインスレッドは待機し、立ち上げたスレッドが終了するまで終わりません。

ですが代わりに`handle.join()`を`for`ループの前に移動するとどうなるのか確認してみましょう。
メインスレッドは立ち上げたスレッドが終了するまで待ち、それから`for`ループを実行するので以下のような出力となります。

```
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
```

どこに`join`を呼ぶかと言った些細なことが、スレッドが同時に走るかどうかに影響することもあります。

## moveクロージャ

`move`クロージャは`thread::spawn`とともによく使用されます。
あるスレッドのデータを別のスレッドで使用できるようになるからです。

第13章では、クロージャの引数リストの前に`move`キーワードを使用して、クロージャに環境で使用している値の所有権を強制的に奪うことができると説明しました。
このテクニックは、あるスレッドから別のスレッドに値の所有権を移すために新しいスレッドを生成する際に有用です。

上記で書いたコードには`thread::spawn`に渡したクロージャに引数はありませんでした。
立ち上げたスレッドのコードでメインスレッドからのデータは何も使用していないということです。
立ち上げたスレッドでメインスレッドのデータを使用するには、立ち上げるスレッドのクロージャは必要な値をキャプチャしなければなりません。

以下のコードはメインスレッドでベクターを生成し、立ち上げたスレッド内で使用しています。

```rust
let v = vec![1, 2, 3];

let handle = thread::spawn(|| {
    println!("Here's a vector: {:?}", v);
});

handle.join().unwrap();
```

クロージャは`v`を使用しているので、`v`をキャプチャしクロージャの環境の一部にしています。
`thread::spawn`はこのクロージャを新しいスレッドで走らせるので、その新しいスレッド内で`v`にアクセスできるはずです。
しかしこのコードをコンパイルしようとすると以下のようなエラーになります。

```
error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
  --> src/main.rs:21:32
   |
21 |     let handle = thread::spawn(|| {
   |                                ^^ may outlive borrowed value `v`
22 |         println!("Here's a vector: {:?}", v);
   |                                           - `v` is borrowed here
   |
note: function requires argument type to outlive `'static`
  --> src/main.rs:21:18
   |
21 |       let handle = thread::spawn(|| {
   |  __________________^
22 | |         println!("Here's a vector: {:?}", v);
23 | |     });
   | |______^
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
   |
21 |     let handle = thread::spawn(move || {
   |                                ++++
```

Rustは`v`のキャプチャ方法を推論し、`println!`は`v`への参照のみを必要とするので、クロージャは`v`を借用しようとします。
ですがコンパイラには立ち上げたスレッドがどのくらいの期間走るのかわからないので、`v`への参照が常に有効であるか把握できないのです。

```rust
let v = vec![1, 2, 3];

let handle = thread::spawn(|| {
    println!("Here's a vector: {:?}", v);
});

drop(v);

handle.join().unwrap();
```

このコードを実行できてしまうなら、立ち上げたスレッドは全く実行されることなく即座にバックグラウンドに置かれる可能性があります。
立ち上げたスレッドは内部に`v`の参照を持ちますが、メインスレッドでは`drop`関数を使用して即座に`v`をドロップしています。
そして立ち上げたスレッドが実行を開始する時には、もう`v`は有効ではなく参照も不正になるのです。

解決策はクロージャの前に`move`キーワードをつけることです。
コンパイラに値を借用すべきと推論させるのではなく、クロージャに使用している値の所有権を強制的に奪わせます。

```rust
let v = vec![1, 2, 3];

let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
});

handle.join().unwrap();
```

ではメインスレッドが`drop`を呼び出すコードの場合どうなるでしょうか？
それは`move`を使用することで`v`をクロージャの環境にムーブさせることになるので、`drop`をそもそも呼び出せなくなります。
これはRustの所有権規則によるものです。

これでスレッドとスレッドAPIの基礎知識を得たことで、次はスレッドでできることを見ていきましょう。
