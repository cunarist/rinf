> 英語以外の言語で書かれたドキュメントでは、文章が不自然になることがあります。ドキュメントの改善にご協力いただける方は、[Pull request](https://github.com/cunarist/rust-in-flutter/pulls)をお願いいたします。いつもご協力に感謝いたします。

# 🆎 Rust-In-Flutter

Flutter アプリを驚くほど高速化するために、簡単に Rust を統合しましょう！

![プレビュー](https://github.com/cunarist/rust-in-flutter/assets/66480156/be85cf04-2240-497f-8d0d-803c40536d8e)

この軽量なフレームワークは、使いやすさ、将来の拡張性、優れたパフォーマンスを考慮して設計されており、裏側の複雑な部分をすべて処理します。このパッケージを Flutter プロジェクトに追加するだけで、Rust のコードを簡単に書くことができます！

## 利点

- Rust の統合：任意のライブラリクレートの使用が可能
- 既存の Rust クレートをそのまま使用可能
- CMake、Gradle、Podfile などの機密ビルドファイルを扱う必要なし
- 開発中の複雑なコード生成なし
- 多くの労力をかけずに無制限の RESTful API エンドポイントの定義
- Dart からの簡単なリクエストと Rust からの非同期相互作用
- Rust から Dart へのストリーミング
- Dart のホットリスタート時に Rust ロジックを再起動
- 最小限のオーバーヘッド
- ネイティブデータ送信時のメモリコピーなし

## プラットフォームサポート

このパッケージによって、難しいビルド設定は自動的に処理されます。Flutter プロジェクトのファイルは影響を受けないことに注意してください。

- ✅ Linux：テスト済みサポート
- ✅ Android：テスト済みサポート
- ✅ Windows：テスト済みサポート
- ✅ macOS：テスト済みサポート
- ✅ iOS：テスト済みサポート
- ⏸️ Web：現時点ではサポートされていません [検討中](https://github.com/cunarist/rust-in-flutter/issues/34)

> 提案がある場合やバグを報告したい場合は、[issue](https://github.com/cunarist/rust-in-flutter/issues)または[pull request](https://github.com/cunarist/rust-in-flutter/pulls)として残してください。できる限り迅速に対応いたします。

## Rust の利用理由

Dart は素晴らしいオブジェクト指向の現代的な言語ですが、ネイティブでガベージコレクションされないため、パフォーマンスが要件を満たさない場合があります。そこで Rust が登場します。Rust のパフォーマンスは、Dart の約[2〜40 倍高速](https://programming-language-benchmarks.vercel.app/dart-vs-rust)であり、さらに複数のスレッドを利用できる能力もあります。

# 👜 コンポーネントのインストール

このセクションでは、すでに[Flutter SDK](https://docs.flutter.dev/get-started/install)をシステムにインストールし、`flutter create`コマンドで Flutter プロジェクトを作成しているものとします。この Flutter プロジェクトのフォルダをターミナルの作業ディレクトリとして使用してください。

まず、このパッケージを Flutter プロジェクトに追加します。

```bash
flutter pub add rust_in_flutter
```

そして、Rust ツールチェーンをインストールしてください。公式 Rust ドキュメントを参照してください：[公式 Rust ドキュメント](https://doc.rust-lang.org/book/ch01-01-installation.html)。

最後に、システムがコンパイルに対して準備ができていることを確認してください。各インストールステップの後でシステムの状態を確認するために以下のコマンドを繰り返すことができます。出力に問題がない場合は、準備が整っています！

```bash
rustc --version
flutter doctor
```

## ビルドツールバージョンの問題

- Android アプリの場合、[この問題](https://github.com/rust-lang/rust/pull/85806)のために Rust 1.68 以上を使用する必要があります。
- Android アプリの場合、`./android/app/build.gradle`内の変数`ndkVersion`の設定が必要ですが、Flutter SDK 3.7 以前で Flutter プロジェクトを作成した場合には、これが欠落している可能性があります。[このディスカッション](https://github.com/cunarist/rust-in-flutter/discussions/60)を参照して、この問題を解決してください。

> Rust を使用してさまざまなビルドターゲットを利用する際には、さまざまな問題が発生することがあります。何か問題が発生した場合は、[ディスカッションページ](https://github.com/cunarist/rust-in-flutter/discussions)を訪れ、アシスタンスのための Q&A スレッドを開いてください。

# 👜 テンプレートの適用

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

Dart からリクエストする際には、操作とアドレスを指定する必要があります。この通信方法は RESTful API の定義に従っています。

```dart
// ./lib/main.dart

import 'package:msgpack_dart/msgpack_dart.dart';
import 'package:rust_in_flutter/rust_in_flutter.dart';

void someFunction() async {
    var rustRequest = RustRequest(
      address: 'basicCategory.counterNumber',
      operation: RustOperation.Read,
      bytes: serialize(
        {
          'letter': 'Hello from Dart!',
          'before_number': 888,
          'dummy_one': 1,
          'dummy_two': 2,
          'dummy_three': [3, 4, 5]
        },
      ),
    );

    var rustResponse = await requestToRust(rustRequest);
    var message = deserialize(rustResponse.bytes) as Map;
    var innerValue = message['after_number'] as int;
}
```

Rust でリクエストを受け取った場合、まずアドレスによって分類する必要があります。

```rust
// ./native/hub/src/with_request.rs

pub async fn handle_request(request_unique: RustRequestUnique) -> RustResponseUnique {
    let rust_request = request_unique.request;
    let interaction_id = request_unique.id;

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
    } else {
        RustResponse::default()
    };

    RustResponseUnique {
        id: interaction_id,
        response: rust_response,
    }
}
```

Rust におけるエンドポイント関数は以下のようになります。メッセージスキーマは、操作の種類によって異なるため、match 文で定義されています。

```rust
// ./native/hub/src/sample_functions.rs

pub async fn calculate_something(rust_request: RustRequest) -> RustResponse {
    match rust_request.operation {
        RustOperation::Create => RustResponse::default(),
        RustOperation::Read => {
            #[allow(dead_code)]
            #[derive(Deserialize)]
            struct RustRequestSchema {
                letter: String,
                before_number: i32,
                dummy_one: i32,
                dummy_two: i32,
                dummy_three: Vec<i32>,
            }
            let slice = rust_request.bytes.as_slice();
            let received: RustRequestSchema = from_slice(slice).unwrap();
            println!("{:?}", received.letter);

            let before_value = received.before_number;
            let after_value = sample_crate::add_seven(before_value);

            #[derive(Serialize)]
            struct RustResponseSchema {
                after_number: i32,
                dummy_one: i32,
                dummy_two: i32,
                dummy_three: Vec<i32>,
            }
            RustResponse {
                successful: true,
                bytes: to_vec_named(&RustResponseSchema {
                    after_number: after_value,
                    dummy_one: 1,
                    dummy_two: 2,
                    dummy_three: vec![3, 4, 5],
                })
                .unwrap(),
            }
        }
        RustOperation::Update => RustResponse::default(),
        RustOperation::Delete => RustResponse::default(),
    }
}
```

RESTful API パターンを拡張して、必要に応じて数百から数千のエンドポイントを作成することができます。Web のバックグラウンドがある方にとっては、このシステムは馴染みがあるかもしれません。実際の Rust テンプレート内には、さらなるコメントと詳細が含まれています。

理想的には、**Flutter**はクロスプラットフォームのユーザーインターフェースを担当し、**Rust**はビジネスロジックを処理します。フロントエンドとバックエンドは完全に分離されるため、Dart と Rust のコードは互いに独立しています。これらの 2 つの世界はチャンネルとストリームを介して通信します。

[MessagePack](https://msgpack.org/)を使用して、Dart と Rust 間で送信されるメッセージを直接 Rust テンプレートで提供される形式でシリアライズします（他の理由がない限り）。MessagePack は、JSON に似た入れ子構造のバイナリ形式であり、高速かつコンパクトです。

Dart と Rust の間で送信されるデータは基本的にバイト配列であり、Dart では`Uint8List`、Rust では`Vec<u8>`として表現されます。MessagePack のシリアライズを使用することをお勧めしますが、高解像度の画像やファイルデータなど、任意のバイトデータを送信することができます。また、必要な場合は空のバイト配列を送信することもできます。

Rust-In-Flutter のすべてのビルド設定により、Rust クレートからコンパイルされたすべてのライブラリファイルが適切に最終ビルドに含まれ、配布の準備が整います。したがって、ライブラリファイルのバンドルについて心配する必要はありません。

その他の詳細なお知らせや情報は、[ディスカッションページ](https://github.com/cunarist/rust-in-flutter/discussions)で共有されています。追加のガイドを読むや質問をするには、このページを訪れてください。

# ☕ サポートをお願いします

😉 Rust-In-Flutter の機能を活用し、役立つと感じている場合は、このプロジェクトのサポートを検討してみてください。寄付いただいた方々の寛大な支援により、Rust-In-Flutter の保守と開発が行われ、継続的な改善と成長が実現されます。

もし気に入っていただけたら、[コーヒーをおごっていただけると嬉しいです](https://www.buymeacoffee.com/cunarist)。
