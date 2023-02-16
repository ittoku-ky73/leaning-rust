# Resultでエラーをリカバリー

> Ref: https://doc.rust-lang.org/stable/book/ch09-02-recoverable-errors-with-result.html

ほとんどのエラーは、プログラムが完全に停止しなければならないほど深刻なものではありません。
関数が失敗したとき、その理由を簡単に解釈して対応できることがあります。
例えば、ファイルを開こうとしたときに、ファイルが存在しないためにその操作が失敗した場合、プロセスを修了するのではなく、ファイルを作成したいと思うかもしれません。

第2章の、Resultによる潜在的な失敗の処理で、Result列挙型が以下のように`Ok, Err`の2つのバリアントを持つように定義しました。

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T, E`は、ジェネリックな型パラメータです。ジェネリックについては第10章で説明します。
`T`は成功した場合に返される値の型を表し、`E`は失敗した場合に返されるエラーの型を表しています。
Resultはこのように汎用的な型パラメータを持っているので、成功した時の値とエラー時の値が異なるさまざまな状況でResult型とその上で定義された関数を使用することが出来ます。

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
}
```

`File::open`の戻り値は`Result<T, E>`です。汎用パラメータ`T`は、File::openの実装によって成功値の型である`std::fs::File`で埋め尽くされています。
エラー値に使われている`E`の型は`std::io::Error`です。この戻り値の型は、`File::open`の呼び出しに成功し、読み書き可能なファイルが返されるかもしれないことを意味します。
例えば、ファイルが存在しなかったり、アクセス権限がなかったりした場合、`File::open`関数には、成功したか失敗したか知らせる方法と、ファイルハンドルかエラー情報を与える必要があります。
それはまさに`Result enum`が伝えるものです！

`File::ope`が成功した場合、`greeting_file_result`の値は、ファイルハンドルを含む`Ok`のインスタンスになります。
失敗した場合、`greeting_file_result`の値はErrのインスタンスで、発生したエラーの種類に関する詳細な情報を含んでいます。

File::openは、返す値によって異なるアクションになるため、上記のコードだけでは不十分です。
ここは第6章で説明した基本的なツールである`match`式を使用して、`Result`を処理する方法をとってみましょう。

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }
}
```

Option列挙型と同様に、Result列挙型とその変種はPreludeによってスコープに入っているため、`match`アーム内で、`Result::`と指定する必要はありません。

結果が`Ok`の場合、ファイル値を返し、その値を`greeting_file`に代入します。これでファイルハンドルを読み書きのために使うことが出来ます。

結果が`Err`の場合、`panic!`マクロを呼び出し、以下のような出力を行います。

```bash
cargo run
   Compiling result v0.1.0 (/Users/ittoku/Documents/learning/rust/projects/result)
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/result`
thread 'main' panicked at 'Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:8:23
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## 違うエラーをマッチさせる

`File::open`はなぜ失敗したかに関係なく、`panic!`になります。
ファイルが存在しないためにFile::openが失敗した場合、ファイルを作成し、新しいファイルへのバンドルを返したい場合や、ファイルを開く権限がなかったときに`panic!`になるようなコードを書くには次のように書きます。

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFount => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", error),
            }
        },
    };
}
```

File::openがErrバリアント内で返す値の型はio::Errorで、これは標準ライブラリで提供される構造体です。
この構造体は、`io::ErrorKind`値を取得するために呼び出すことのできる`kind`を持っています。
`enum io::ErrorKind`は標準ライブラリで提供されており、`io`操作から生じる可能性のあるさまざまな種類のエラーを表すバリアントを持っています。
ここで使用しているのは`ErrorKind::NotFound`で、これは開こうとしているファイルがまだ存在しないことを示しています。

`match`式でチェックしたい条件は、`error.kind()`が返す値が`ErrorKind enum`の`NotFound`であるかどうかです。
もしそうなら、File::createでファイルを作成します。しかし、File::createも失敗する可能性があるので、`match`式が必要です。
`match`式の1つ目はファイルを開くことができるか、2つ目はなぜファイルを開けないのか、3つ目はファイルを作成できるかをみています。

