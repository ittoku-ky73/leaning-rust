# オブジェクト指向デザインパターン

> Ref: https://doc.rust-jp.rs/book-ja/ch17-03-oo-design-patterns.html

ステートパターンは、オブジェクト指向デザインパターンの1つです。
このパターンは値が一連のステートオブジェクトで表す内部状態を持ち、その内部状態に基づいて値の振る舞いが変化するというものです。
ステートオブジェクトは機能を共有します。Rustでは構造体とトレイトを使用します。
各ステートオブジェクトは自身の振る舞いと別の状態に変化すべき時に責任を持ちます。
ステートオブジェクトを保持する値は、状態ごとの異なる振る舞いや、いつ状態が移行するか何も知りません。

ステートパターンを使用することはプログラムの業務要件が変わる時、状態を保持する値のコードや、値を使用するコードを変更する必要がないことを意味します。
ステートデザインパターンの例と、Rustでの使用方法を見ていきましょう。

例のブログ記事のワークフローは次の通りです。

1. 空の草稿を作成する
2. 査読機能を追加する
3. 記事が承認されたら公開する
4. 公開された記事のみを表示して、未承認の記事は公開しない

それ以外の記事に対する変更は効果をもつべきではありません。
例えば、査読を要求する前にブログ記事の草稿を承認すると、記事は非公開の草稿のままになるべきです。

以下のコードは上記のワークフローに則って作成したライブラリクレートに実装するAPIの例です。
まだコンパイルはできません。

```rust
extern crate oop;
use oop::Post;

let mut post = Post::new();

post.add_text("I ate a salad for lunch today");
assert_eq!("", post.content());

post.request_review();
assert_eq!("", post.content());

post.approve();
assert_eq!("I ate a salad for lunch today", post.content());
```

ユーザーが`Post::new`で新しいブログ記事の草稿を作成します。
そして草稿状態の間にブログ記事にテキストを追加できるようにします。
承認前に記事の内容を即座に得ようとしたら、記事はまだ走行なので何も起きません。
デモ目的でコードに`assert_eq!`を追加しています。
この単体テストは、ブログ記事の走行が`content`メソッドから空の文字列が返ってくることをテストします。

次に記事の査読を要求し、査読を待機している間`content`に空の文字列を返します。
記事が承認されたら公開し、`content`を呼び出した時に記事のテキストが返されるということです。

クレートから相互作用している唯一の型は`Post`のみです。
この型はステートパターンを使用し、記事がなり得る種々の状態を表す「草稿、査読待ち、公開中」のステートオブジェクトのうち1つになる値を保持します。
状態の変更には`Post`型内部で管理します。
`Post`インスタンスのライブラリ使用者が状態の変化を直接管理する必要はありません。
またユーザーは査読前に記事を公開するなど状態を誤ることもないでしょう。

### Postを定義

ライブラリの実装に取り掛かりましょう！
何らかの内容を保持する公開の`Post`構造体が必要なので、構造体の定義と関連する公開用の`Post`インスタンスを生成する`new`関数から始めましょう。
また非公開の`State`トレイトも作成します。それから、`Post`は`state`という非公開のフィールドに、`Option`で`Box<State>`のトレイトオブジェクトを保持します。

```rust
pub struct Post {
    state: Option<Box<State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
}

trait State {}

struct Draft {}

impl State for Draft {}
```

`State`トレイトは異なる記事の状態で共有される振る舞いを定義し、`Draft, PendingReview, Published`状態は全て、`State`トレイトを実装します。
今はトレイトにメソッドは何もなく、`Draft`が記事の初期状態にしたい状態なので、その状態だけを定義することから始めます。

新しい`Post`を作る際、`state`フィールドは`Box`を保持する`Some`値にセットします。
この`Box`が`Draft`構造体の新しいインスタンスを指します。
これにより新しい`Post`を作るたびに、草稿から始まることが保証されます。
`Post`の`state`フィールドは非公開なので、`Post`を他の状態で作成する方法はないのです。
`Post::new`関数では、`content`フィールドを新しい空の`String`をセットします。

## 記事の内容を格納

以下のコードは`add_text`というメソッドを呼び出し、ブログ記事のテキスト内容に追加される`&str`を渡せるようにします。
これを`pub content`フィールドにするのではなくメソッドとして実装します。
これは`content`フィールドデータの読み込みを制御するメソッドを実装できることを意味しています。
`add_text`メソッドは非常に素直なので、`impl Post`ブロックに追加して見ましょう。

