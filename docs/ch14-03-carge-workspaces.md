# Cargoワークスペース

> Ref: https://doc.rust-jp.rs/book-ja/ch14-03-cargo-workspaces.html

第12章ではバイナリクレートとライブラリクレートを含むパッケージを構築しました。
プロジェクトの開発が進むにつれて、ライブラリクレートの肥大化が進み、複数のライブラリクレートにパッケージを分割したくなることでしょう。
Cargoはワークスペースという協調して開発された関連のある複数のパッケージを管理するのに役立つ機能を提供しています。

## ワークスペースの生成

ワークスペースは同じCargo.lockと出力ディレクトリを共有する一連のパッケージです。
ワークスペースを使用したプロジェクトを作成し、ワークスペースの構造に集中できるよう、瑣末なコードを書いていきます。
ワークスペースを構築する方法は複数ありますが、一般的な方法を提示します。
まずバイナリ1つとライブラリ2つを含むワークスペースを作ります。
バイナリは主要な機能を提供し、2つのライブラリに依存します。
ライブラリは`add_one`関数と`add_two`関数を提供します。
これら3つのクレートが同じワークスペースの一部になります。
ワークスペース用の新しいディレクトリを作ることから始めましょう。

```bash
mkdir cargo_workspaces
cd cargo_workspaces
```

次にcargo_workspacesディレクトリにワークスペース全体を設定するCargo.tomlファイルを作成します。
このファイルには他のCargo.tomlでみられる`[package]`セクションやメタデータはありません。
代わりにバイナリクレートへのパスを指定することで、ワークスペースにメンバを追加する`[workspace]`セクションを使用します。

```toml
[workspace]

members = [
    "adder",
]
```

次に`adder`バイナリクレートを作成します

```bash
cargo new --bin adder
```

この時点で`cargo build`を走らせるとワークスペースを構築できます。

```txt
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

ワークスペースにはコンパイルした生成物を置けるように最上位に`target`ディレクトリがあります。
`adder`クレートには`target`ディレクトリはありません。
`adder`ディレクトリ内部から`cargo build`を実行してもコンパイルした生成物は`cargo_workspaces/target`に置かれます。
ワークスペースのクレートはお互いに依存し合うことを意味するので、Cargoはワークスペースの`target`ディレクトリをこのように構成します。
各クレートが`target`ディレクトリを持っていたら、各クレートがワークスペースの他のクレートを再コンパイルし、`target`ディレクトリに生成物がある状態にしなければなりません。
1つの`target`ディレクトリを共有することで、クレートは不必要な再ビルドを回避できるのです。

## 2つ目のクレートを作成

ワークスペースに`add-one`というメンバクレートを作成します。
最上位のCargo.tomlを次のように変更します。

```toml
[workspace]

members = [
    "adder",
    "add-one",
]
```

そして`add-one`という名前のライブラリクレートを生成します。

```bash
cargo new add-one --lib
```

cargo_warkspacesディレクトリの構造は次のようになります。

```tree
├── Cargo.lock
├── Cargo.toml
├── add-one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

`add-one/src/lib.rs`に`add_one`関数を追加しましょう。

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

ワークスペースにライブラリクレートを追加したので、バイナリクレート`adder`をライブラリクレート`add-one`に依存することができます。
そのためには`add-one`へのパス依存を`adder/Cargo.toml`に追加する必要があります。

```toml
[dependencies]

add-one = { path = "../add-one" }
```

Cargoはワークスペースのクレートが、お互いに依存していると想定していないので、クレート間の依存関係について明示する必要があります。

次に`adder`クレートの`add-one`クレートから`add_one`関数を使用しましょう。
`adder/src/main.rs`ファイルに以下のコードを記述します。

```rust
extern crate add_one;

fn main() {
    let num = 10;

    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
}
```

cargo_workspacesディレクトリでワークスペースをビルドしてみましょう。

```bash
cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
```

cargo_workspacesディレクトリからバイナリクレートを実行するには、`-p`引数とパッケージ名を`cargo run`とともに使用して、使用したいワークスペースのパッケージを指定する必要があります。
`cargo run`でも実行できます。

```bash
cargo run -p adder
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```

## 外部クレートに依存する

ワークスペースには各クレートのディレクトリそれぞれにCargo.lockが存在するのではなく、最上位階層にのみ存在します。
これにより全クレートが全依存の同じバージョンを使用することを確認できます。
`rand`クレートを`adder, add-one`に追加すると、Cargoは両者をあるバージョンの`rand`に解決し、それを1つのCargo.lockに記録します。
ワークスペースの全クレートに同じ依存を使用させることは、ワークスペースのクレートが相互に互換性を常に維持するということです。
`add-one`の`[dependencies]`に`rand`クレートを追加してみましょう。

```toml
[dependencies]

rand = "0.8.5"
```

これで`add-one`で`extern crate rand;`を追加でき、`cargo build`を実行して`rand`クレートをコンパイルすることができます。

これでCargo.lockに`rand`に対する`add-one`の依存の情報を含むようになりました。
ですが`rand`はワークスペースのどこで使用されているにもかかわらず、それぞれのCargo.tomlにも、`rand`を追加しない限りワークスペースの他のクレートで使用することができません。
例えば`adder`クレートで`extern crate rand;`を追加するとエラーが出ます。

これを修正するには、`adder`のCargo.tomlを編集し、`rand`に依存していることを示します。
`adder`をビルドすると、`rand`をCargo.lockの`adder`の依存一覧に追加しますが、`rand`がダウンロードされることはありません。
Cargoがワークスペースの`rand`を使用するどのクレートも、同じバージョンを使っていることを確かめてくれます。
これにより複数のコピーが存在せずスペースを節約し、相互換性を維持することができます。

## テストを追加する

`add-one`クレート内に`add_one::add_one`関数のテストを追加しましょう。

```rust
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
```

では最上位の`cargo_workspaces`ディレクトリで`cargo test`を実行します。

```bash
cargo test
     Running unittests src/lib.rs (target/debug/deps/add_one-dfbd496a7091ca9f)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/adder-6d31b9f7c67d9f3e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add-one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

ワークスペースの最上位でテストを行うと、全クレートのテストを実行します。

`-p`フラグにクレート名を指定することで、特定したクレートのテストのみを実行することができます。

ワークスペースのクレートをCrates.ioに公開したら、ワークスペースのクレートは個別に公開する必要があります。
`cargo publish`コマンドには`--all, -p`フラグはないので、各クレートのディレクトリに移動して、各クレートを`cargo publish`で公開しなければなりません。

鍛錬を積むために`add-two`クレートを追加してみてください！

プロジェクトが肥大化してきたら、ワークスペースの使用を考えてみてください。
大きな1つのコードの塊より、微細で個別のコンポーネントの方が理解しやすいです。
またワークスペースにクレートを保持することは、同時に変更されることが多いなら、協調しやすくなることにも繋がります。
