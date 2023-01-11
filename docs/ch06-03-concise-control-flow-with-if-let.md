# if letを用いた制御フロー

> 参考: https://doc.rust-lang.org/stable/book/ch06-03-if-let.html

`if let`構文を使うと、1つのパターンにマッチする値を処理する一方で残りを無視することができます。`match`では冗長になってしまうところを`if let`だとそれを解決できます。
例として、`config_max`変数の`Option<u8>`値にマッチするが、値が`Some variant`である場合にのみコードを実行したい場合を考えてみましょう。`match`ではこう書きます。

```rust
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}
```

上記のコードを`if let`構文で書くと次のようになります。

```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}
```

`if let`構文は、パターンと式を等号で区切って取ります。この場合、パターンは`Some(max)`であり、`max`は`Some`内の値と結合します。そして`Some(max)`と`config_max`のパターンが一致した場合に括弧のコードが実行されます。マッチしなかった場合は、括弧の中のコードは実行されません。

`if let`を使うと、タイプ数が減り、インデントが減り、定型的なコードにならなくなりますが、`match`のような網羅的なチェックはできなくなります。
`match`と`if let`のどちらを使うかは、状況次第で決まります。

言い換えれば`if let`は、値がパターンにマッチした時にコードを実行し、それ以外の値を無視する`match`のシンタックスシュガーと考えることができます。

`if let`には`else`を含めることができます。`else`に付随するコードのブロックは、`match`式の`_ case`と同じです。前のセクションで書いたコイン列挙型を`match`式と`if let`構文で書いて比較してみましょう。

**match**

```rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}", state),
    _ => (),
}
```

**if let**

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}", state);
} else {
    count += 1;
}
```

`if let`の方が、行が長くなってしまいますが、読みやすくもなっていると思います。

## まとめ

ここまで列挙型を使用して、一連の列挙された値のうちの1つになることができるカスタム型を作成する方法についてみていきました。
標準ライブラリの`Option<T>`型が、エラーを防ぐために型システムのに役立つことを見てきました。
`enum`値がその内部にデータを持つ場合、処理する必要があるケース数に応じて、`match`または`if let`を使用して値を抽出して使用する方法を見てきました。

Rustのプログラムでは、構造体や列挙型を使用してドメイン内の概念を表現できるようになりました。APIで使用するカスタム型を作成すると、コンパイラは各関数が期待する型の値のみを取得することを保証します。

次は、ユーザーが必要とするものだけを公開し、使いやすく整理されたAPIをユーザーに提供するためのモジュールについてみていきます。