```rust
impl Post {
    // --snip--

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

`add_text`メソッドは、`self`の可変参照を取ります。これは`Post`インスタンスを変更するためです。
そして`content`から`push_str`を呼び出し、`text`引数を渡して保存された`content`に追加します。
この振る舞いは記事の状態によらないのでステートパターンの一部ではありません。
`add_text`メソッドは`state`フィールドと全く相互作用しませんが、サポートしたい振る舞いの一部ではあります。

### 草稿の記事の内容を保証する

`add_text`を呼び出して記事に内容を追加した後でさえ、記事はまだ草稿状態なので、`content`メソッドには空の文字列スライスを返すようにします。
そのためにまず常にからの文字列スライスを返す`content`メソッドを実装しましょう。

```rust
impl Post {
    pub fn content(&self) -> &str {
        ""
    }
}
```

### 記事の査読を要求する

次に記事の査読を要求する機能を追加します。
これは状態が`Draft`から`PendingReview`に変更します。

```rust
impl Post {
    // --snip--
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
```

`Post`に`self`の可変参照を取る`request_review`という公開メソッドを追加します。
そして`Post`の現在の状態に対して内部の`request_review`メソッドを呼び出し、このメソッドが現在の状態を消費し、新しい状態を返します。

`State`トレイトに`request_review`メソッドを追加します。
このトレイトを実装する型は全て、`request_review`メソッドを実装する必要があります。
メソッドの第1引数に`self, &self, &mut self`ではなく`self: Box<Self>`にしています。
この記法は型を保持する`Box`に対して呼ばれた時のみ、このメソッドが合法になることを意味しています。
この記法は`Box<Self>`の所有権を奪い、古い状態を無効化にするので`Post`の状態値は常に新しい状態に変形します。

古い状態を消費するために`request_review`メソッドは状態地の所有権を奪う必要があります。
ここで`Post`の`state`フィールドの`Option`が問題になります。
`take`メソッドを呼び出して`state`フィールドから`Some`値を取り出し、その箇所に`None`を残します。
なぜならRustは、構造体に未代入のフィールドを持つことができないからです。
これにより借用するのではなく`Post`の`state`値をムーブすることができます。
それから記事の`state`値をこの処理の結果にセットします。

`self.state = self.state.request_review()`のようなコードで`state`値の所有権を得るように設定するのではなく、一時的に`None`に`state`をセットする必要があります。
これにより新しい状態に変形した後に`Post`が古い`state`値を使えないことが保証されます。

`Draft`の`request_review`メソッドは新しい`PendingReview`構造体の新しいボックスのインスタンスを返す必要があり、記事が査読街の状態を表します。
`PendingReview`構造体も`request_review`メソッドを実装しますが、何も変形はしません。
というのもすでに`PendingReview`状態にある記事の査読を要求したら`PendingReview`状態に留まるべきだからです。

ようやくステートパターンの利点が見えてき始めました。
`state`値が何であれ、`Post`の`request_review`メソッドは同じです。
各状態は独自の規則にのみ責任を持ちます。

`Post`の`content`メソッドを空の文字列スライスを返してそのままにします。
これで`Post`は`PendingReview, Draft`状態になり得ますが、`PendingReview`状態でも同じ振る舞いを要求します。

### 振る舞いを変化させる

`approve`メソッドは`request_review`メソッドと似ています。
状態が承認された場合、現在の状態があるべきという値に`state`をセットします。

```rust
impl Post {
    // --snip--
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    // --snip--
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    // --snip--
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
```

`State`トレイトに`approve`メソッドを追加し、`Published`状態という`State`を実装する新しい構造体を追加します。

`request_review`のように`Draft`に対して`approve`メソッドを呼び出したら`self`を返すので何も効果はありません。
`PendingReview`に対して`approve`を呼び出すと、`Published`構造体の新しいボックス化されたインスタンスを返します。
`Published`構造体は`State`トレイトを実装し、`request_review, approve`メソッド両方に対して`self`を返します。
そのような場合に記事は`Published`状態に留まるべきだからです。

これで`Post`の`content`メソッドを更新する必要が出てきました。
状態が`Published`なら、記事の`content`フィールドの値を返すようにします。
それ以外なら空の文字列スライスを返します。

```rust
impl Post {
    // --snip--
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    // --snip--
}
```

目的はこれらの規則全てを`State`を実装する構造体の内部に押し留めることなので`state`の値に対して`content`メソッドを呼び出し、記事のインスタンスを引数として渡します。
そして`state`値の`content`メソッドを使用したことから帰ってきた値を返します。

`Option`に対して`as_ref`メソッドを呼び出します。値の所有権ではなく`Option`内部の値への参照が欲しいからです。
`state`は`Option<Box<State>>`なので`as_ref`を呼び出すと`Option<&Box<State>>`が返ってきます。
`as_ref`を呼ばなければ`state`を関数引数の借用した`&self`からムーブできないのでエラーになります。

さらに`unwrap`メソッドを呼び出し、これは絶対にパニックしないことがわかっています。
なぜなら`Post`のメソッドがそれらのメソッドが完了した際に`state`は常に`Some`値を含んでいることを保証するからです。
これはコンパイラには理解できません。

この時点で`&Box<State>`に対して`content`を呼び出すと、参照外し型強制が`&, Box`に働くので、`content`メソッドが`State`トレイトを実装する型に対して呼び出されることになります。
つまり、`content`を`State`トレイト定義に追加する必要があり、そこが現在の状態に応じてどの内容を返すべきかというロジックを配置する場所です。

```rust
trait State {
    // --snip--
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

// --snip--

struct Published {}

impl State for Published {
    // --snip--
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
```

空の文字列スライスを返す実装を`content`メソッドに追加しています。
これにより`Draft, PendingReview`構造体の`content`メソッドは常に空の文字列スライスを返します。

このメソッドにはライフタイム注釈が必要になります。
`post`の参照を引数として取り、`post`の一部の参照を返すので、返却される参照のライフタイムは、`post`引数のライフタイムに関連します。

これでブログ記事ワークフローの規則でステートパターンを実装できました！
この規則に関連するロジックは`Post`中に散乱するのではなくステートオブジェクトによるものです。

### ステートパターンの代償
