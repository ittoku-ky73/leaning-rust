# 構造体の定義とインスタンス化

> 参考：https://doc.rust-lang.org/book/ch05-01-defining-structs.html

構造体は、タプル型で説明したタプルに類似しており、どちらも関連する複数の値を保持します。タプルとは異なり、構造体では各データに名前をつけて、値が何を意味するのかを明確にします。名前をつけることで構造体はタプルよりも柔軟性が高くなり、データの順序に依存せずにインスタンスの値を指定したりアクセスすることができます。

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

構造体を定義するには、キーワード`struct`を入力し、構造体全体に名前をつけます。構造体の名前は、グループ化されているデータの重要性を示すものでなければなりません。次に、括弧の中にフィールドと呼ばれるデータの名前と型を定義します。

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

構造体から特定の値を取得するには、ドット記法を使用します。上記の例だと、メールアドレスは、`user1.email`と書きます。インスタンスが`mutable`の場合、ドット記法を使用して値を変更することができます。

```rust
let mut user1 = User {
    email: String::from("someone@example.com"),
    // ...
};

user1.email = String::from("anotheremail@example.com");
```

Rustでは、特定のフィールドだけをミュータブルとしてマークすることはできません。関数本体の最後の式で構造体の新しいインスタンスを作成し、その新しいインスタンスを暗黙的に返すことができます。

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

関数のパラメータに構造体のフィールドと同じ名前をつけるのは理にかなっていますが、電子メールとユーザー名のフィールド名と変数を繰り返さなければならないのは、少し面倒です。嬉しいことにRustにはこれについての便利な省略記法があります。

## Field Initのショートハンドを使う

上記の`build_user`関数は以下のように省略して書くことができます。

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

## 構造体の更新構文

構造体の新しいインスタンスを作成する際、他のインスタンスの値から作成し、一部変更すると便利なことがよくあります。このような場合は、構造体の更新構文を使用します。

```rust
let user2 = User {
    active: user1.active,
    username: user1.username,
    email: String::from("another@example.com"),
    sign_in_count: user1.sign_in_count,
};
```

構造体の更新構文を使用すると、上記のコードをより少ないコードで書くことができます。下記のコードは、`user1`の構造体から、電子メールの値のみが違う`user2`という新しい変数を作成しています。

```rust
let user2 = User {
  email: String::from("another@example.com"),
  ..user1
};
```

これは、変数とデータの相互作用：移動で見たように、データを移動させるためです。上記だと`user1`変数を`user2`変数に渡しているので`user1`は使えなくなります。

## タプル構造体から異なる型を生成

Rustでは、タプル構造体と呼ばれる、タプルに似た構造体もサポートしています。タプル構造体は、フィールドに名前が関連づけられておらず、フィールドの型が示されているだけです。タプルに名前をつけて他のタプルと異なる型にしたい場合や、通常の構造体のように各フィールドに名前をつけると冗長になる場合に便利です。

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

`black, origin`は異なるタプル構造体のインスタンスなので、型が違います。構造体のフィールドが同じ型であっても、定義した構造体はそれぞれ独自の型になります。例えば、Color型のパラメータを受け取る関数は、引数としてPoint型を受け取ることはできません。

## フィールドを持たない構造体

ユニット型構造体は、ある型の特徴を実装する必要がありますが、その型自体に格納したいデータがない場合に便利です。詳しくは第10章で説明します。

```rust
struct AlwaysEqual;

fn main() {
  let subject = AlwaysEqual;
}
```

のちにこの型の動作を実装して、`AlwaysEqual`の全てのインスタンスが他の全ての型のインスタンスと常に等しくなるようにすることを想像してください。そのような動作を実装するためのデータは必要ありません。第10章では、ユニット型構造体を含む任意の型に`trait`を定義して実装する方法を説明します。

> **構造体の所有権**
>
> User構造体の定義で、`&str`文字列スライス型ではなく、所有権を持つString型を使用しています。これはこの構造体の各インスタンスがその全てのデータを所有し、構造体全体が有効である限り、そのデータが有効であるようにするため、意図的に選択したものです。
>
> 構造体は、他のものが所有するデータへの参照を格納することもできますが、そのために第10章で説明するライフタイムを使用する必要があります。ライフタイムは、構造体が参照するデータが構造体が存在する限り有効であることを保証します。例えば、次のようにライフタイムを指定せずに構造体に参照を格納しようとすると、うまくいきません。
>
> ```rust
> struct User {
>     active: bool,
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
> }
> 
> fn main() {
>     let user1 = User {
>         email: "user1@example.com",
>         username: "user1",
>         active: true,
>         sign_in_count: 1,
>     };
> }
> ```
>
> コンパイルしようとすると、次のようなエラーが出ます。
>
> ```shell
> cargo run
>    Compiling struct_introduction v0.1.0 (projects/struct_introduction)
> error[E0106]: missing lifetime specifier
>   --> src/main.rs:18:15
>    |
> 18 |     username: &str,
>    |               ^ expected named lifetime parameter
>    |
> help: consider introducing a named lifetime parameter
>    |
> 16 ~ struct User<'a> {
> 17 |     active: bool,
> 18 ~     username: &'a str,
>    |
> 
> error[E0106]: missing lifetime specifier
>   --> src/main.rs:19:12
>    |
> 19 |     email: &str,
>    |            ^ expected named lifetime parameter
>    |
> help: consider introducing a named lifetime parameter
>    |
> 16 ~ struct User<'a> {
> 17 |     active: bool,
> 18 |     username: &str,
> 19 ~     email: &'a str,
>    |
> 
> For more information about this error, try `rustc --explain E0106`.
> error: could not compile `struct_introduction` due to 2 previous errors
> ```
>
> 第10章では、構造体に参照を格納できるようにこれらのエラーを修正する方法を説明しますが、今のところ、`&str`のような参照ではなく、Stringのような所有型を使ってこれらのエラーを修正することになります。