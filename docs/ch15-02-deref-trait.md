# Derefトレイト

> Ref: https://doc.rust-jp.rs/book-ja/ch15-02-deref.html

`Deref`トレイトを実装することで、参照外し演算子の`*`の振る舞いをカスタマイズできます。
`Deref`を実装してスマートポインタを普通の参照のように扱うことができれば、参照に対し処理を行うコードを書き、そのコードをスマートポインタに対して使うことができるのです。

まずは参照外し演算子が普通の参照に対して動作するところを見てみましょう。
それから`Box<T>`のように振る舞う独自の型を定義していみましょう。
参照とは異なり、新しく定義した型には参照外し演算子を使えません。その理由を確認します。
`Deref`トレイトを実装すればスマートポインタは参照と同じように機能するのでその方法を見ていきましょう。
そしてRustには参照外し強制という機能があり、その機能により参照やスマートポインタをうまく使うことができるので、こちらも紹介します。

## 参照外し演算子

普通の参照は1種のポインタであり、ポインタはどこか他の場所に格納された値への矢印と見なすことができます。
以下のコードは、`i32`の参照を生成してから参照外し演算子を使ってデータまで参照をたどります。

```rust
let x = 5;
let y = &x;

assert_eq!(5, x);
assert_eq!(5, *y);
```

`x`は5を保持しています。`y`はxへの参照として設定しています。
`x`は5に等しいとテストしています。`y`の値をテストしたい場合、参照外し演算子を使用して参照が指している値まで追いかける必要があります。
`y`の参照を外せば、`y`が指している値にアクセスすることができます。

では参照外し演算子を使用せずコンパイル使用とすると、以下のようなエラーになります。

```bash
cargo test
error[E0277]: can't compare `{integer}` with `&{integer}`
  --> src/main.rs:28:9
   |
28 |         assert_eq!(5, y);
   |         ^^^^^^^^^^^^^^^^ no implementation for `{integer} == &{integer}`
   |
   = help: the trait `PartialEq<&{integer}>` is not implemented for `{integer}`
   = help: the following other types implement trait `PartialEq<Rhs>`:
             f32
             f64
             i128
             i16
             i32
             i64
             i8
             isize
           and 6 others
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
```

内容は、数値と数値の参照の比較が許されていないこと、PartialEqは数値に対して実装されていないことが書かれています。

## Box<T>を参照のように使う
