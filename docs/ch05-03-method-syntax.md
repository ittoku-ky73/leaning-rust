# メソッド構文

> 参考: https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html

メソッドは関数と似ています。`fn`キーワードと名前で宣言し、パラメータと戻り値を持つことができ、メソッドが他の場所から呼び出されたときに実行されるコードが含まれています。メソッドは関数とは異なり、構造体のコンテキスト内で定義され、最初のパラメータは常に`self`です。これはメソッドが呼び出されている構造体のインスタンスを表しています。

## メソッドの定義

以下のコードは、Rectangleインスタンスをパラメータとする`area`関数を変更し、代わりにRectangle構造体に定義された`area`メソッドを作成しています。

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

`impl`キーワードは、コンテキストで関数を定義するときに使用します。この`impl`ブロックにあるものが全てRectangle型に関連付けられます。

`area`のシグネチャでは、`&Rectangle`の代わりに、`&self`を使用しています。Self型は`impl`ブロックのための型の別名です。メソッドは最初のパラメータにSelf型の`self`という名前のパラメータを持たなければならないので、Rustでは最初のパラメータの場所に`self`という名前だけをつけて省略することができます。

関数では`&Rectangle`を、メソッドでは`&self`を使いました。メソッドでは構造体のデータを読み取るだけで、書き込むことはできません。書き込む場合は、`&mut self`を使用します。このテクニックは、メソッドが`self`を別のものに変換し、呼び出し元が変換後に元のインスタンスを使用できないようにする場合に使われます。

関数の代わりにメソッドを使う主な理由は、メソッド構文を提供し、全てのメソッドのシグネチャで`self`の型を繰り返す必要がないことや、整理するためです。将来このコードを使う人に、私たちが提供するライブラリの様々な場所でRectangleの機能を探す必要がなくなります。

また、構造体のフィールドと同じ名前をメソッドにもつけることができます。

```rust
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}
```

このメソッドは、`rect1.width`ではなく、`rect1.width()`とすることで呼び出すことができます。

フィールドと同じ名前のメソッドを与える場合、そのフィールドの値のみを返し、他のことは何もしないようにしたいことがあります。このようなメソッドをゲッター(Getter)と呼びますが、Rustでは他の言語のように構造体フィールドに対して自動的に実装されることはありません。ゲッターは、フィールドを非公開にしてメソッドを公開することで、その型の公開APIの一部としてそのフィールに読み取り専用でアクセスできるようにするのが便利です。`private, public`などについては第7章で説明します。

> **-> Operator**とは？
>
> `C, C++`では、メソッドの呼び出しに2つの異なる演算子を使用します。オブジェクトのメソッドを直接呼び出す場合は`.`、オブジェクトへのポインタのメソッドを呼び出す場合は`->`を使用し、最初にポインタをデリファレンスする必要があります。つまり、`object`がポインタの場合、`object->something()`は、`(*object).something()`と同じような意味になります。
>
> Rustには、`->`演算子のようなものはなく、自動参照・自動非参照という機能があります。これは、`object.something()`でメソッドを呼び出すと、Rustが自動的に`&, &mut, *`を追加して`object`がメソッドのシグネチャに一致するようにする仕組みです。
>
> ```rust
> p1.distance(&p2);
> (&p1).distance(&p2);
> ```
>
> 初めのコード例の方が見やすいです。この自動的な参照動作はメソッドに明確なうけて(`self`)があるため機能します。メソッドのレシーバと名前があれば、Rustはそのメソッドが読み取り(`&self`)、変異(`&mut self`)、消費(`self`)のいずれであるかを明確に判断することができます。Rustがメソッドのレシーバに対して暗黙の借用を行うことは、実際の所有権を人間工学的なものにするための大きな要素です。

## パラメータの多いメソッド

Rectangle構造体に対して2つ目のメソッドを実装して、メソッドの使い方を練習して見ましょう。以下のコードはまだ動きません。これを動くようにしていきます。`can_hold`メソッドは、`self`の範囲が引数のRectangleインスタンスより小さければ`true`、大きければ`false`になるようにプログラムしていきます。

```rust
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

プログラムが完成したときに、このような出力になればOKです。

```none
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

まずは、`impl Rectangle`に`can_hold`メソッドを定義します。パラメータとして別のRectangleの`immutable borrow`を取ります。戻り値は`bool`で、実装は`self`の幅と高さがそれぞれもう一方のRectangleの幅と高さより大きいかどうかチェックします。

```rust
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

上記のコードを追加することで、プログラムは正常に動作します。

## 関連付け関数

`impl`で定義された全ての関数は、名付けられた型と関連付けられるので、関連付け関数と呼ばれます。また最初のパラメータに`self`を持たせない関数を定義することもできます。

メソッドではない関連する関数は、構造体の新しいインスタンスを返すコンストラクタによく使われます。これはしばしば`new`と呼ばれますが、Rustにとって`new`は特別な名前ではなく、言語に組み込まれているわけでもありません。例えば、`square`という正方形を定義する関連関数では、1つの`size`パラメータを持たせるだけで、簡単にRectangleを作成することができます。

```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

let sq = Rectangle::square(3);
```

戻り値のSelfは、この場合Rectangleです。呼び出しに使われる`Rectangle::square(3)`で使われている`::`構文は、関連付けられた関数と、モジュールによって作成された名前空間の両方に使用されます。モジュールについては第7章で説明します。

## 複数の`impl`ブロック

核構造体は、複数の`impl`ブロックを持つことが許可されています。

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

複数の`impl`ブロックが有用であるケースは、第10章のジェネリック型とtraitで説明します。

## まとめ

構造体を使用すると、ドメインにとって意味のあるカスタムタイプを作成することができます、構造体を使用することで、関連するデータの断片を互いに結びつけておくことができ、それに名前をつけてコードをわかりやすくすることができます。`impl`ブロックは、型に関連する関数を定義することができ、メソッドは関連する関数の一種で、構造体のインスタンスが持つ振る舞いを指定することができます。

しかし、カスタム型を作成する方法は構造体だけではありません。次の章では`enum`について説明します。
