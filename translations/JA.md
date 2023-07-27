> 英語以外の言語で書かれたドキュメントでは、文章が不自然になることがあります。ドキュメントの改善にご協力いただける方は、[Pull request](https://github.com/cunarist/rust-in-flutter/pulls)をお願いいたします。いつもご協力に感謝いたします。

# 🆎 Rust-In-Flutter

Flutter アプリを驚くほど高速化するために、簡単に Rust を統合しましょう！

![プレビュー](https://github.com/cunarist/rust-in-flutter/assets/66480156/be85cf04-2240-497f-8d0d-803c40536d8e)

敏感なビルドファイルの心配なし、開発中の複雑なコード生成も不要です。すべてがスムーズに動作します。

この軽量なフレームワークは、使いやすさ、将来の拡張性、優れたパフォーマンスを考慮して設計されており、裏側の複雑な部分をすべて処理します。このパッケージを Flutter プロジェクトに追加するだけで、Rust のコードを簡単に書くことができます！

## プラットフォームサポート

すべての難しいビルド設定は、このパッケージによって自動的に処理されます。

- ✅ Linux：テスト済みサポート
- ✅ Android：テスト済みサポート
- ✅ Windows：テスト済みサポート
- ✅ macOS：テスト済みサポート
- ✅ iOS：テスト済みサポート
- ⏸️ Web：現時点ではサポートされていません [検討中](https://github.com/cunarist/rust-in-flutter/issues/34)

## Rust の利用理由

Dart は GUI アプリケーション向けの素晴らしいオブジェクト指向のモダンな言語ですが、ネイティブのガベージコレクションにより、要件を満たすことができない場合があります。そこで、Rust が登場し、Dart よりも約[2~40 倍高速](https://programming-language-benchmarks.vercel.app/dart-vs-rust)であり、さらに複数スレッドを活用することができます。

Rust は Stack Overflow で[最も愛されているプログラミング言語](https://survey.stackoverflow.co/2022#section-most-loved-dreaded-and-wanted-programming-scripting-and-markup-languages)として支持されています。そのネイティブなパフォーマンスは、ゼロキャストの抽象化哲学により高い生産性を実現します。多くの開発者は、将来的に Rust が C++の代替となる可能性を予測しています。Rust のシンプルさ、メモリの安全性、さまざまなシナリオでの優れたパフォーマンス、活気あるコミュニティ、堅牢なツールサポートが人気の向上に寄与しています。

Rust の世界をさらに探求するには、公式の書籍をご覧ください：[https://doc.rust-lang.org/book/foreword.html](https://doc.rust-lang.org/book/foreword.html)。

# 🛠️ Rust ツールチェーンのインストール

このセクションでは、システムに[Flutter SDK](https://docs.flutter.dev/get-started/install)がインストールされていることを前提としています。

Rust ツールチェーンのインストールは非常に簡単です。[公式のインストールページ](https://www.rust-lang.org/tools/install)にアクセスして、指示に従うだけです。

Rust ツールチェーンのインストールが完了したら、システムが正常に準備されているか確認してください。Flutter SDK は、さまざまなプラットフォームを対象とするためにいくつかの追加コンポーネントが必要になる場合があります。出力に問題がなければ、準備完了です！

```bash
rustc --version
flutter doctor
```

# 👜 Rust のテンプレートを適用する

このセクションでは、既に Flutter プロジェクトを作成したことを前提としています。まだ作成していない場合は、[この素晴らしい公式チュートリアル](https://docs.flutter.dev/get-started/codelab)に従ってプロジェクトを作成してください。

まず、このパッケージを Flutter プロジェクトに追加します。

```bash
flutter pub add rust_in_flutter
```

単純に、Flutter プロジェクトのディレクトリで以下のコマンドをコマンドラインで実行してください。

```bash
dart run rust_in_flutter:apply_template
```

このコマンドを実行した後、新しいファイルとフォルダが生成され、スターター Rust テンプレートとなります。

```diff
    my_flutter_project/
    ├── android/
    ├── ios/
    ├── lib/
*   │   ├── main.dart
    │   └── ...
    ├── linux/
+   ├── native/
+   │   ├── hub/
+   │   │   ├── src/
+   │   │   └── Cargo.toml
+   │   ├── sample_crate/
+   │   │   ├── src/
+   │   │   └── Cargo.toml
+   │   └── README.md
    ├── web/
    ├── windows/
*   ├── .gitignore
+   ├── Cargo.toml
*   ├── pubspec.yaml
    └── ...
```

`./native/README.md`ファイルを最初に読むことを忘れないでください。コードにはさまざまなコメントが記述されており、構造を理解するのに役立ちます。また、本番環境では`sample_crate`を削除することを検討するかもしれません。すでに使用したい Rust クレートがある場合は、それを`./native`内に配置し、`hub`クレートの依存として設定してください。

これで、`./native/hub/src/lib.rs`に移動して、Rust のコードを書き始めることができます！

# 🧱 コードの書き方

## Dart からのリクエスト、Rust からのレスポンス

アプリが大きくなるにつれて、新しい Rust API エンドポイントを定義する必要があります。

Dart から Rust に配列と文字列を送信し、それに対していくつかの計算を行う新しいボタンを作成したいとします。リクエストを送信してレスポンスを待つ方法を理解するために、以下の手順に従うことができます。

[デフォルトの例](https://github.com/cunarist/rust-in-flutter/tree/main/example)を起点にして、Dart でユーザーの入力を受け付けるボタンウィジェットを作成します。

```diff
  // lib/main.dart
  ...
  child: Column(
    mainAxisAlignment: MainAxisAlignment.center,
    children: [
+     ElevatedButton(
+       onPressed: () async {},
+       child: Text("Request to Rust"),
+     ),
  ...
```

`onPressed`関数は Rust にリクエストを送信する必要があります。まずは`RustRequest`オブジェクトを作成しましょう。

```diff
  // lib/main.dart
  ...
  import 'package:msgpack_dart/msgpack_dart.dart';
  import 'package:rust_in_flutter/rust_in_flutter.dart';
  ...
  ElevatedButton(
+   onPressed: () async {
+     final rustRequest = RustRequest(
+       address: 'myCategory.someData',
+       operation: RustOperation.Read,
+       bytes: serialize(
+         {
+           'input_numbers': [3, 4, 5],
+           'input_string': 'Zero-cost abstraction',
+         },
+       ),
+     );
+   },
    child: Text("Request to Rust"),
  ),
  ...
```

`address`はアプリのデザインに合った任意の文字列で、ドットで層になっているキャメルケースの文字列として表現されます。`operation`は、このシステムが RESTful API の定義に従っているため、作成、読み取り、更新、削除のいずれかになります。そして、`bytes`は単純なバイト配列であり、通常は[MessagePack](https://msgpack.org/)のシリアル化によって作成されます。

それでは、このリクエストを Rust に送信します。これを行うのは`requestToRust`関数で、`RustResponse`オブジェクトを返します。

```diff
  // lib/main.dart
  ...
  import 'package:msgpack_dart/msgpack_dart.dart';
  import 'package:rust_in_flutter/rust_in_flutter.dart';
  ...
  ElevatedButton(
    onPressed: () async {
      final rustRequest = RustRequest(
        address: 'myCategory.someData',
        operation: RustOperation.Read,
        bytes: serialize(
          {
            'input_numbers': [3, 4, 5],
            'input_string': 'Zero-cost abstraction',
          },
        ),
      );
+     final rustResponse = await requestToRust(rustRequest);
    },
    child: Text("Request to Rust"),
  ),
    ...
```

したがって、新しい API アドレスは`myCategory.someData`です。Rust のリクエストハンドラ関数がこれを受け入れることを確認してください。

```diff
    // native/hub/src/with_request.rs
    ...
    use crate::bridge::api::RustResponse;
    use crate::sample_functions;
    ...
    let layered: Vec<&str> = rust_request.address.split('.').collect();
    let rust_response = if layered.is_empty() {
        RustResponse::default()
    } else if layered[0] == "basicCategory" {
        if layered.len() == 1 {
            RustResponse::default()
        } else if layered[1] == "counterNumber" {
            sample_functions::calculate_something(rust_request).await
        } else {
            RustResponse::default()
        }
+   } else if layered[0] == "myCategory" {
+       if layered.len() == 1 {
+           RustResponse::default()
+       } else if layered[1] == "someData" {
+           sample_functions::some_data(rust_request).await
+       } else {
+           RustResponse::default()
+       }
    } else {
        RustResponse::default()
    };
    ...
```

これが新しいエンドポイントの Rust 関数である`sample_functions::some_data`です。このシンプルな API エンドポイントでは、配列の各要素に 1 を加算し、文字列のすべての文字を大文字に変換してそれらを返します。メッセージスキーマは、操作のタイプによって異なるため、マッチステートメントで定義されています。

```diff
    // native/hub/src/sample_functions.rs
    ...
    use crate::bridge::api::RustOperation;
    use crate::bridge::api::RustRequest;
    use crate::bridge::api::RustResponse;
    use rmp_serde::from_slice;
    use rmp_serde::to_vec_named;
    use serde::Deserialize;
    use serde::Serialize;
    ...
+   pub async fn some_data(rust_request: RustRequest) -> RustResponse {
+       match rust_request.operation {
+           RustOperation::Create => RustResponse::default(),
+           RustOperation::Read => {
+               #[allow(dead_code)]
+               #[derive(Deserialize)]
+               struct RustRequestSchema {
+                   input_numbers: Vec<i8>,
+                   input_string: String,
+               }
+               let slice = rust_request.bytes.as_slice();
+               let received: RustRequestSchema = from_slice(slice).unwrap();
+
+               let new_numbers = received.input_numbers.into_iter().map(|x| x + 1).collect();
+               let new_string = received.input_string.to_uppercase();
+
+               #[derive(Serialize)]
+               struct RustResponseSchema {
+                   output_numbers: Vec<i8>,
+                   output_string: String,
+               }
+               RustResponse {
+                   successful: true,
+                   bytes: to_vec_named(&RustResponseSchema {
+                       output_numbers: new_numbers,
+                       output_string: new_string,
+                   })
+                   .unwrap(),
+               }
+           }
+           RustOperation::Update => RustResponse::default(),
+           RustOperation::Delete => RustResponse::default(),
+       }
+   }
    ...
```

わかりました！Dart から Rust からのレスポンスを受け取ったら、その中のバイトデータを自由に処理できます。

```diff
  // lib/main.dart
  ...
  import 'package:msgpack_dart/msgpack_dart.dart';
  import 'package:rust_in_flutter/rust_in_flutter.dart';
  ...
  ElevatedButton(
    onPressed: () async {
      final rustRequest = RustRequest(
        address: 'myCategory.someData',
        operation: RustOperation.Read,
        bytes: serialize(
          {
            'input_numbers': [3, 4, 5],
            'input_string': 'Zero-cost abstraction',
          },
        ),
      );
      final rustResponse = await requestToRust(rustRequest);
+     final message = deserialize(rustResponse.bytes) as Map;
+     print(message["output_numbers"]);
+     print(message["output_string"]);
    },
    child: Text("Request to Rust"),
  ),
    ...
```

そして、コマンドラインに出力された結果が見られます！

```
flutter: [4, 5, 6]
flutter: ZERO-COST ABSTRACTION
```

ここでは単にメッセージを出力しているだけですが、実際のアプリではこのレスポンスデータを使用して Flutter のウィジェットをリビルドすることができます。

このような RESTful API のパターンを拡張し、必要に応じて数百、数千のエンドポイントを作成することができます。Web のバックグラウンドがある場合、このシステムは馴染みのあるものに見えるかもしれません。

## Rust から Dart へのストリーミング

Rust から Dart に毎秒増加する数値を送信したいとします。この場合、Dart が繰り返しリクエストを送信するのは非効率です。ここでストリーミングが必要になります。

[デフォルトの例](https://github.com/cunarist/rust-in-flutter/tree/main/example)を起点にします。Rust で非同期関数を生成します。

```diff
    // native/hub/src/lib.rs
    ...
    use tokio::task::spawn;
    ...
    mod sample_functions;
    ...
    spawn(sample_functions::keep_drawing_mandelbrot());
+   spawn(sample_functions::keep_sending_numbers());
    while let Some(request_unique) = request_receiver.recv().await {
    ...
```

永遠に実行される非同期 Rust 関数を定義し、毎秒数値を Dart に送信します。

```diff
    // native/hub/src/sample_functions.rs
    ...
    use crate::bridge::api::RustSignal;
    use crate::bridge::send_rust_signal;
    ...
    use rmp_serde::to_vec_named;
    ...
    use serde::Serialize;
    ...
+   pub async fn keep_sending_numbers() {
+       let mut current_number: i32 = 1;
+       loop {
+           tokio::time::sleep(std::time::Duration::from_secs(1)).await;
+
+           #[derive(Serialize)]
+           struct RustSignalSchema {
+               current_number: i32,
+           }
+           let rust_signal = RustSignal {
+               address: String::from("myCategory.increasingNumbers"),
+               bytes: to_vec_named(&RustSignalSchema {
+                   current_number: current_number,
+               })
+               .unwrap(),
+           };
+           send_rust_signal(rust_signal);
+           current_number += 1;
+       }
+   }
    ...
```

最後に、`StreamBuilder`を使用して Dart で信号を受信し、`where`メソッドでアドレスでフィルタリングし、ウィジェットを再構築します。

```diff
  // lib/main.dart
  ...
  import 'package:msgpack_dart/msgpack_dart.dart';
  import 'package:rust_in_flutter/rust_in_flutter.dart';
  ...
  children: [
+   StreamBuilder<RustSignal>(
+     stream: rustBroadcaster.stream.where((rustSignal) {
+       return rustSignal.address == "myCategory.increasingNumbers";
+     }),
+     builder: (context, snapshot) {
+       final received = snapshot.data;
+       if (received == null) {
+         return Text("Nothing received yet");
+       } else {
+         final signal = deserialize(received.bytes) as Map;
+         final currentNumber = signal["current_number"] as int;
+         return Text(currentNumber.toString());
+       }
+     },
+   ),
  ...
```

以上の内容です。

# ✋ よくある質問（FAQ）

1. Rust をいつ使用すべきですか？

   理想的には、**Flutter**がクロスプラットフォームのユーザーインターフェースを担当し、**Rust**がビジネスロジックを扱うようにします。フロントエンドとバックエンドを完全に分離することができ、Dart と Rust のコードが互いに切り離すことができます。これらの 2 つの世界はストリームを介して通信します。

2. Dart と Rust の間でデータはどのように渡されますか？

   Dart と Rust の間で送信されるデータは基本的にバイト配列です。Dart では`Uint8List`として、Rust では`Vec<u8>`として表現されます。MessagePack シリアル化の使用をお勧めしますが、高解像度の画像やファイルデータなど、任意の種類のバイトデータを送信することができます。必要な詳細がない場合は、空のバイト配列を送信することもできます。

3. "MessagePack"とは何で、なぜ推奨されていますか？

   MessagePack は JSON に似たネストされたバイナリ構造であり、より高速でより小さいです。MessagePack は JSON よりも[より多くの種類](https://github.com/msgpack/msgpack/blob/master/spec.md#type-system)の内部データをサポートしており、バイナリも含まれます。他に理由がない限り、Dart と Rust の間で送信されるメッセージをシリアル化するために、Rust のテンプレートで提供される MessagePack を使用してください。

4. Rust クレートから生成されたライブラリファイルはどこにありますか？

   Rust-In-Flutter のすべてのビルド設定により、Rust クレートからコンパイルされたすべてのライブラリファイルが最終ビルドに適切に含まれ、配布の準備が整います。そのため、ライブラリファイルをバンドルする必要はありません。

5. Android アプリのビルドに失敗しました。どうすればよいですか？

   Android アプリでは、[この問題](https://github.com/rust-lang/rust/pull/85806)のために Rust 1.68 以上を使用する必要があります。また、`./android/app/build.gradle`ファイルに`ndkVersion`変数が存在している必要がありますが、Flutter SDK 3.7 以前で Flutter プロジェクトを作成した場合には欠落している可能性があります。この問題を解決するには、[このディスカッション](https://github.com/cunarist/rust-in-flutter/discussions/60)を参照してください。

6. 助けを求める場所はどこですか？

   問題に遭遇した場合は、[ディスカッションページ](https://github.com/cunarist/rust-in-flutter/discussions)を訪れ、アシスタンスを求めるための Q&A スレッドを開いて自由に質問してください。追加のガイドを読み、質問をするためにこのページを訪れてください。

# ☕ サポートをお願いします

😉 Rust-In-Flutter の機能を活用し、役立つと感じている場合は、このプロジェクトのサポートを検討してみてください。寄付いただいた方々の寛大な支援により、Rust-In-Flutter の保守と開発が行われ、継続的な改善と成長が実現されます。

もし気に入っていただけたら、[コーヒーをおごっていただけると嬉しいです](https://www.buymeacoffee.com/cunarist)。
