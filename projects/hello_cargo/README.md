# Hello Cargo

CargoはRustのビルドシステムおよびパッケージマネージャーです。ほとんどのRustaceanはこのツールを使用してRustプロジェクトを管理します。

これはCargoがコードのビルド、コードが依存するライブラリのダウンロード、それらのライブラリのビルドなどの多くのタスクを処理するためです。

以下のコマンドでCargoがインストールされているかどうかを確認します。

```shell
cargo --version
```

## Cargoでプロジェクトを作成する

以下のコマンドでプロジェクトを作成します。

```shell
cargo new hello_cargo
```

これで、hello_cargoという新しいディレクトリとプロジェクトが作成され、cargo.tomlファイルと、main.rsファイルを含むsrcディレクトリが作成されます。

**Cargo.toml**

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
```

このファイルは、Cargo の設定形式である[*TOML*](https://toml.io) ( *Tom's Obvious, Minimal Language ) 形式です。*

`package`は、次のステートメントがパッケージを構成していることを示すセクションの見出しになっています。

`package`の下の3行は、Cargoがプログラムをコンパイルするために必要な構成情報を設定します。

`dependencies`は、プロジェクトの依存関係をリストするためのセクションです。Rustでは、コードのパッケージはcrate（クレート）と呼ばれます。

Cargoは、ソースファイルがsrcディレクトリ内に存在することを想定しています。最上位のプロジェクトディレクトリは、README、ライセンス、構成ファイル、およびコードに関係のないその他のもの専用です。

## Cargoプロジェクトの構築と実行

それでは実際に以下のコマンドを実行してビルドしてみましょう。

```shell
cargo build
   Compiling hello_cargo v0.1.0 (/Users/MYNAME/Documents/learning-rust/projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
```

このコマンドは、*/target/debug/hello_cargo*に実行可能ファイルを作成します。デフォルトのビルドはデバッグビルドであるため、Cargoはバイナリを*debug*という名前のディレクトリに配置します。

以下のコマンドで実行可能ファイルを実行できます。

```shell
./target/debug/hello_cargo
Hello, world!
```

うまくいけば「Hello, world!」と端末に出力されます。`cargo build`を初めて実行すると、Cargoは最上位に新しいファイル*Cargo.lock*を作成します。このファイルは、プロジェクト内の依存関係の正確なバージョンを追跡します。このファイルを手動で変更する必要はありません。

また、以下のコマンドでコードをコンパイルし、結果の実行可能ファイルをすべて1つのコマンドで実行することもできます。

```shell
cargo run
   Compiling hello_cargo v0.1.0 (/Users/MyName/Documents/learning-rust/projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/hello_cargo`
Hello, world!
```

Cargoはファイルが変更されていないことを認識すると、再構築せずにバイナリを実行することに注意してください。

以下のコマンドは、コードを素早くチェックして、コンパイルされているが実行可能ファイルが生成されていないことを確認します。

```shell
cargo check
    Checking hello_cargo v0.1.0 (/Users/MyName/Documents/learning-rust/projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
```

多くのRustaceansはこのコマンドで定期的にプログラムを実行してコンパイルを確認します。そして、`cargo build`は実行可能ファイルを使用する準備ができたときに実行されます。

Cargoについての要約

- `cargo new`、プロジェクトを作成
- `cargo build`、プロジェクトを構築
- `cargo run`、プロジェクトの構築、実行
- `cargo check`、バイナリを生成せずプロジェクトの構築を行う
- ビルドの結果を現在のディレクトリと、*target/debug*ディレクトリに保存する。

## リリース用のビルド

プロジェクトが最終的にリリースの準備ができたら、`cargi build --release`コマンドで最適化してコンパイルをします。このコマンドは、*target/release*に実行可能ファイルを作成します。最適化によりRustコードの実行が高速化されますが、コンパイルにかかる時間が長くなります。

これが、2つの異なるプロファイルがある理由です。1つは開発用で、迅速かつ頻繁に再ビルドする必要がある場合で、もう1つはユーザーに提供する最終的なプログラムをビルドするためのものです。

コードの実行時間をベンチマークする場合は、必ず`cargo build --release`を実行し、*target/release*にある実行ファイルを使ってベンチマークしましょう。

## Cargoを条約とする

GitとCargoを組み合わせることによって、他のプロジェクトのコードをダウンロードして、そのプロジェクトのディレクトリに移動してビルドすることもできます。

```shell
git clone example.org/someproject
cd someproject
cargo build
```

## まとめ

今まで学んだこと

- ローカルにインストールされたドキュメントを開く
- `rustc`コマンドを使用して、「Hello, world!」を出力する
- Cargoの規則を使用して新しいプロジェクトを作成、実行する

次の第2章では、当てっこゲームのプログラムを作成します。

もし一般的なプログラミングの概念がRustでどのように機能するかを学ぶことから始めたい場合は、第3章を見てから、第2章に戻ってもいいかもしれないです。
