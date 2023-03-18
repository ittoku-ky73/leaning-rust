# Crates.ioにクレートを公開

> Ref: https://doc.rust-jp.rs/book-ja/ch14-02-publishing-to-crates-io.html

プロジェクトの依存としてcrates.ioのパッケージを使用してきましたが、自分のパッケージを公開することで他の人のコードを共有することもできます。
crates.ioのクレート登録場は、自分のパッケージのソースコードを配布するので、主にオープンソースのコードをホストします。

RustとCargoは公開したパッケージを人が使用し、そもそも見つけやすくしてくれる機能を有しています。
これらの機能の一部を紹介し、そしてパッケージの公開方法も見ていきます。

## ドキュメンテーションコメント

パッケージを正確にドキュメントすることで、他のユーザーがパッケージを使用する方法や、いつ使用すべきかを理解することができます。
第3章では`//`でRustのコードにコメントをつけることを学びました。
Rustにはドキュメンテーション用のコメントも用意されていて、これはドキュメンテーションコメントといい、HTMLドキュメントを生成します。
クレートの実装法とは対照的にクレートの使用法を知ることに興味のあるプログラマ向けの、公開API用のドキュメンテーションコメントの中身をHTMLで表示します。

ドキュメンテーションコメントは`///`を使用し、テキストを整形するMarkdown記法もサポートしています。
ドキュメント対象の要素の直前にドキュメンテーションコメントを配置します。

```rust
/// Adds one to the number given
///
/// # Examples
///
/// ``
/// let five = 5;
///
/// assert_eq!(6, cratesio::add_one(5));
/// ``
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

ここでは`add_one`関数の説明、`Examples`というタイトルのセクション、`add_one`関数の使用方法が書かれています。
ドキュメンテーションコメントは`cargo doc`を実行することで、HTMLドキュメントを生成することができます。
このコマンドはコンパイラとともに配布されている`rustdoc`ツールを実行し、生成されたHTMLドキュメントを`target/doc`ディレクトリに配置します。

`cargo doc --open`を実行すると、現在のクレートのドキュメント用のHTML（と自分のクレートが依存している全てのドキュメント）を構築し、その結果をWebブラウザで開きます。実行して確認してみましょう。

## セクション

以下のセクションはクレート筆者がよく使用するセクションです。

- `Panics`、ドキュメント対象の関数が`panic!`する可能性のある筋書きです。
- `Errors`、`Result`を返す関数で起きうるエラーの種類と条件が記載せれます。
- `Safety`、関数呼び出しが`unsafe`（第19章で説明）なら、その理由が記載されます。

多くのドキュメンテーションコメントでは、これら全てのセクションが必要になることはありません。
しかし自分のコードを呼び出しているユーザーが、知りたいと思う場面もあります。

## テストのドキュメンテーションコメント

ドキュメンテーションコメントのコードブロックは、ライブラリの使用方法のデモに役立ちます。
そして`cargo test`を走らせると、ドキュメントのコードブロックをテストしてくれます！
デモ付きのドキュメントに上回るものはありません。
しかしコードの変更などで、デモ通りに動作しない事態となったら、全く意味のないものになります。
では`cargo test`を走らせてドキュメンテーションコメントに書かれているテストが実行されるか確認してみましょう。

あとはコードがパニックになるように変更してもう一度`cargo test`を実行し、デモとコードがお互いに同期されていないことも確認してみましょう。

## 要素にコメントする

`//!`はコメントを含む要素にドキュメンテーションを付け加えます。
これはよくクレートのルートファイル内部やモジュール内部にコメントし、クレートやモジュール全体にドキュメントを付け加える際に使用されます。

以下の例は、`src/lib.rs`に`add_one`関数を含む`cratesio`クレートの目的を解説するドキュメンテーションを追加しています。

```rust
//! # Crates IO
//!
//! `cratesio` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given
// --snip--
```

`//!`はコードにコメントをつけるわけではなく、ドキュメントにコメントをつけます。
上記のドキュメンテーションコメントは、クレート全体の説明をしています。

では、`cargo doc --open`を実行して、ページの変化を確認しましょう。

このドキュメンテーションコメントは、クレートやモジュールを解説するのに有用です。
コンテナ全体の目的を説明し、クレートの使用者がクレートの体系を理解するためにこのコメントは使用されます。

## 公開APIをエクスポート

