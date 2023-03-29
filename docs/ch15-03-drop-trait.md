# Dropトレイト

> Ref: https://doc.rust-jp.rs/book-ja/ch15-03-drop.html

`Drop`トレイトはスマートポインタパターンにとって重要な2番目の機能です。
これのおかげで値がスコープを抜けそうになったときに起こることをカスタマイズすることができます。

またどんな型に対しても`Drop`トレイトの実装を提供することができます。
例えば指定したコードはファイルやネットワーク接続などのリソースを開放するのに活用できます。
`Drop`トレイトの機能はほぼ常にスマートポインタを実装するときに使われています。
例えば、`Box<T>`は`Drop`をカスタマイズしてボックスが指しているヒープの領域を開放しています。

ある言語ではプログラマがスマートポインタのインスタンスを使い終わるたびにメモリやリソースを開放するコードを呼ばなくてはなりませんでした。
これを忘れてしまったら、プログラムがクラッシュする可能性があります。

Rustでは値がスコープを抜けるたびに特定のコードが走るように指定でき、コンパイラはこのコードを自動的に挿入します。
結果として特定の型のインスタンスを使い終わったプログラムの箇所全てにクリーンアップコードをプログラマが書く必要がなくなります。
これでリソースをリークすることがなくなるわけです。

`Drop`トレイトを実装することで値がスコープを抜けたときに走るコードを指定できます。
実装するには`self`への可変参照を取る`drop`というメソッドを実装する必要があります。
いつRustがdropを呼び出すのか確認するために、`println!`マクロを使って実際に`drop`を実装してみましょう。

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {} !", self.data);
    }
}

let c = CustomSmartPointer { data: String::from("my stuff") };
let d = CustomSmartPointer { data: String::from("other stuff") };
println!("CustomSmartPointers created");
```

`Drop`トレイトは初期化処理に含まれるのでインポートする必要はありません。
`CustomSmartPointer`に`Drop`トレイトを実装し、`println!`を呼び出す`drop`メソッドの実装を提供しています。
`drop`関数の本体は、自分の型のインスタンスがスコープを抜ける際に実行するロジックを配置します。

`CustomSmartPonter`インスタンスを2つ作り、それから`println!`で出力しています。
コンパイラは最後のメッセージを出力して、`drop`メソッドにあるコードを呼び出しています。

プログラムを実行すると、以下のような出力になります。

```
CustomSmartPointers created
Dropping CustomSmartPointer with data other stuff !
Dropping CustomSmartPointer with data my stuff !
```

インスタンスがスコープを抜けた際に指定したコードを呼び出しながらコンパイラは、`drop`を自動的に呼び出します。
変数は生成された逆の順番でドロップされるので、`d, c`の順でドロップされます。
通常はメッセージ出力でなく、自分の方が実行する必要のあるクリーンアップコードを指定します。

## std::mem::drop

自動的な`drop`機能を無効化することは、単純ではありません。普通は`drop`を無効化することはないので。
`Drop`トレイトの最重要な要点は自動的に考慮されることです。
しかし時として、値を早期に片付けたくなる可能性があります。

例えばロックを管理するスマートポインタを使用するケースです。
`drop`トレイトの`drop`メソッドは明示的に呼び出すことはできません。
これでは同じスコープの他のコードがロックを獲得できるように、ロックを開放するようなコードを書くことができません。
スコープが終わる前に値を強制的にドロップさせたい場合、標準ライブラリが提供する`std::mem::drop`関数を使用します。

以下の例は`Drop`トレイトの`drop`メソッドを呼び出そうとしています。コンパイルはできません。

```rust
let c = CustomSmartPointer {
    data: String::from("some data"),
};
println!("CustomSmartPointer created");
c.drop();
println!("CustomSmartPointer dropped before the end of main");
```

エラーは次のようになります。

```
error[E0040]: explicit use of destructor method
   --> src/main.rs:101:11
    |
101 |         c.drop();
    |         --^^^^--
    |         | |
    |         | explicit destructor calls not allowed
    |         help: consider using `drop` function: `drop(c)`

For more information about this error, try `rustc --explain E0040`.
```

明示的に`drop`メソッドを呼び出すことはできないことが書かれています。
エラーメッセージはデストラクタという専門用語を使っており、これはインスタンスを片づける関数の一般的なプログラミング専門用語です。
デストラクタはコンストラクタに類似しており、インスタンスを生成します。
Rustの`drop`関数は、1種の特定のデストラクタです。

コンパイラは`main`の終わりに`drop`を呼び出すので明示的に呼び出すことはできません。
コンパイラが2回同じ値を片付けようとするので、これは二重開放エラーになります。

値を早期に片付ける必要があるなら、`std::mem::drop`関数を使用します。

`std::mem::drop`関数は、`Drop`トレイトの`drop`メソッドとは異なります。
早期に強制的にドロップさせたい値を引数に渡すことで呼び出します。
この関数は初期化処理に含まれているので、`use`を使わずに`drop`関数を呼び出すことができます。

```rust
let c = CustomSmartPointer {
    data: String::from("some data"),
};
println!("CustomSmartPointer created");
drop(c);
println!("CustomSmartPointer dropped before the end of main");
```

このコードを実行すると以下のように出力されます。

```
CustomSmartPointer created
Dropping CustomSmartPointer with data some data !
CustomSmartPointer dropped before the end of main
```

`Drop`トレイト実装で指定されたコードを色々な方法で使用し、片付けを便利で安全に行うことができます。
この機能を理解することで独自のメモリアロケータも作ることができるでしょう。
`Drop`トレイトとRustの所有権システムがあれば、コンパイラが自動的に行うので片付けを覚えておく必要はなくなります。

まだ使用中の値を間違って片付けてしまうことによる問題もなくなります。
参照の有効性を確認する所有権システムが、`drop`が1度だけ呼ばれることを保証してくれるためです。

これで`Box<T>`とスマートポインタの特徴の一部をみてきました。
次は標準ライブラリに定義されている他のスマートポインタを見ていきましょう。
