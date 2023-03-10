# クロージャ

> Ref: https://doc.rust-jp.rs/book-ja/ch13-01-closures.html

Rustのクロージャは変数に保存したり、引数として他の関数に渡すことのできる匿名関数です。
ある場所でクロージャを生成し、それから別の文脈でクロージャを呼び出して評価することができます。
関数とは異なり、呼び出されたスコープの値をキャプチャすることができます。
これらのクロージャの機能がコードの再利用や、動作のカスタマイズを行わせてくれることを見ていきましょう。

## クロージャで動作の抽象化を行う

クロージャが有用な場面についてみていきましょう。
ここでは、クロージャの記法、型推論、トレイトについてみていきます。

以下のような架空の場面を考えてみてください。
カスタマイズされたエクササイズのトレーニングプランを生成するアプリを作るスタートアップで働くことになりました。
バックエンドはRustで記述され、トレーニングプランを生成するアルゴリズムは、ユーザーの年齢、BMI、運動の好み、最近のトレーニング、指定された強弱値などの多くの要因を考慮します。
実際に使用されるアルゴリズムは、この例では重要ではありません。 重要なのは、この計算が数秒要することです。
必要な時だけこのアルゴリズムを呼び出し1度だけ呼び出すことで、必要以上にユーザーを待たすことがなくなります。

以下の例は、`simulated_expensive_calculation`関数でこの過程のアルゴリズムを呼び出すことをシミュレートしています。
この関数では、`calculating slowly`と出力し、2秒待ってから、渡した数値を返します。

```rust
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
```

次は、`main`関数を書いていきます。
ユーザーがトレーニングプランを要求したときにアプリを呼び出すコードを書いていきます。
アプリのフロントエンドと相互作用する部分は、クロージャの仕様と関係ないので、プログラムの入力の値はハードコードします。

- ユーザーの強弱値、これはユーザーがトレーニングを要求して、低強度、高強度のトレーニングがしたいかを示すときに指定します。
- 乱数、これはトレーニングプランにバリエーションを起こします。

出力は、推量されるトレーニングプランとなります。

```rust
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number,
    );
}
```

ここではハードコードにしていますが、実際のプログラムでは強弱値はフロントエンドから取得し、乱数の生成には、`rand`クレートを使用します。
`main`関数は、シミュレートされた入出力値と共に`generate_workout`関数を呼び出しています。

これで文脈ができたので、アルゴリズムに取り掛かります。
`generate_workout`関数は、このアプリの重要なビジネスロジックを含んでいます。
アプリで変更を加えるときは、主にこの関数を変更します。

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}
```

このコードには遅い計算を行う関数の呼び出しが複数あります。

`generate_workout`関数の振る舞いは、ユーザーが低強度、高強度のトレーニングか確認することです。

低強度のトレーニングは腕立て伏せや腹筋運動で、高強度のトレーニングは乱数が3なら休憩と水分補給を勧め、そうでなければ、ランニングを要求します。

もしこのコードが、データサイエンスチームが`simulated_expensive_calculation`関数を呼び出しに変更を加える必要があると決定したとしましょう。
そのような変更が起きた際に更新を簡略化するために`simulated_expensive_calculation`関数を1度だけ呼び出すようにすれば、リファクタしやすくなります。
理想は、結果が必要でなければ関数を呼び出さず、それでも1度だけ呼び出したいということです。

## 関数でリファクタリング

では`simulated_expensive_calculation`関数の重複した呼び出しを変数に抽出してみましょう。

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = 
        simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result
        );
        println!(
            "Next, do {} situps",
            expensive_result
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result
            );
        }
    }
}
```

この変更により`simulated_expensive_calculation`の呼び出しが単一化されました。
しかし、この関数は必ず1度呼び出されることになるため、この関数を使わない条件式であっても関数が呼び出されます。

クロージャを使うことで、結果が本当に必要なところだけコードを実行することができます。

