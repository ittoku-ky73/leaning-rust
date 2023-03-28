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
