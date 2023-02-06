# モジュールをファイルに分ける

> Ref: https://doc.rust-lang.org/stable/book/ch07-05-separating-modules-into-different-files.html

これまでの章では、1つのファイルで複数のモジュールを定義していました。
モジュールが大きなっていくと、その定義を別のファイルに移動して、コードをナビゲートしたくなるかもしれません。

例えば、複数のレストランモジュールを含む以下のコードで考えてみましょう。
クレートルートファイルですべてのモジュールを定義する代わりに、モジュールをファイルに抽出します。
この場合、クレートのルートファイルは`src/lib.rs`になります。

まず、`front_of_house`モジュールを独自のファイルに抽出します。まずは`src/lib.rs`ファイルを変更しましょう。

```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

次に、`src/front_of_house.rs`ファイルを作成し、`front_of_house`を定義します。

```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

ファイル分割した際のファイルのロードは、1度だけ行います。

次に、`hosting`モジュールを独自のファイルに抽出します。モジュールのディレクトリ名は、モジュールツリーの先祖にちなんで名付けられます。今回の場合、`src/front_of_house/`になります。

モジュールの移動を開始する前に、`src/front_of_house.rs`を以下のように変更します。

```rust
pub mod hosting;
```

次に、`src/front_of_house/hosting.rs`を作成して、モジュールで作成された定義を含めます。

```rust
pub fn add_to_waitlist() {}
```

もし、`hosting.rs`をsrcディレクトリに配置すると、コンパイラは`hosting.rs`が`front_of_house`モジュールの子として宣言されていないと想定します。
どのファイルをチェックしてどのモジュールのコードを確認するかというコンパイラの規則は、ディレクトリとファイルがモジュールツリーにより厳密に一致することを意味します。

> **代替ファイルパス**
>
> これまで、Rustコンパイラがし最も使用する慣用的なファイルパスについて説明してきましたが、Rustは古いスタイルのファイルパスもサポートしています。
> crateルートで宣言された`front_of_house`のモジュールの場合、コンパイラは次のモジュールを探します。
>
> - src/front_of_house.rs
> - src/front_of_house/mod.rs (Old Style)
>
> `hosting`のサブモジュールである`front_of_house`の場合、コンパイラは次のモジュールのコードを探します。
>
> - src/front_of_house/hosting.rs
> - src/front_of_house/hosting/mod.rs (Old Style)
>
> 同じモジュールに両方のスタイルを使用すると、コンパイルエラーが発生します。スタイルは統一しましょう。
>
> また、mod.rsのスタイルの主な欠点は、プロジェクトがmod.rsだらけになることです。そうなると目当てのファイルが見つかりづらくなります。

各モジュールのコードを別のファイルに移動はしましたが、モジュールツリーは同じままです。定義が異なるファイルでも関数呼び出しは、`eat_at_restaurant`の変更は必要ありません。
この手法をとると、モジュールのサイズが大きくなった時に、モジュールを新しいファイルに移動することが出来ます。

`src/lib.rs`の`pub use crate::front_of_house::hosting`も変更されておらず書き換えは必要ありません。便利ですね。

### まとめ

- Rustでは、パッケージを複数のクレートに分割し、クレートをモジュールに分割できるため、あるモジュールで定義された項目を別のモジュールから参照できます。
- これらは絶対、相対パスを指定し、パスは`use`を使用してスコープに入れることができるため、その短いパスを使用できます。
- モジュールコードはデフォルトでプライベートですが、`pub`キーワードを追加することで定義を公開することが出来ます。