## クロージャでコードを保存

クロージャは変数に保存できます。`simulated_expensive_calculation`本体を実際に、保存してみましょう。
クロージャ定義は、`Smalltalk, Ruby`のクロージャ定義と類似しています。
`|num|`の箇所は、引数がある場合に使用されます。
`{}`の中に、処理を書きます。ここは関数と同じで、最終的に`num`が返されます。

```rust
let expensive_closure = |num| {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

この`let`文は匿名関数を呼び出した結果の値ではなく、匿名関数の定義が代入されます。
つまり条件式などで、`let`で定義された変数にアクセスしなかった場合、変数は実行されません。

クロージャを実行するには、関数のように記述します。

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}
```

これで重い計算は1箇所だけ呼び出され、その結果が必要なコードを実行するだけになりました。

ところが、ここでもう1つ問題が浮上してしまいました。変数が2度以上使われる場合です。
クロージャは値ではなく式を保存します。つまり複数回クロージャが呼び出されるとその度にコードが実行されるということです。
これによって重いコードが複数回呼び出されると、ユーザーを待たせてしまいます。
クロージャの呼び出し結果を保存する`if`ブロックに固有の変数を生成することで問題を解決できますが、クロージャは他の解決法を用意しています。
その解決策はもう少し後で行い、まずはクロージャ定義に型注釈がない理由とクロージャに関わるトレイトについてみていきます。

## クロージャの型推論と注釈

クロージャでは、関数のように引数の型や戻り値の型を注釈する必要はありません。
関数では型注釈は必要です。 ユーザーに露出する明示的なインターフェイスの一部だからです。
インターフェイスを堅実に定義することは、関数が使用したり、返したりする値の型についてみんなが合意していることを保証するために重要です。
しかしクロージャはこのような露出するインターフェイスには使用されません。
変数に保存され、名前付けしたり、ライブラリの使用者にさらされることなく使用されます。

クロージャは通常短く、あらゆる任意の筋書きではなく、短い文脈でのみ関係します。
このような限定された文脈内では、コンパイラは多くの変数の型を推論できるのと似たように、引数や戻り値の型を推論することができます。

このような小さく匿名の関数で型をプログラマに注釈させることは煩わしく、コンパイラがすでに利用可能な情報と大きくかぶっています。

クロージャが必要以上に冗長になっても良いというなら、型注釈を加えることもできます。