> **`Result<T, E`と`match`の組み合わせの代替案**
>
> `match`するケースが多いときに`match`式は便利ですが、同時にとても原始的でもあります。第13章ではクロージャについて学びます。
> クロージャは`Result<T, E>`に定義されている多くのメソッドで使用されます。これらのメソッドを使うことで、`match`式を使うよりも簡潔にコードを書くことが出来ます。
>
> ```rust
> use std::fs::File;
> use std::io::ErrorKind;
> 
> fn main() {
>     let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
>         if error.kind() == ErrorKind::NotFound {
>             File::create("hello.txt").unwrap_or_else(|error| {
>                 panic!("Problem creating the file: {:?}", error);
>             })
>         } else {
>             panic!("Probrem opening the file: {:?}", error);
>         }
>     });
>  }
> ```
>
> 上記のコードは、`match`式を含んでいないので読みやすくなっています。標準ライブラリの`unwrap_or_else`メソッドは、エラーを処理するときに巨大なネストされた`match`式をクリーンアップできます。

## エラー時のパニックのショートカット、`unwrap, expect`

`match`式は便利ですが少し冗長になりがちで意図がうまく伝わらないことがあります。`Result<T, E>`型にはさまざまな、より具体的なタスクを実行するために、多くのヘルパーメソッドが定義されています。
`unwrap`メソッドは、`match`式と同じように実装されたショートカットメソッドです。
Result値が`Ok`バリアントである場合、`unwrap`はOkの中の値を返します。

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```

Result値が`Err`バリアントである場合、`unwrap`は以下のようなパニックを出力します。

```bash
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value:
Os { code: 2, kind: NotFound, message: "No such file or directory" }',
src/main.rs:38:53
```

同様に、`expect`メソッドでは`panic!`エラーメッセージも選択できます。
`unwrap`の代わりに`expect`を使用し、エラーメッセージを提供することで明確に意図を伝えることが出来、パニックの原因を突き止めることが容易になります。

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```

実運用に耐える品質のコードでは、ほとんどのRustaceansは`unwrap`ではなく`expect`を選択し、なぜその操作が常に成功すると期待されるのかについて、より多くの文脈を与えています。
そうすることで、仮にその過程が間違いであることが証明された場合、デバッグに使える情報をより多く得ることが出来ます。

## エラーの伝搬

関数の実装が失敗する可能性のあるものを呼び出す場合、関数自体の中でエラーを処理するのではなく、呼び出し元のコードにエラーを返し、呼び出し元がどうすべきかを決定できるようにすることが出来ます。
これは*propagating the error*と呼ばれ、呼び出し側のコードにエラーの処理方法を決定する情報やロジックが、コードのコンテキストで利用可能なものより多く存在する可能性があるため、より制御しやすくなります。

以下のコードは、ファイルからユーザー名を読み取る関数を示しています。ファイルが存在しないか、読み取れない場合、この関数は関数を呼び出したコードにこれらのエラーを返します。

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

この関数はもっと短く書くことが出来ますが、エラー処理を調べるために、まず多くの部分を手作業で書いています。
まず、この関数の戻り値の型は`Result<String, io::Error>`になっています。これは`Result<T, E>`型の値を返すことを意味します。
一般パラメータ`T`には具体型`String`が、一般パラメータ`E`には具体型`io::Error`が埋め込まれています。

この関数が問題なく成功した場合、ファイルから読み取ったユーザ名（String）を保持する`Ok`値を受け取ります。
この関数が何かしらの問題で失敗した場合、問題が何であったかについての詳細な情報を含む`io::Error`のインスタンスを保持する`Err`値を受け取ります。

この関数の戻り値の型として`io::Error`を選んだのは、この関数の本体で読んでいるFile::open関数とread_to_stringメソッドという失敗するかもしれない操作から返されるエラー値の型がたまたま`io::Error`になっているからです。

この関数ではまず`File::open`関数を呼び出し、`match`式でResultの値を処理しています。
成功すると、ミュータブルな`username_file`に結果が代入され関数が続行されます。
失敗すると、`panic!`の代わりに、`return`キーワードで関数の戻り値としてエラー値を返します。

`username_file`にファイルハンドルがある場合、`username`に新しいStringを作成し、`username_file`のファイルハンドルに対して、`read_to_string`メソッドを呼び出してファイルの内容を`username`に読み込ませます。
成功すれば、`username`にファイルの内容を代入します。
失敗すれば、ファイルが読み込めなかったエラー値を返します。これは関数内の最後の式なので、明示的に`return`とつける必要はありません。