第7章では`mod`キーワードを使用してモジュールにコードを体系化する方法、`pub`キーワードで要素を公開する方法、`use`キーワードで要素をスコープに導入する方法を見てきました。
しかしながらクレートの開発中に、自分にとって意味のある構造は、ユーザーにはあまり便利ではない可能性があります。
複数階層を含み、自分の構造体を体系化したいときに、それから階層の深いところで定義した型を使用したい人は、型が存在することを見つけ出すのに困難を伴う可能性もあります。
またそのような人は、`use cratesio::UsefullType`の代わりに`use cratesio::some_module::another_module::UsefullType`と入力するのが煩わしく感じる可能性もあります。

自分の公開APIの構造は、クレートを公開する際に考慮すべき点です。
自分のクレートを使用したい人は、自分よりその構造に馴染みがないですし、クレートのモジュール階層が大きければ、使用したい部分を見つけるのが困難になる可能性があります。

嬉しいことに、構造がユーザーにとって他のライブラリから使用するのに便利ではない場合、内部的な体型を再構築する必要はありません。
代わりに要素を再エクスポートし、`pub use`で自分の非公開構造とは異なる公開構造にできます。
再エクスポートはある場所の公開要素を1つを取り、別の場所で定義されているかのように別の場所で公開します。

例えば、芸術的な概念をモデル化するために`art`というライブラリを作ったとしましょう。
このライブラリ内には、2つのモジュールがあります。
`PrimaryColor, SecondaryColor`という2つの列挙型を含む`kind`モジュールと`mix`という関数を含む`utils`モジュールです。

```rust
//! # Art
//!
//! A library for modeling artistic concepts.

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use super::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
    }
}
```

今のままでは、`PrimaryColor, SecondaryColor`型と`mix`関数はページに表示されません。
`kinds, utils`をクリックして参照はすることができます。

このライブラリに依存する別のクレートは、現在定義されているモジュール構造を指定して、`art`の要素をインポートする`use`文が必要になります。

```rust
extern crate art;

use art::kinds::PrimaryColor;
use art::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
```

コードの筆者は`art`クレートを使用しているので、`PrimaryColor`が`kinds`モジュールにあること、`mix`が`utils`モジュールにあることを知っておく必要があります。
`art`クレートのモジュール構造は、`art`クレートの使用者より開発者の方が関係が深いです。

公開APIから内部体系を除去するために、`src/lib.rs`に以下のコードを記述します。

```rust
//! # Art
//!
//! A library for modeling artistic concepts.

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds {
    // --snip--
}

pub mod utils {
    // --snip--
}
```

このクレートに対して`cargo doc`が生成するAPIドキュメントは、トップページに再エクスポートを列挙しリンクするので、型や関数を見つけやすくなります。

ネストされたモジュールがたくさんあるような場合、最上位階層で`pub use`により型を再エクスポートすることは、クレートの使用者の経験に大きな違いを生みます。

役にたつAPI構造を作ることは、科学というよりも芸術の領域であり、ユーザーにとって何が最善のAPIなのか、探求するために繰り返してみることができます。
`pub use`は内部的なクレート構造に柔軟性をもたらし、その内部構造をユーザーに提示する構造から切り離してくれます。
インストールしてある他のクレートを見て、内部構造が公開APIと異なっているか確認してみてください。

## Crates.ioアカウント

