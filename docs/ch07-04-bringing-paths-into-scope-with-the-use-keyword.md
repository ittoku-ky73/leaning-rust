# useキーワードでパスをスコープに入れる

> Ref: https://doc.rust-lang.org/stable/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html

関数を呼び出すのに毎回パスを書くのは面倒です。以下のコードでは、`add_waitlist()`を呼び出すたびに、`front_of_house::hosting::add_to_waitlist`と入力しなければならず、非常に不便で可読性も下がります。
`use`キーワードを使うことによって、パスのショートカットを作成し、スコープ内のすべての場所で短く読みやすい名前で呼び出しを行うことが出来ます。

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

スコープにパスを追加することは、ファイルシステムにシンボリックリンクを作成することに似ています。
`use crate::front_of_house::hosting`をクレートルートに追加することで、`hosting`という短いコードで呼び出すことができるようになります。

`use`は特定のスコープのショートカットのみを作成します。そのため関数本体はコンパイルされません。

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
```

上記のコードはエラーになります。`customer`モジュールで囲まれたことにより、`hosting`ショートカットを利用できなくなったためです。

## 慣用的な`use`パスの作成

パスは、以下のように関数に対しても指定することが出来ます。

```rust
use crate::front_of_house::hosting::add_to_waitlist;
```

しかし一般的には、`use`キーワードは使用する関数の親モジュールを指定します。
こうすることで完全パスの繰り返しを最小限に抑えながら、関数がローカルで定義されていないことが明確になります。

一方、構造体、列挙型、その他項目を`use`で持ち込む場合は、フルパスを指定するのが慣用的です。
以下の`HashMap`は、標準ライブラリの構造体をバイナリクレートのスコープに持ち込む慣用的な方法を示しています。

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

これは単なる慣例であり、Rustaceansはこのように読み書きすることに慣れています。

このイディオムの例外は、同じ名前の2つの項目を`use`ステートメントでスコープに持ち込む場合です。Rustではそれは許可されていません。
以下の例は、名前が同じだが親モジュールが異なる2つの肩をスコープに入れる方法と、参照する方法を示しています。

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```

上記のコードでもし、`use std::fmt::Result;`と定義した場合、どうなるでしょう。Resultというスコープが2つになりどちらがどちらのResultなのかRustは認識できなくなります。

## `as`キーワードによる新しい名前の提供

同じ名前の2つの型を同じスコープに入れる問題には、もう1つ別の解決策があります。
それは、パスの後に`as`を指定して、型の新しいローカル名、またはエイリアスを指定することです。
以下の例では、`as`を使った2つのResult型のうちの1つの名前を変更したものです。

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

## `pub use`で名前を再エクスポートする

`use`キーワードを使用してスコープに入れると、新しいスコープで使用できる名前は非公開になります。`pub`と`use`キーワードを組み合わせることで、スコープと公開を行うことが出来ます。

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

`pub use`を使うことによって、呼び出しを行う際に、外部コードの代わりにパスを使用できるようになります。
つまり、`restaurant::front_of_house::hosting::add_to_waitlist()`と呼び出すところを、`restaurant::hosting::add_to_waitlist()`と呼び出すことができるようになるということです。

再エクスポートは、コードの内部構造がコードを呼び出すプログラマがドメインについて考える方法と異なる場合に役立ちます。
例えば、レストランを経営している人は、表と裏について考えています。しかしレストランに訪れる顧客は、おそらくレストランの各部分をそのように考えることがありません。
`pub use`を使用すると、コードを1つの構造で記述しながら、別の構造を公開できます。
そうすることで、ライブラリで作業するプログラマとライブラリを呼び出すプログラマのために、ライブラリが適切に編成されます。

## 外部パッケージの使用

第2章では、乱数を取得するために`rand`パッケージを使用して、推測ゲームをプログラミングしました。このプロジェクトでも`rand`パッケージを使用したいのでCargo.tomlに以下のコードを追加しましょう。

```toml
rand = "0.8.5"
```

Cargo.tomlに`rand`を依存関係として追加すると、Cargoはパッケージとすべての依存関係を、[crates.io](https://crates.io/)からダウンロードし、プロジェクトで利用できるようになります。

次に、`use rand::Rng`を定義し、パッケージをスコープに入れます。これは第2章でもやりました。

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

Rustコミュニティのメンバーは、多くのパッケージをcrates.ioで利用できるように作成しており、それらのパッケージを取り込むには、Cargo.tomlにパッケージをリストし、`use`を使用して、クレートからアイテムをスコープして取り込むという手順が必要です。

`std`標準ライブラリは、外部パッケージのクレートとは違います。。標準ライブラリはRust言語に同梱されているため、Cargo.tomlに追加する必要はありません。しかし、そのライブラリを使用するには、`use`を使って定義する必要があります。

```rust
use std::collections::HashMap;
```

## ネストされたパスで大規模な`use`を整理する

同じパッケージ、モジュールで定義された複数の項目を使用する場合、それぞれの項目を独自の行にリストすることで、そのライブラリのさまざまな機能を使うことが出来ます。

```rust
use std::cmp::Ordering;
use std::io;
```

また、以下のように括弧で囲むことでも複数の項目の機能を使うことが出来ます。以下の定義は上の定義と等価です。

```rust
use std::{cmp::Ordering, io};
```

大きなプログラムでは、ネストしたパスを使用して同じクレートアモジュールから多くのアイテムをスコープに取り込むことが出来、`use`ステートメントの数を大幅に減らすことが出来ます。

パスはどのレベルでも使用することができるので、サブパスを共有する2つの`use`文を組み合わせて使うこともできます。

```rust
use std::io;
use std::io::Write;
```

また以下のように書くこともできます。

```rust
use std::io::{self, Write};
```

## Glob演算子

パッケージのすべてのアイテムをスコープに入れたい場合は、パスの後に`*`グロブ演算子を使用します。

```rust
use std::collections::*;
```

このグロ部演算子を使う際は注意が必要です。この演算子を使うと、どの名前がスコープ内にあるのか、定義元がどこにあるのかが分かりにくくなってしまうからです。

`tests`グロブ演算子は、テスト中の全てをモジュールに取り込むテストをする時によく使用されます。これについては第11章で詳しく見ていきます。