このコードを呼び出したコードは、ユーザ名を含むOk値か、`io::Error`を含むErr値のどちらかを取得します。
これらの値をどうするかは、呼び出し側のコード次第です。もし呼び出し側のコードがErr値を取得したとすれば、以下のような処理が選択肢としてあります。

- panic!マクロを呼び出して、クラッシュさせる。
- デフォルトのユーザ名を使い、ファイル以外のどこかからユーザ名を探したりする

このようにエラーが伝播するパターンはRustではよくあります。Rustではこれを容易にするための`?`演算子というものがあります。

## エラーの伝播ショートカット、?演算子

以下のコードは、`?`演算子を使った`read_username_from_file`関数です。

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

`Result`の値の後に置かれた`?`は、Resultの値を処理するために定義した`match`式とほぼ同じように動作するように定義されています。
Result値がOkなら、Okの中の値がこの式から返され、プログラムは続行されます。
Result値がErrなら、`return`キーワードを使用したかのように関数全体から返されるので、エラー値は呼び出し側のコードに伝搬されます。

`match`式と`?`演算子が行うことには違いがあります。`?`演算子が呼ばれたエラー値は、標準ライブラリの`From`特性で定義されている`from`関数を経由して、ある型から別の型に値を変換するために使用されます。
演算子が`from`関数を呼び出すと、受け取ったエラー型は現在の関数の戻り値の型に定義されているエラー型に変換されます。
これは関数が1つのエラー型を返すときに、関数が失敗する可能性のあるすべての方法を表すことができ、たとえ部品が多くの異なる理由で失敗したとしても、便利です。

例えば、read_username_from_file関数を、個人で定義した`OurError`というカスタムエラータイプを返すように変更することが出来ます。
また、`io::Error`から`OurError`のインスタンスを構築するために`OurError`に対して`impl From<io::Error>`を定義すると、read_username_from_file本体の`?`演算子の呼び出しは、関数にコードを追加することなくエラータイプの変換を行います。

コンテキストでは、File::open呼び出しの最後にある`?`は、`username_file`にOk内の値を返します。エラーが発生した場合、`?`演算子は`return`し、Err値を与えます。

`?`演算子を使うことで、多くの定型句がなくなり、この関数の実装がよりシンプルになります。`?`の直後にメソッド呼び出しを連結することで、このコードをさらに短くすることもできます。

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username);
    Ok(username)
}
```

`username`の新しいStringの作成は関数の最初に移動しましたが、その部分は変更されていません。
`username_file`を作成する代わりに、read_to_stringの呼び出しをFile::open("hello.txt")の結果に直接連結することで、コードを簡潔にすることが出来ます。

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

ファイルを文字列に読み込むのはかなり一般的な操作なので、`fs::read_to_string`という関数が標準ライブラリにあります。
これはファイルを開いて新しいStringを作成し、ファイルの内容を読んでStringに代入して、返しています。

## `?`演算子を使える場所

`?`演算子は、`?`演算子が使われる値と、戻り値の型が一致する関数の中でしか使うことが出来ません。これは`match`式と同じように、`?`演算子が関数から値を早期に返すように定義されているからです。

では実際に、`?`演算子を使用して値の型と互換性のない戻り値の型があるメイン関数で`?`演算子を使用した場合に発生するエラーについてみていきましょう。

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")?;
}
```

このコードではファイルを開こうとしていますが、失敗する可能性があります。
File::openが返すResultの値を`?`演算子が追っていますが、このメイン関数の戻り値の型はResultではなく`()`です。
このコードをコンパイルすると、次のようなエラーメッセージが表示されます。

```bash
cargo build
   Compiling result v0.1.0 (/Users/ittoku/Documents/learning/rust/projects/result)
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
  --> src/main.rs:60:52
   |
4  | fn main() {
   | --------- this function should return `Result` or `Option` to accept `?`
...
60 |         let greeting_file = File::open("hello.txt")?;
   |                                                    ^ cannot use the `?` operator in a function that returns `()`
   |
   = help: the trait `FromResidual<Result<Infallible, std::io::Error>>` is not implemented for `()`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `result` due to previous error
```

