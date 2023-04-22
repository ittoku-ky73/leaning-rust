# 状態共有並行性

> Ref: https://doc.rust-jp.rs/book-ja/ch16-03-shared-state.html

メッセージ受け渡しは、並行性を扱う素晴らしい方法ですが、唯一の方法ではありません。
Go言語のドキュメントのスローガンでも「メモリを共有することでやりとりする」と書かれています。

メモリを共有することでやりとりするとはどういうことでしょうか。
さらになぜメッセージ受け渡しに熱狂的な人は、それを使わずに全く逆のことをするのでしょうか。

ある意味ではどんなプログラミング言語のチャンネルも単独の所有権に類似しています。
一旦チャンネルに値を転送したら、その値はもはや使用することがないからです。

メモリ共有並行性は、複数の所有権に似ています。
複数のスレッドが同時に同じメモリ位置にアクセスできるのです。
第15章でスマートポインタが複数の所有権を可能にしたように、異なる所有者を管理する必要があるので、複数の所有権は複雑度が上がります。
Rustの型システムと所有権規則は、この管理を正しく行う大きな助けになります。
例として、メモリ共有を行う一般的な並行性の基本型の1つであるミューテックスを見ていきましょう。

# ミューテックス

ミューテックスはどんな時も1つのスレッドのみデータへのアクセスを許可しないという**mutual exclusion**（相互排他）の省略形です。
ミューテックスにあるデータにアクセスするには、ミューテックスのロックを所望することでアクセスしたいことをスレッドに通知しなければなりません。
ロックとは現在誰かがデータへの排他的アクセスを行っているかを追跡するミューテックスの一部をなすデータ構造です。
故にミューテックスはロックシステム経由で保持しているデータを死守すると解説されます。

ミューテックスには2つの規則があり、これは覚えておく必要があります。

- データを使用する前にロックの獲得を試みなければならない。
- データの使用を終えたら他のスレッドがロックを獲得できるようにアンロックする必要がある。

ミューテックスを現実世界の物で例えるなら、マイクが1つしかない会議のパネルディスカッションになります。
パネリストが発言できる前にマイクを使用したいと申し出たり通知しなければなりません。
マイクを受け取ったら話したいことを話し、それから次に発言を申し出たパネリストにマイクを手渡します。
パネリストの発言が終わったのにマイクを手渡さなければ、他の人は発言することができません。
共有されているマイクの管理がうまくいかなければ、パネルは予定通りに機能しなくなります。

ミューテックスの管理は正しく行うのに著しく技巧を要することがあるので、多くの人はチャンネルに熱狂的になるわけです。
しかしながらRustの型システムと所有権規則のおかげで、ロックとアンロックをおかしくすることはありません。

### Mutex\<T> API

ミューテックスの使用方法の例として、ミューテックスをシングルスレッドの文脈で使うことから始めましょう。

```rust
let m = Mutex::new(5);
{
    let mut num = m.lock().unwrap();
    *num = 6;
}
println!("m = {:?}", m);
```

多くの型同様、`new`という関連関数を使用して`Mutex<T>`を生成します。
ミューテックス内部のデータにアクセスするには、`lock`メソッドを使用してロックを獲得します。
この呼び出しは現在のスレッドをブロックするので、ロックを得られる順番が来るまで何も作業はできません

ロックを保持している他のスレッドがパニックしたら、`lock`の呼び出しは失敗します。
その場合、誰もロックを取得することはできないので`unwrap`し、そのスレッドをパニックさせます。

ロックを獲得した後に`num`変数に戻り値を中に入っているデータへの可変参照として扱うことができます。
型システムにより`m`の値を使用する前にロックを獲得していることが確認されます。
`Mutex<i32>`は`i32`ではないので、`i32`を使用するにはロックを獲得しなければならないのです。

`Mutex<T>`はスマートポインタです。正確には`lock`の呼び出しで`MutexGuard`というスマートポインタを返却します。
このスマートポインタが内部のデータを指す`Deref`を実装しています。
さらに`MutexGuard`がスコープから外れたら自動的にロックを解除する`Drop`実装もしており上記のコードだと内部スコープの終わりでそれは発生します。
結果としてロックの解除が自動的に行われるので、ロック解除忘れや他のスレッドで使用されるのを阻害するリスクを負いません。

では上記のコードを実行して、内部の`i32`の値が6に変更されているか確認してみましょう。

## Mutex\<T>を共有

`Mutex<T>`を使って複数のスレッド間で値を共有してみましょう。
10個のスレッドを立ち上げ、各々カウンタの値を1ずつインクリメントさせます。
以下の数例は、コンパイルエラーになることに注意し、そのエラーを使用して`Mutex<T>`の使用法と、コンパイラがそれを正しく活用する手助けをしてくれる方法について学びます。