```rust
let expensive_closure = |num: u32| -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

型注釈を付け加えると、関数の記法と酷似して見えます。
以下の例は、関数の定義と、クロージャの定義を並べたものです。

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

## クロージャを保存

トレーニング生成アプリに戻りましょう。まだコードは必要以上の回数、重い計算のクロージャを呼んでいました。
この問題を解決する1つの選択肢は、重いクロージャの結果を再利用できるように変数に保存し、クロージャを再度呼ぶ代わりに、結果が必要になる箇所それぞれでその変数を使用することです。
しかしながら、この型法は同じコードを大量に繰り返す可能性があります。

運のいいことに別の解決策として、クロージャやクロージャの呼び出し結果の値を保持する構造体を作る型法があります。
結果の値が必要な場合のみ、その構造体はクロージャを実行し、その結果の値をキャッシュします。
残りのコードは結果を保存し、再利用する責任を負いません。
このパターンは、メモ化(memorization)、遅延評価(lazy evaluation)と呼ばれます。

クロージャを保持する構造体を作成するには、クロージャの型を指定する必要があります。
クロージャを使用する構造体、列挙型、関数引数を定義するには、ジェネリクスとトレイト境界を使用します。

`Fn`トレイト境界にいくつかの型を追加することで、このトレイト境界に合致するクロージャが持つべき引数と戻り値の型を書きます。

以下の例は、クロージャとオプションの結果値を保持する`Cacher`構造体の定義を示しています。

```rust
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}
```

`Cacher`構造体は、ジェネリックな型`T`の`calculation`フィールドを持ちます。
`T`のトレイト境界は、`Fn`トレイトを使うことでクロージャであると指定しています。
`calculation`フィールドに保存したいクロージャはすべて、1つの`u32`引数を取り、`u32`を返さなければなりません。

> 注釈、関数も3つの`Fn`トレイト全てを実装しています。
> もし環境から値をキャプチャする必要がなければ、`Fn`トレイトを実装する何かが必要になるクロージャではなく、関数を使用できます。

`value`フィールドの型は、`Option<u32>`です。
クロージャを実行する前に、`value`を参照すると`None`になります。
`Cacher`を使用するコードがクロージャの結果を求めたら、`Cacher`はクロージャを実行し、その結果を`value`フィールドの`Some`列挙子に保存します。
それからコードが再度クロージャの結果を求めたら、クロージャを再実行するのではなく、`Cacher`は`Some`列挙子に保持された結果を返します。

```rust
impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}
```

呼び出し元のコードにこれらのフィールドの値を直接変えてもらうのではなく、`Cacher`に構造体のフィールドの値を管理して欲しいので、これらのフィールドは非公開になっています。

`Cacher::new`関数はジェネリックな引数の`T`を取り、`Cacher`構造体と同じトレイト境界を持つように定義しました。
それから`calculation`フィールドに指定されたクロージャと、`value`フィールドに`None`値を保持する`Cacher`インスタンスを返します。

呼び出し元のコードがクロージャの評価結果を必要としたら、クロージャを直接呼ぶ代わりに、`value`メソッドを呼びます。
このメソッドは、結果の値が`self.value`の`Some`にすでにあるかどうか確認します。
あればクロージャを再度実行することなく`Some`内の値を返します。

`self.value`が`None`なら、コードは`self.calculation`に保存されたクロージャを呼び出し、結果を将来使えるように`self.value`に保存し、その値を返します。

以下の例は、`generate_workout`でこの`Cacher`構造体を使用する型法を示しています。

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure.value(intensity),
        );
        println!(
            "Next, do {} situps",
            expensive_closure.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}
```

クロージャを変数に直接保存する代わりに、クロージャを保持する`Cacher`の新規インスタンスを保存しています。
そして結果が必要な場所それぞれで、その`Cacher`インスタンスに対して`value`メソッドを呼び出しています。
必要なだけ`value`メソッドを呼び出したり、全く呼び出さないこともでき、重い計算は最大でも1度しか実行されません。

ではこのプログラムを走らせてみてください。 条件によって、実行にかかる時間が変わっているはずです。

## Cacher実装の限界

値をキャッシュすることは、コードの他の部分でも異なるクロージャで行いたくなる一般的に有用な振る舞いです。
しかし現在の`Cacher`の実装には、他の文脈で再利用すると困難になる問題が2つあります。

1つ目の問題は、`Cacher`インスタンスが常に`value`メソッドの引数`arg`に対して同じ値になると想定していることです。
以下のテストは失敗します。

```rust
#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let _v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
```

このテストでは`Cacher`インスタンスを作成し、そこからインスタンスに対して2つの値で呼び出し、2という`arg`値の`value`呼び出しは2を返すべきと期待しています。

このテストの結果は次のとおりです。

```bash
thread 'call_with_different_values' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `2`', src/main.rs:40:5
```

問題は、最初に`c.value`を1で呼び出した時に、`Cacher`インスタンスは`self.value`に`Some(1)`を保存したことです。

これを単独の値ではなく、ハッシュマップを保持できるように`Cacher`を改変してみてください。
ハッシュマップのキーは渡される`arg`値となり、ハッシュマップの値はそのキーでクロージャを呼び出した結果になります。
`self.value`が直接`Some, None`値であることを調べる代わりに、`value`関数はハッシュマップの`arg`を調べ、存在するならその値を返します。
存在しなければ`Cacher`はクロージャを呼び出し、`arg`値に紐づけてハッシュマップに結果の値を保存します。

