# Leaning-rust

Rustプログラミング言語について学んだことを記録するリポジトリ

> References: https://doc.rust-lang.org/book/

- [Hello fellow Rustaceans](https://github.com/ittokun/leaning-rust/tree/main/projects/hello_fellow_rustaceans)

0.  [Introduction](https://github.com/ittokun/leaning-rust/tree/main/docs/ch00-00-introduction.md)
1.  Getting Started
   - [Hello World](https://github.com/ittokun/leaning-rust/tree/main/docs/ch01-02-hello-world.md)
   - [Hello Cargo](https://github.com/ittokun/leaning-rust/tree/main/docs/ch01-03-hello-cargo.md)
2.  [guessing Game](https://github.com/ittokun/leaning-rust/tree/main/docs/ch02-00-guessing-game.md)
3.  Common Programming Concepts
   - [variables](https://github.com/ittokun/leaning-rust/tree/main/docs/ch03-01-variables-and-mutability.md)
   - [data_types](https://github.com/ittokun/leaning-rust/tree/main/docs/ch03-02-data-types.md)
   - [functions](https://github.com/ittokun/leaning-rust/tree/main/docs/ch03-03-functions.md)
   - [Comments](https://github.com/ittokun/leaning-rust/tree/main/docs/ch03-04-comments.md)
   - [Control Flow](https://github.com/ittokun/leaning-rust/tree/main/docs/ch03-05-control-flow.md)
4.  Understanding Ownership
   - [Ownership](https://github.com/ittokun/leaning-rust/tree/main/docs/ch04-01-what-is-ownership.md)
   - [References and Borrowing](https://github.com/ittokun/leaning-rust/tree/main/docs/ch04-02-references-and-borrowing.md)
   - [The Slice Type](https://github.com/ittokun/leaning-rust/tree/main/docs/ch04-03-the-slice-type.md)
5.  Using Structs to Structure Related Data
   - [Defining and Instantiating Structs](https://github.com/ittokun/leaning-rust/tree/main/docs/ch05-01-defining-and-instantiating-structs.md)
   - [An Example Program Using Structs](https://github.com/ittokun/leaning-rust/tree/main/docs/ch05-02-an-example-program-using-structs.md)
   - [Method Syntax](https://github.com/ittokun/leaning-rust/tree/main/docs/ch05-03-method-syntax.md)
6.  Enums and Pattern Matching
   - [Defining an Enum](https://github.com/ittokun/leaning-rust/tree/main/docs/ch06-01-defining-an-enum.md)
   - [match Control Flow](https://github.com/ittokun/leaning-rust/tree/main/docs/ch06-02-the-match-control-flow-construct.md)
   - [Concise Control Flow with if let](https://github.com/ittokun/leaning-rust/tree/main/docs/ch06-03-concise-control-flow-with-if-let.md)
7.  Managing Growing projects with Packages, Crates, Modules
   - [packages and Crates](https://github.com/ittokun/leaning-rust/tree/main/docs/ch07-01-packages-and-crates.md)
   - [Defining Modules to Control Scope and Privacy](https://github.com/ittokun/leaning-rust/tree/main/docs/ch07-02-defining-modules-control-scope-privacy.md)
   - [Paths for Referring to an Item in the Module Tree](https://github.com/ittokun/leaning-rust/tree/main/docs/ch07-03-paths-referring-to-item-module-tree.md)
   - [Bringing Paths into Scope with the use Keyword](https://github.com/ittokun/leaning-rust/tree/main/docs/ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
   - [Separating Modules into Different Files](https://github.com/ittokun/leaning-rust/tree/main/docs/ch07-05-separating-modules-into-different-files.md)
8.  [Common Collections](https://github.com/ittokun/leaning-rust/tree/main/docs/ch08-00-common-collections.md)
   - [Storing Lists of Values with Vectors](https://github.com/ittokun/leaning-rust/tree/main/docs/ch08-01-storing-lists-of-values-with-vectors.md)
   - [Storing UTF-8 Encoded Text with Strings](https://github.com/ittokun/leaning-rust/tree/main/docs/ch08-02-storing-utf8-encoded-text-with-strings.md)
   - [Storing Keys with Associated Values in Hash Maps](https://github.com/ittokun/leaning-rust/tree/main/docs/ch08-03-hash-maps.md)
9.  [Error Handling](https://github.com/ittokun/leaning-rust/tree/main/docs/ch09-00-error-handling.md)
   - [Unrecoverable Errors with panic!](https://github.com/ittokun/leaning-rust/tree/main/docs/ch09-01-panic.md)
   - [Recorverable Errors with Result](https://github.com/ittokun/leaning-rust/tree/main/docs/ch09-02-result.md)
   - [To panic! or Not to panic!](https://github.com/ittokun/leaning-rust/tree/main/docs/ch09-03-panic-or-not-panic.md)
10. [ジェネリック型、トレイト、ライフタイム](https://github.com/ittokun/leaning-rust/tree/main/docs/ch10-00-generics.md)
   - [ジェネリックなデータ型](https://github.com/ittokun/leaning-rust/tree/main/docs/ch10-01-syntax.md)
   - [トレイト](https://github.com/ittokun/leaning-rust/tree/main/docs/ch10-02-trait.md)
   - [ライフタイム](https://github.com/ittokun/leaning-rust/tree/main/docs/ch10-03-lifetime.md)
11. [テストを書く](https://github.com/ittokun/leaning-rust/tree/main/docs/ch11-00-testing.md)
   - [テストの書き方](https://github.com/ittokun/leaning-rust/tree/main/docs/ch11-01-writing-test.md)
   - [テストの実行](https://github.com/ittokun/leaning-rust/tree/main/docs/ch11-02-running-tests.md)
   - [テストの体系化](https://github.com/ittokun/leaning-rust/tree/main/docs/ch11-03-test-organization.md)
12. [コマンドラインプログラム](https://github.com/ittokun/leaning-rust/tree/main/docs/ch12-00-commandline.md)
   - [コマンドライン引数](https://github.com/ittokun/leaning-rust/tree/main/docs/ch12-01-arguments.md)
   - [ファイル読み込み](https://github.com/ittokun/leaning-rust/tree/main/docs/ch12-02-reading-a-file.md)
   - [リファクタリング](https://github.com/ittokun/leaning-rust/tree/main/docs/ch12-03-refactoring.md)
   - [テスト駆動開発](https://github.com/ittokun/leaning-rust/tree/main/docs/ch12-04-testing.md)
   - [環境変数](https://github.com/ittokun/leaning-rust/tree/main/docs/ch12-05-environment-variable.md)
   - [エラーメッセージ](https://github.com/ittokun/leaning-rust/tree/main/docs/ch12-06-error-message.md)
13. [イテレータとクロージャ](https://github.com/ittokun/leaning-rust/tree/main/docs/ch13-00-functional-features.md)
   - [クロージャ](https://github.com/ittokun/leaning-rust/tree/main/docs/ch13-01-closures.md)
   - [イテレータ](https://github.com/ittokun/leaning-rust/tree/main/docs/ch13-02-iterators.md)
   - [入出力プログラムの改善](https://github.com/ittokun/leaning-rust/tree/main/docs/ch13-03-improving-io-project.md)
   - [パフォーマンス比較](https://github.com/ittokun/leaning-rust/tree/main/docs/ch13-04-performance.md)
14. Cargoとcrates.io
   - [リリースプロファイル](https://github.com/ittokun/leaning-rust/tree/main/docs/ch14-01-release-profiles.md)
   - [crates.io](https://github.com/ittokun/leaning-rust/tree/main/docs/ch14-02-crates-io.md)
   - [Cargoワークスペース](https://github.com/ittokun/leaning-rust/tree/main/docs/ch14-03-carge-workspaces.md)
   - [バイナリインストール](https://github.com/ittokun/leaning-rust/tree/main/docs/ch14-04-install-binary.md)
   - [Cargoを拡張](https://github.com/ittokun/leaning-rust/tree/main/docs/ch14-05-extending-cargo.md)
15. [スマートポインタ](https://github.com/ittokun/leaning-rust/tree/main/docs/ch15-00-smart-pointers.md)
