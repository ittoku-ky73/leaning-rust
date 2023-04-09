# 循環参照

> Ref: https://doc.rust-jp.rs/book-ja/ch15-06-reference-cycles.html

Rustのメモリ安全保障によって、誤って絶対に片付けられることのないメモリ（メモリリーク）を生成するようなことはなくなったと思われていますが、そうではありません。
コンパイル時にデータ競合を防ぐのと同じようにメモリリークは完全に回避するのは、Rustの保証の1つでしかなく、メモリリークはRustにおいてはメモリ安全であることを意味します。

Rustでは、`Rc<T>, RefCell<T>`を使用してメモリリークを許可します。
これはつまり要素がお互いに循環して参照する参照を生成することも可能ということです。
循環の各要素の参照カウントが絶対に0にならないので、これはメモリリークを引き起こし、値はドロップされません。

## 循環参照する

以下のコードは前の節で書いたコードの`List`enum定義に`tail`メソッドを追加しています。

```rust
#[derive(Debug)]
pub enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    pub fn tail(&self) -> Option<&RefCell<Rc<ListV4>>> {
        match self {
            List::Cons(_, ref item) => Some(item),
            List::Nil => None,
        }
    }
}
```

上記のコードの`Cons(i32, RefCell<Rc<List>>)`の部分では、`Cons`列挙子が指している`List`値の先を変えています。
また`tail`メソッドを追加して`Cons`列挙子がある時に2番目の要素にアクセスすることができます。

では上記で定義した`List`を使ってみましょう。

```rust
let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
println!("a initial rc count = {}", Rc::strong_count(&a));
println!("a next item = {:?}", a.tail());
let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
println!("a rc count after b creation = {}", Rc::strong_count(&a));
println!("b initial rc count = {}", Rc::strong_count(&b));
println!("b next item = {:?}", b.tail());
if let Some(link) = a.tail() {
    *link.borrow_mut() = Rc::clone(&b);
}
println!("b rc count after changing a = {}", Rc::strong_count(&b));
println!("a rc count after changing a = {}", Rc::strong_count(&a));
// Uncomment the next line to see that we have a cycle;
// it will overflow the stack
// println!("a next item = {:?}", a.tail());
```

上記のコードでは、`a`が`b`を刺すように変更を加えて循環させています。
そしてこのコードを実行すると以下のような出力になります。

```
a initial rc count = 1
a next item = Some(RefCell { value: Nil })
a rc count after b creation = 2
b initial rc count = 1
b next item = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })
b rc count after changing a = 2
a rc count after changing a = 2
```

この循環参照を可視化したすると以下のような図式になります。

![reference-cycles](https://doc.rust-jp.rs/book-ja/img/trpl15-04.svg) 

また上記のコードのコメントを外してプログラムを実行すると、`a`が`b`を指して、`b`が`a`を指してと繰り返し、スタックがオーバーフローするまでコンパイラはこの循環を出力しようとします。

この場合、循環参照を作る直後にプログラムは終了します。
この循環の結果は、それほど悲壮なものではありません。
しかしながら、より複雑なプログラムが多くのメモリを循環で確保し長い間その状態を保つとします。
プログラムは必要以上のメモリを使用し、使用可能なメモリを枯渇させてシステムを参らせてしまう可能性があります。

循環参照は簡単にできることではありませんが、不可能というわけでもありません。
`Rc<T>`値を含む`RefCell<T>`値などの内部可変性と参照カウントのある型がネスとして組み合わさっていたら、循環しないことを保証しなければなりません。
コンパイラがそれを捕捉することを信頼できないのです。
循環参照をするのは、自動テストやコードレビューなどの他のソフトウェア開発手段を使用して最小化すべきプログラム上のロジックバグでしょう。

循環参照を回避する別の解決策は、ある参照は所有権を表現して他の参照はしないというようにデータ構造を再構成することです。
結果として所有権のある関係と所有権のない関係からなる循環ができ、所有権のある関係だけだ値をドロップするかどうかに影響します。

上記のコードでは、常に`Cons`列挙子にリストを所有して欲しいので、データ構造を再構成することはできません。
親ノードと子ノードからなるグラフを使った例に目を向けて、どんな時に所有権のない関係が循環参照を回避するのに適切な方法なのか見ていきます。