クレートを公開する前に、[crates.io](https://crates.io)のアカウントを作成し、APIトークンを取得する必要があります。
Githubアカウントでログインすると良いです。
ログインしたら、[https://crates.io/me](https://crates.io/me)で、APIキーを取得します。
そして`cargo login`コマンドをAPIキーとともに実行します。

```bash
cargo login abcdefghijklmnopqrstuvwxyz012345
```

このコマンドは、CargoにAPIトークンを知らせ、`~/.cargo/credentials`に保存します。
このトークンは秘密です。他人とは共有しないでください。

## クレートにメタデータを追加

アカウントを作成して公開したいクレートがあるとします。
公開するには、Cargo.tomlファイルの`[package]`セクションにメタデータを追加する必要があります。

クレートには独自の名前が必要です。
クレート名は最初に来たものガチの精神で付与されますので、いったんクレーと名が取られてしまったら、その名前のクレートを他の人が公開することは絶対にできません。
試しにCargo.tomlファイルに以下のセクションを追加して公開できるかみてみましょう。

```toml
[package]
name = "guessing_game"
```

以下のコマンドを実行すると、警告とエラーが出ます。

```bash
cargo publish
    Updating crates.io index
warning: manifest has no description, license, license-file, documentation, homepage or repository.
--snip--
Caused by:
  the remote server responded with an error: missing or empty metadata fields: description, license. Please see https://doc.rust-lang.org/cargo/reference/manifest.html for how to upload metadata
```

エラーの内容は、説明、ライセンス、ドキュメント、ホームページかリポジトリがないと書かれています。
クレートを公開するには、目的と条件を他のユーザーに提示する必要があります。

`license`フィールドには、ライセンス識別子を与える必要があります。
[Software Package Data Exchange(SPDX)](http://spdx.org/licenses)に、使用できる識別子があります。
例えばこのクレートをMITライセンスにする場合、以下のようにします。

```toml
[package]
name = "guessing_game"
license = "MIT"
```

SPDXにないライセンスを使用する場合、ライセンスファイルをプロジェクトに置き、`license-file`フェール度にそのファイル名を指定する必要があります。

どのライセンスを使えば良いかは、ここでは説明しません。
Rustコミュニティの多くの人は、`MIT OR Apache-2.0`のデュアルライセンスを使用します。

独自の名前、バージョン、筆者の詳細、説明、ライセンスを追加することで、やっとクレートを公開することができます。

```toml
[package]
name = "guessing_game"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
description = "A fun game where you guess what number the computer has chosen."
              (コンピュータが選択した数字を言い当てる面白いゲーム)
license = "MIT OR Apache-2.0"

[dependencies]
```

[Cargoドキュメンテーション](https://doc.rust-lang.org/cargo)には、よりたやすくクレートを使用できることを保証する他のメタデータが解説されています。

## Crates.ioに公開

アカウント作成、APIトークン取得、クレート名の指定、メタデータの指定が終わったら、公開する準備が整います。
クレートを公開すると、crates.ioに他のユーザーが使用できるようにアップロードされます。

公開は永久ですのでクレートの効果維持には気をつけてください。
バージョンの上書きはできずコードも削除できません。
crates.ioの1つの主な目標が、クレートに依存している全てのプロジェクトのビルドが、動き続けるようにコードの永久アーカイブとして機能することです。
バージョン削除を許してしまうと、動かなくなるプロジェクトが出現する可能性があります。

再度以下のコマンドを実行してみましょう。

```bash
cargo publish
    Updating crates.io index
warning: manifest has no documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
   Packaging ittokun_guessing_game v0.1.0 (/Users/ittoku/Documents/learning/rust/projects/guessing_game)
   Verifying ittokun_guessing_game v0.1.0 (/Users/ittoku/Documents/learning/rust/projects/guessing_game)
   Compiling ittokun_guessing_game v0.1.0 (/Users/ittoku/Documents/learning/rust/projects/guessing_game/target/package/ittokun_guessing_game-0.1.0)
    Finished dev [unoptimized + debuginfo] target(s) in 3.08s
    Packaged 5 files, 3.8KiB (1.8KiB compressed)
   Uploading ittokun_guessing_game v0.1.0 (/Users/ittoku/Documents/learning/rust/projects/guessing_game)
    Updating crates.io index
     Waiting on `ittokun_guessing_game` to propagate to crates.io index (ctrl-c to wait asynchronously)
```

おめでとうございます！
これでRustコミュニティとコードを共有し、誰でもこのクレートを依存として簡単に追加することができます。

## クレートの新バージョンを公開

クレートに変更を行い新バージョンをリリースする準備ができたら、Cargo.tomlファイルに指定された`version`の値を変更し、再公開します。
セマンティックバージョンルールを使用して加えた変更の種類に基づいて適切なバージョン番号を指定します。
そして`cargo publish`を実行して新バージョンをアップロードします。

## Crates.ioからバージョンを削除

以前のバージョンのクレートは削除できないですが、将来のプロジェクトがこれに新たに依存することを防ぐことはできます。
これは何らかの理由により、クレートバージョンが壊れている場合に有用です。
そのような場面において、Cargoはクレートバージョンの取り下げ(yank)をサポートしています。

バージョンを取り下げると既存のプロジェクトは、引き続きダウンロードしたり依存したりできますが、新規プロジェクトが新しくそのバージョンに依存する事態を防ぐことができます。
つまり、すでにCargo.lockに存在するプロジェクトは壊さないが、将来のCargo.lockは取り下げられたバージョンを使用しないことを意味します。

バージョンのクレートを取り下げるには以下のコマンドを実行します。

```bash
cargo yank --vers 1.0.1
```

`--undo`オプションを付与することで、取り下げを取り消し、再度そのバージョンにプロジェクトを依存させることもできます。

```bash
cargo yank --vers 1.0.1 --undo
```

取り下げによるコードの削除は行われません。
取り下げ機能は誤ってアップロードされた秘密鍵を削除するものではありません。
もしそうなってしまったら、即座に秘密鍵をリセットしなければなりません。
