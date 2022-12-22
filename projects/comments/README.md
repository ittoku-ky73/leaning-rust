# コメント

> 参考：https://doc.rust-lang.org/book/ch03-04-comments.html

全てのプログラマーはコードを理解しやすくするよう努めていますが、追加の説明が必要な場合もあります。このような場合、コンパイラーが無視するソースコードにコメントを残すことで、ソースコードを読む人にとって役立つ場合があります。

```rust
// hello, world
```

Rustでは、慣用的なコメントスタイルは2つのスラッシュでコメントを開始し、コメントは行末まで続きます。コメントが1行を超える場合は、`//`を各行に含める必要があります。

```rust
// So we're doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what's going on.
```

コードを含む行の最後にコメントを配置することもできます。

```rust
fn main() {
    let lucky_number = 7; // I'm felling lucky today
}
```

コードの上に注釈をつけてコメントをつける方法もあります。

```rust
fn main() {
    // I'm felling lucky today
    let lucky_number = 7;
}
```

Rustには別の種類のコメント、ドキュメンテーションコメントもあります。これについては、第14章で説明します。