```rust
let counter = Mutex::new(0);
let mut handles = vec![];

for _ in 0..10 {
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();

        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("Result: {}", *counter.lock().unwrap());
```

`conter`変数を生成して`Mutex<T>`の内部に`i32`を保持しています。
次に数値の範囲をマッピングして10個のスレッドを生成しています。
`thread::spawn`を使用して全スレッドに同じクロージャを与えています。
このクロージャはスレッド内にカウンタをムーブし、`lock`メソッドを呼ぶことで`Mutex<T>`のロックを獲得しています。
それから`Mutex`の値に1を足し、スレッドがクロージャを実行し終えたら、`num`はスコープ外に出てロックを解除し、他のスレッドに`Mutex`の値を使えるようにしています。

メインスレッドですべてのjoinハンドルを収集します。
そして各に対して`join`を呼び出し、全スレッドが終了するのを確かめています。
その時点でメインスレッドはロックを獲得し、このプログラムの結果を出力します。

しかしこのコードはコンパイルできません。原因を探りましょう。

```
error[E0382]: use of moved value: `counter`
  --> src/main.rs:96:36
   |
92 |     let counter = Mutex::new(0);
   |         ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
...
96 |         let handle = thread::spawn(move || {
   |                                    ^^^^^^^ value moved into closure here, in previous iteration of loop
97 |             let mut num = counter.lock().unwrap();
   |                           ------- use occurs due to use in closure
```

エラーメッセージは、`counter`値はクロージャにムーブされ、`lock`を呼び出した時にキャプチャされていると述べています。
その説明は所望した動作のように聞こえますが許可されていないのです！

プログラムを単純化してこれを理解しましょう。
`for`ループで10個スレッドを生成する代わりにループなしで2つのスレッドを作るだけにしてどうなるか確認してみましょう。

```rust
let counter = Mutex::new(0);
let mut handles = vec![];

let handle = thread::spawn(move || {
    let mut num = counter.lock().unwrap();

    *num += 1;
});
handles.push(handle);

let handle2 = thread::spawn(move || {
    let mut num = counter.lock().unwrap();

    *num += 1;
});
handles.push(handle2);

for handle in handles {
    handle.join().unwrap();
}

println!("Result: {}", *counter.lock().unwrap());
```

2つのスレッドを生成し、2番目のスレッドの変数名を`handle2, num2`に変更しています。
上記のコードをコンパイル使用とすると以下のようなエラーを吐きます。

```
error[E0382]: use of moved value: `counter`
   --> src/main.rs:122:33
    |
112 |     let counter = Mutex::new(0);
    |         ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
...
115 |     let handle = thread::spawn(move || {
    |                                ------- value moved into closure here
116 |         let mut num = counter.lock().unwrap();
    |                       ------- variable moved due to use in closure
...
122 |     let handle2 = thread::spawn(move || {
    |                                 ^^^^^^^ value used here after move
123 |         let mut num = counter.lock().unwrap();
    |                       ------- use occurs due to use in closure

error[E0382]: borrow of moved value: `counter`
   --> src/main.rs:133:29
    |
112 |     let counter = Mutex::new(0);
    |         ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
...
122 |     let handle2 = thread::spawn(move || {
    |                                 ------- value moved into closure here
123 |         let mut num = counter.lock().unwrap();
    |                       ------- variable moved due to use in closure
...
133 |     println!("Result: {}", *counter.lock().unwrap());
    |                             ^^^^^^^^^^^^^^ value borrowed here after move
```

なるほど！最初のエラーメッセージは、`handle`に紐づけられたスレッドのクロージャに`counter`がムーブされていることを示唆しています。
そのムーブにより、それに対して`lock`を呼び出し、結果を2番目のスレッドの`num2`に保持しようとした時に、`counter`をキャプチャすることを妨げています。

故にコンパイラは`counter`の所有権を複数のスレッドに移すことはできないと教えてくれています。
これは以前では確認しづらかったことです。なぜならスレッドはループの中にあり、ループの違う繰り返しにある違うスレッドをコンパイラは指し示すことができないからです。
第15章で学んだ複数所有権メソッドでこのエラーを修正しましょう。

## 複数所有権

第15章でスマートポインタの`Rc<T>`を使用して参照カウントの値を作ることで1つの値に複数の所有者を与えました。
ここでも同じことをしてどうなるのかみてみましょう。
`Rc<T>`に`Mutex<T>`を包含し、所有権をスレッドに移す前に`Rc<T>`をクローンします。

```rust
let counter = Rc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Rc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();

        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("Result: {}", *counter.lock().unwrap());
```

まだコンパイルはできません。エラーメッセージは次のとおりです。