このエラーでは、`Result, Option, FromResidual`を実装したその他の型を返す関数内でのみ、`?`演算子を使用することが許されていると指摘しています。

このエラーを修正するには2つの選択肢があります。1つは、`?`演算子を使う値と互換性があるように関数の戻り値の型を変更することと、もう1つは、`match, Result<T, E>`を使って適切な方法で処理する方法です。

エラーメッセージには、`?`演算子は`Option<T>`値でも使用できると書かれています。これはResultに対して`?`演算子を使う場合と同様に、`Option`を返す関数の中でOptionに対して`?`演算子を使うことが可能です。
`Option<T>`上で呼び出された時の`?`演算子の動作は、`Result<T, E>`上で呼び出された時の動作と似ています。
以下のコードは、与えられたテキスト内の最初の行の最後の文字を見つける関数の例です。

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

この関数は`Option<char>`を返します。なぜなら、そこに文字がある可能性もありますが、ない可能性もあるからです。
このコードはテキスト文字列のスライス引数をとり、それに対して`lines`メソッドを呼び出し、文字列の行に対するイテレータを返します。
そして`next`メソッドを呼び出し、そのイテレータから最初の値を取得しています。
この場合、last_of_first_lineを停止してNoneを返します。textが空文字列でない場合、`next`はtextの最初の行の文字列スライスを含むSome値を返します。

次に文字列スライスを抽出し、それに対して`chars`メソッドを呼び出してその文字のイテレータを取得します。
最後にそこからイテレータの最後の項目を返す`last`メソッドを呼び出します。例えば、テキストが空白行で始まり、他の行に文字列がある場合、"\nhi"のように最初の行が空文字列である可能性があるからです。ただし最初の行に最後の文字がある場合はSomeバリアントで返されます。
`?`演算子は、このロジックを簡潔に表現してくれるので、1行で関数を実装することができるのです。
もしOptionで`?`演算子が使えなかったら、もっとメソッド呼び出しや`match`四季を使ってこのロジックを実装しなければならないでしょう。

Resultを返す関数内でResultに`?`演算子を使うことも、Optionを返す関数内でOptionに`?`演算子を使うこともできますが、混在させることはできません。
`?`演算子は、ResultをOptionに、または逆に変換することはありません。そのような場合は、Resultの`ok`メソッドや、Optionの`ok_or`メソッドなどを使って、明示的に変換を行うことが可能です。

これまで作ってきた`main`関数はすべて`return()`でした。main関数は実行プログラムの入口と出口になるため特別で、プログラムが期待通りに動作するためには、その戻り値の型に制約があるのです。

幸いなことに、`main`は`Result<(), E>`を返すこともできます。以下のコードは、`main`関数の戻り値の型を`Result<(), Box<dyn Error>>`に変更し、戻り値の`Ok(())`を最後に追加しています。このコードはコンパイルされます。

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt");
    Ok(())
}
```

`Box<dyn Error>`型は、第17章で説明する特性オブジェクトです。今はあらゆる種類のエラーという意味で読んでください。
エラータイプBox`<dyn Error>`を持つmain関数内のResult値に`?`を使用することは、任意のErr値を早期に返すことができるため、許可されています。
このmain関数の本体は`std::io::Error`型のエラーしか返しませんが、`Box<dyn Error>`を指定すると、他のエラーを返すコードがmain本体に追加されてもシグネチャは正しく保たれます。

main関数が`Result<(), E>`を返す時、mainが`Ok(())`を返せば、実行ファイルは0という値で終了し、Err値を返せば0以外の値で終了します。
C言語で書かれた実行形式は、終了時に整数を返します。正常に終了したプログラムは整数の0を返し、エラーが発生したプログラムは0以外の整数を返します。Rustもこの監修に従うために、実行形式から整数を返します。

main関数は、ExitCodeを返す関数レポートを含む`std::process::Termination`特性を実装している任意の型を返すことが出来ます。
独自の型にTermination特性を実装するための詳細については、標準ライブラリのドキュメントを参照してください。

ここでは`panic!`を呼び出すか、`Result`を返すかの詳細について説明しましたが、どのような場合にどちらを使用するのか、次の節で見ていきましょう。