2つ目の問題は、引数の型に`u32`を1つ取り、返すクロージャしか受け付けていないことです。
この問題を修正するには、`Cacher`機能の柔軟性を向上させるためにジェネリックな引数を導入することです。

## クロージャで環境をキャプチャ

トレーニング生成の例においては、クロージャをインラインの匿名関数として使っただけでした。
しかしクロージャには、関数にはない追加の能力があります。
それは環境をキャプチャして、自分が定義されたスコープの変数にアクセスできることです。

```rust
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}
```

ここでは`x`は`equal_to_x`の引数でもないのに、クロージャ内で`x`を使用できています。

クロージャが環境から値をキャプチャすると、メモリを使用してクロージャ本体で使用できるようにその値を保持します。
そのメモリ使用は、環境をキャプチャしないコードを実行するようなもっと一般的な場合には払いたくないオーバーヘッドです。
関数は絶対に環境をキャプチャすることは許可されていないので、関数を定義して使えば、このオーバーヘッドを招くことは絶対にありません。

クロージャは3つの型法で環境から値をキャプチャでき、この型法は関数が引数を取れる3つの方法に直に対応します。
所有権を奪う、可変で借用する、普遍で借用するです。これらは以下のように3つの`Fn`トレイトでコード化されています。

- `FnOnce`
  - クロージャの環境として知られている内包されたスコープからキャプチャした変数を消費する。
  - キャプチャした変数を消費するために、定義された際にクロージャはこれらの所有権を奪い、自身にムーブする。
  - 名前のうち`Once`の部分は、このクロージャは同じ変数の所有権を2回以上奪うことができないことを表している。
- `FnMut`、可変で値を借用するので、環境を変更することができる。
- `Fn`、環境から値を普遍で借用する。

クロージャを生成する時、クロージャが環境を使用する方法に基づいて、コンパイラはどのトレイトを使用するか推論します。
少なくとも1回は呼び出されるので、全てのクロージャは`FnOnce`を実装しています。
キャプチャした変数をムーブしないクロージャは`FnMut`も実装し、キャプチャした変数に可変でアクセスする必要のないクロージャは、`Fn`も実装しています。
`equal_to_x`クロージャは`x`を不変で借用しているので`Fn`トレイトです。

環境でクロージャが使用している値の所有権を奪うことをクロージャに強制したいなら、引数リストの前に`move`キーワードを使用します。
このテクニックは、新しいスレッドにデータが所有されるように、クロージャを新しいスレッドに渡して、データをムーブする際に有用です。

並行性について語る第16章では`move`クロージャの例が出てきます。
とりあえずの例として、以下のコードは`move`キーワードをクロージャ定義に使用されています。

```rust
fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
```

このコードを実行すると以下のようなエラーになります。

```bash
cargo test
error[E0382]: borrow of moved value: `x`
  --> src/main.rs:76:40
   |
72 |     let x = vec![1, 2, 3];
   |         - move occurs because `x` has type `Vec<i32>`, which does not implement the `Copy` trait
73 |
74 |     let equal_to_x = move |z| z == x;
   |                      --------      - variable moved due to use in closure
   |                      |
   |                      value moved into closure here
75 |
76 |     println!("can't use x here: {:?}", x);
   |                                        ^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
```

クロージャが定義された際に、クロージャに`x`の値がムーブされています。
そしてクロージャが`x`の所有権を持っているので、`main`は`x`を使用することはできません。

`Fn`トレイトのどれかを指定する場合、`Fn`から始めるとコンパイラがクロージャ本体内で起こることで、`FnMut, FnOnce`が必要な時は教えてくれるでしょう。

環境をキャプチャできるクロージャが関数の引数として有用な場面を説明するために、次はイテレータについてみていきましょう。
