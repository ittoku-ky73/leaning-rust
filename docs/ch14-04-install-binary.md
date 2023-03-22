# Crates.ioからバイナリをインストール

> Ref: https://doc.rust-jp.rs/book-ja/ch14-04-installing-binaries.html

`cargo install`コマンドを使用すると、バイナリクレートをローカルにインストールし、使用することができます。
これはシステムパッケージを置き換えることを意図したものではありません。
即ちRustの開発者が、他人がCrates.ioに共有したツールをインストールするのに便利な方法を意味します。
バイナリターゲットを持つパッケージのみインストールすることができます。
バイナリターゲットとは、クレートが`src/main.rs`やバイナリとして指定された他のファイルを持つ場合に生成される実行可能なプログラムのことです。
逆に単独で実行不可能で他のプログラムに含むのに適しているのがライブラリターゲットとなります。
通常クレートにはREADME.mdにクレートがライブラリかバイナリターゲットか、両方を持つかという情報があります。

`cargo install`でインストールされるバイナリは全て、`bin`フォルダに保存されます。
`rustup`を使用し独自の設定を行なっていなければ、`HOME/.cargo/bin`になります。
`cargo install`でインストールしたプログラムを実行するには、`PATH`を通す必要があります。

例えば第12章で`ripgrep`という`grep`ツールのRust版に触れました。
`ripgrep`をインストールするには以下のコマンドを実行します。

```bash
cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v13.0.0
  --snip--
  Compiling ripgrep v13.0.0
   Finished release [optimized + debuginfo] target(s) in 26.42s
  Installing /Users/ittoku/.cargo/bin/rg
   Installed package `ripgrep v13.0.0` (executable `rg`)
```

ログにはインストールされたバイナリの位置と名前を示しています。
インストールディレクトリが`PATH`に存在する限り、`rg --help`を走らせて、より高速でRustらしいファイル検索ツールを使用することができます。
