# 普及不可能なエラーで`panic!`

> Ref: https://doc.rust-lang.org/stable/book/ch09-01-unrecoverable-errors-with-panic.html

コードの中で悪いことが起きて、どうにもならないことがあります。このような場合、Rustには`panic!`マクロがあります。
`panic!`を起こすには、コードがパニックになるような動作（例えば、配列の末尾にアクセスする）を行うか、明示的に`panic!`を使うか、どちらの場合もプログラムにパニックを発生させます。
デフォルトでは、これらのパニックは失敗メッセージを表示し、アンワインドし、スタックをクリーンアップし、終了します。
環境変数を使って、パニック発生時にRustにコールスタックを表示させ、パニックの原因を突き止めるのを容易にすることもできます。

> **パニック時のスタック解除・中止について**
>
> デフォルトでは、パニックが発生すると、プログラムは巻き戻しを開始します。
> つまりRustは、スタックを歩いて戻り、遭遇した各関数からデータをクリーンアップするのです。
> しかしこのスタックバックとクリーンアップは大変な作業です。
> そこでRustでは、クリーンアップを行わずにプログラムを終了させる「即時アボート(immediately aborting)」という選択肢も用意されています。
>
> プログラムが使用していたメモリは、OSによってクリーンアップされる必要があります。
> もしプロジェクトで生成されるバイナリをできるだけ小さくする必要があるなら、`Cargo.toml`ファイルに以下の記述をすることで、パニック時に巻き戻しからabortingにするように切り替えることが出来ます。
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

では、`panic!`を呼び出すプログラムを作成してみましょう。
```rust
fn main() {
    panic!("crash and burn");
}
```
プログラムの出力は次のようになります。
```bash
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/error_handling`
thread 'main' panicked at 'crash and burn', src/main.rs:2:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
`panic!`の呼び出しにより、最後の2行に含まれるエラーメッセージが表示されます。

この場合、表示された行は私たちのコードの一部であり、その行に移動すると`panic!`マクロの呼び出しが表示されます。
バックトレースを使用すると、`panic!`呼び出しが発生した関数の問題を引き起こしているコードの部分を特定することが出来ます。

## `panic!`バックトレースを使う

`panic!`マクロを直接呼び出すコードではなく、コードのバグが原因でライブラリから`panic!`が呼び出される場合についてみていきましょう。
以下のコードは、有効なインデックスの範囲を超えるベクター内のインデックスにアクセスしようとするコードです。
```rust
let v = vec![1, 2, 3];

v[99];
```
C言語では、データ構造の終端を超えて読み込もうとすると、未定義の動作になります。
メモリがその構造体に属していないにも関わらず、データ構造体のその要素に対応するメモリの場所にあるものを取得する可能性があります。
これはバッファオーバーリードと呼ばれ、攻撃者がインデックスを操作して、データ構造の後に格納されている許されるべきでないデータを読み取ることが出来た場合、セキュリティ上の脆弱性につながる可能性があります。

このような脆弱性からプログラムを守るために、存在しないインデックスの要素を読み込もうとすると、Rustは実行を停止して続行を拒否します。
```rust
cargo run
   Compiling error_handling v0.1.0 (/Users/ittoku/Documents/learning/rust/projects/error_handling)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/error_handling`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
このエラーはmain.rsの4行目、インデックス99にアクセスしようとしている箇所を指しています。
次の文には、`RUST_BACKTRACE`環境変数を設定することで、何が起こったのかを正確にバックトレースを取得できると書かれています。
バックトレースとは、この時点までに呼び出されたすべての関数のリストです。
Rustのバックトレースは他の言語と同じように動作します。
バックトレースを読むためのコツは、先頭から始めて自分が書いたファイルがあるところまで読むことです。
そこが問題が発生した場所で、この前後の行には、Rustのコアコード、標準ライブラリのコード、使用しているクレートが含まれるかもしれません。
環境変数`RUST_BACKTRACE`に0以外の値を設定してバックトレースを取得してみましょう。
```bash
RUST_BACKTRACE=1 cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/error_handling`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/std/src/panicking.rs:575:5
   1: core::panicking::panic_fmt
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/panicking.rs:64:14
   2: core::panicking::panic_bounds_check
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/panicking.rs:147:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/slice/index.rs:260:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/slice/index.rs:18:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/alloc/src/vec/mod.rs:2727:9
   6: error_handling::main
             at ./src/main.rs:4:5
   7: core::ops::function::FnOnce::call_once
             at /rustc/fc594f15669680fa70d255faec3ca3fb507c3405/library/core/src/ops/function.rs:507:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
たくさんの出力がありますね。表示される正確な出力は、OSやRustのバージョンによって異なる場合があります。
このような情報を含むバックトレースを取得するには、デバッグシンボルを有効にする必要があります。
デバッグシンボルは、`cargo build, cargo run --release`など指定しない限り、デフォルトで有効になっています。

この出力では、バックとレースの6行目が問題を引き起こしているプロジェクトの行を指しています。src/main.rsの4行目です。
プログラムをパニックにしたくないのであれば、私たちが書いたファイルに言及している最初の行が指す場所で調査を開始すべきです。
この場合、パニックを修正する方法は、ベクターのインデックスの範囲を超えて要素を要求しないことです。
今後コードがパニックを起こすときは、コードがどのような値でどのような動作をしてパニックを起こしているのか、代わりにどのような動作をすべきかを把握する必要があります。