```
error[E0277]: `Rc<Mutex<i32>>` cannot be sent between threads safely
   --> src/main.rs:98:36
    |
98  |           let handle = thread::spawn(move || {
    |                        ------------- ^------
    |                        |             |
    |  ______________________|_____________within this `[closure@src/main.rs:98:36: 98:43]`
    | |                      |
    | |                      required by a bound introduced by this call
99  | |             let mut num = counter.lock().unwrap();
100 | |
101 | |             *num += 1;
102 | |         });
    | |_________^ `Rc<Mutex<i32>>` cannot be sent between threads safely
    |
    = help: within `[closure@src/main.rs:98:36: 98:43]`, the trait `Send` is not implemented for `Rc<Mutex<i32>>`
note: required because it's used within this closure
   --> src/main.rs:98:36
    |
98  |         let handle = thread::spawn(move || {
    |                                    ^^^^^^^
note: required by a bound in `spawn`
   --> /Users/ittoku/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/thread/mod.rs:704:8
    |
704 |     F: Send + 'static,
    |        ^^^^ required by this bound in `spawn`
```

とても長いエラーメッセージですが、注目すべきところは`error[E0277]: Rc<Mutex<i32>> cannot be sent between threads safely`です。
そしてその理由について述べているところは`the trait Send is not implemented for Rc<Mutex<i32>>`です。
`Send`については次節で説明しますが、スレッドと共に使用している型が並行な場面で使用されることを意図したものであることを保証するトレイトに1つです。

残念ながら`Rc<T>`はスレッド間で共有するには安全ではないのです。
`Rc<T>`が参照カウントを管理する際、`clone`が呼び出されるたびにカウントを追加し、クローンがドロップされるたびにカウントを差し引きます。

つまり並行基本型を使用してカウントの変更が別のスレッドに妨害されないことを確認しません。
これは間違ったカウントにつながる可能性があり、メモリリークやドロップされることにつながる可能性のある潜在的なバグです。
必要なのはいかにも`Rc<T>`のようだけれども参照カウントへの変更をスレッドセーフに行うものです。


## Arc\<T>で参照カウント

幸いなことに`Arc<T>`は`Rc<T>`のような並行な状況で安全に使用できる型です。
`a`は`atomic`を表し、原始的に参照カウントする型を意味します。
アトミックはここでは詳しく説明しない並行性の別の基本型です。詳細は`std::sync::atomic`の標準ライブラリのドキュメントをご覧ください。
今のところアトミックは、基本型のように動くけれどもスレッド間で共有しても安全なことだけ知っていれば良いです。

ではなぜすべての基本型がアトミックでなく、標準ライブラリの型も標準で`Arc<T>`を使って実装されていないのでしょうか。
その理由はスレッド安全性が本当に必要な時だけ支払いたいパフォーマンスの犠牲と共に得られるものだからです。
シングルスレッドで値に処理を施すだけなら、アトミックが提供する補償を強制する必要がない方がコードはより早く走るのです。

例に戻りましょう。`Arc<T>, Rc<T>`は同じAPIなので、`use, new, clone`を変更してプログラムを修正してみましょう。

```rust
let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();

        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("Result: {}", *counter.lock().unwrap());
```

上記のコードを実行すると以下のような出力になります。

```
Result: 10
```

0から10まで数えることができました！
これはあまり印象的ではないように思えるかもしれませんが、本当に`Mutex<T>`とスレッド安全性についていろいろなことを教えてくれました。
このプログラムの構造を利用して、カウンタをインクリメントする以上の複雑な処理を行うこともできます。
例えば、計算を独立した部分に小分けしてその部分をスレッドに分割し、それから`Mutex<T>`を使用して各スレッドに最終結果を更新させることもできます。

## RefCell\<T>, Rc\<T>, Mutex\<T>, Arc\<T>

`counter`は不変なのに、その内部にある値への可変参照を得ることができたことに気づいたでしょうか。
つまり`Mutex<T>`は`Cell`系のように内部可変性を提供します。
第15章で`RefCell<T>`を使用して`Rc<T>`の内容を可変化できるようにしたのと同様に、`Mutex<T>`を使用して`Arc<T>`の内容を可変化しているのです。

気づいておくべき別の詳細は、`Mutex<T>`を使用する際にあらゆる種類のロジックエラーからはコンパイラは保護してくれないということです。
第15章で`Rc<T>`は循環参照を生成してしまうリスクを伴いメモリリークを引き起こしてしまうと説明しました。
同様に`Mutex<T>`でもデッドロックを生成するリスクを伴っています。

これは処理が2つのリソースをロックする必要があり、2つのスレッドがそれぞれにロックを1つ獲得して永久にお互いを持ち合ってしまう時に起こります。
デッドロックの詳細についてはデッドロックのあるRustプログラムを組んでみてください。
それからどんな言語でも良いのでミューテックスに対してデッドロックを緩和する方法を調べてRustで実装してみてください。
その際には`Mutex<T>, MutexGuard`に関する標準ライブラリのドキュメントが役に立ちます。

次節では`Send, Sync`トレイトの独自の型で使用する方法についてみていきます。
