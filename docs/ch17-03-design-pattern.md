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

オブジェクト指向のステートパターンを実装することで状態によって記事に異なる振る舞いをカプセル化することができました。
`Post`のメソッドは種々の振る舞いについて何も知りません。
コードを体系化する仕方によれば、公開された記事が振る舞う方法を知るには、1箇所のみを調べるだけで済みます。

ステートパターンを使用しない実装なら、状態を確認するために`match`式を使用するかもしれません。
その場合、複数箇所を調べて記事が公開状態にあることの裏の意味全てを理解する必要があります。
これは追加した状態が増えるほど、理解が難しくなることを意味します。

ステートパターンでは`Post`のメソッドと使用する箇所で`match`式が必要になることはなく、新しい状態を追加するときは新しい構造体を追加し、その1つの構造体にトレイトメソッドを実装するだけで済みます。

ステートパターンを使用した実装は、拡張して機能を増やすことが容易です。
ステートパターンを使用するコードの管理の単純さを確認するために、以下の提言を試して見てください。

- [x] 記事の状態を`PendingReview`から`Draft`に戻す`reject`メソッドを追加する。
- [x] 状態が`Published`に変化する前に`approve`を2回呼び出すようにする。
- [x] 記事が`Draft`状態の時のみテキスト内容をユーザーが追加できるようにする。

ステートパターンの欠点は、状態が状態間遷移を実装していることにより、状態の一部が結合した状態になってしまうことです。
`PendingReview`と`Published`の間に、`Scheduled`のような別の状態を追加すると、代わりに`PendingReview`のコードを`Scheduled`に遷移するように変更しなければいけません。
状態が追加されても`PendingReview`を変更する必要がなければ作業量は減りますが、別のデザインパターンに切り替えることになります。

別の欠点にロジックの一部を重複させてしまうことです。
重複を除くためには`State`トレイトの`request_review, approve`メソッドに`self`を返すデフォルト実装を試みる可能性があり、これはオブジェクトの安全性を侵害します。
というのも具体的な`self`が一体何なのかトレイトには知る方法がないからです。
`State`をトレイトオブジェクトとして使用できるようにするのであれば、メソッドにはオブジェクト安全になってもらう必要があります。

他の重複には`Post`の`request_review, approve`メソッドの実装が似ていることが含まれます。
両メソッドは`Option`の`state`の値に対する同じメソッドの実装に移譲しており、`state`フィールドに新しい値をセットします。
このパターンに従う`Post`のメソッドが多くあれば、マクロを定義して繰り返しを排除することも考慮するも大事です。

オブジェクト指向言語で定義されている通りに忠実にステートパターンを実装しても、Rustの強みを発揮しません。
`blog`クレートに対して行うことのできる無効な状態と繊維をコンパイルエラーにできる変更を見ていきましょう。

### 状態と振る舞いを型としてコード化

ステートパターンを再考して別の方法を見ていきます。
状態と遷移を完全にカプセル化して、外部のコードに知らせないようにするよりも、状態を異なる型にコード化します。
結果的にRustの型検査システムが公開記事にみが許可される箇所で草稿記事の使用を試みるとコンパイルエラーになるようにします。

以下の`main`の最初の部分について考えて見ましょう。

```rust
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
}
```

`Post::new`で草稿状態の新しい記事を生成し、内容を追加することができます。
しかし空の文字列を返す草稿記事の`content`メソッドを定義しない場合、草稿状態の記事から`content`メソッドを呼び出そうとするとコンパイルエラーになります。
こうすることで誤ってプロダクションコードで草稿記事の内容を表示することがなくなります。
以下のコードでは`Post, DraftPost`構造体と、メソッドを定義しています。

```rust
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}
```

`Post, DraftPost`のどちらにも非公開の`content`フィールドがあります。
状態のコードかを構造体の型に移動したので、`state`フィールドは必要なくなりました。
`Post`は公開された記事を表し、`content`メソッドを持ちます。

`Post::new`関数は`DraftPost`のインスタンスを返すので、`Post`のインスタンスは今の所生成することができません。

`DraftPost`には`add_text`メソッドがありますが`content`メソッドはありません。
これによって全ての記事が草稿記事から始まり、草稿記事は表示できる内容がないことを保証します。
この制限を無視する実装はコンパイルエラーになるでしょう。

### 遷移を異なる型の変形として実装する

公開された記事は、草稿記事を査読し、承認されて、初めて公開されます。
査読待ち状態の記事は内容を表示するべきではありません。
そこで`PendingReviewPost`構造体を追加し、`DraftPost`に`PendingReviewPost`を返す`request_review`メソッドを定義し、`PendingReviewPost`に`Post`を返す`approve`メソッドを定義します。

```rust
impl DraftPost {
    // --snip--
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
```

`request_review, approve`メソッドは`DraftPost, PendingReviewPost`インスタンスを消費し、それぞれ`PendingReviewPost, Post`に変形します。
`content`メソッドが公開された`Post`インスタンスを得る唯一の方法が`PendingReviewPost`に対して`approve`を呼び出すことで、`PendingReviewPost`を得る唯一の方法が`DraftPost`に対して`request_review`を呼び出すことです。
これでブログ記事のワークフローを型システムにコードかすることに成功しました。

ですがまだ完成ではなく、呼び出し側のコードに少し変更を加える必要があります。
`request_review, approve`メソッドは呼ばれた構造体を変更するのではなく、新しいインスタンスを返すので、`let post =`というシャドーイング代入を追加し、返却されたインスタンスを保存する必要があります。
また、走行と査読待ち記事のアサートも必要なくなります。
`main`の更新されたコードは以下のとおりです。

```rust
extern crate blog;
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a gyudon for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a gyudon for lunch today", post.content());
}
```

上記の変更はオブジェクト指向のステートパターンに沿っていないことを意味します。
状態間の変形は`Post`内でカプセル化されていませんが、Rustの型システムとコンパイル時の型チェックのおかげで無効な状態を判断することができます。
これにより未公開の記事の内容が表示されるバグをプロダクションコードに移る前に発見されることが保証されます。

`blog`クレートに関して「ステートパターンの代償」の提言された作業を試して見てください。
この設計ではすでに作業の一部が達成されています。

Rustはオブジェクト指向のデザインパターンを実装する能力はあるものの、状態を型システムにコード化するなどのパターンも、Rustで使えることがわかりました。
これらのパターンには異なる代償があります。
オブジェクト指向のパターンは、オブジェクト指向言語にはない所有権などの特定の機能によりRustでは必ずしも最残の解決策ではないでしょう。

### まとめ

この章では、Rustでオブジェクト指向プログラミングが可能であり、それを行う方法を学びました。
オブジェクト指向パターンは必ずしもRustの強みを活かす最善の方法にはなりませんが、利用可能な選択肢の1つではあります。

次はパターンを見ていきます。
パターンも多くの柔軟性を可能にするRustの機能です。
今まででも使ってはきましたが、次は本格的にそれを学んでいきます。
