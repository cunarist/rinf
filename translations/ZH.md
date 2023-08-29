> 感谢您的帮助！非英语语言的文档可能会有语法不太通顺的问题。如果您想要为文档的改进做出贡献，请在 [Pull request](https://github.com/cunarist/rust-in-flutter/pulls) 留下您的意见。我们随时欢迎您的帮助，再次感谢！

[![Pub Version](https://img.shields.io/pub/v/rust_in_flutter)](https://pub.dev/packages/rust_in_flutter)
[![Pub Popularity](https://img.shields.io/pub/popularity/rust_in_flutter)](https://pub.dev/packages/rust_in_flutter)
[![Pub Points](https://img.shields.io/pub/points/rust_in_flutter)](https://pub.dev/packages/rust_in_flutter)
[![GitHub Stars](https://img.shields.io/github/stars/cunarist/rust-in-flutter)](https://github.com/cunarist/rust-in-flutter/stargazers)
[![Build Test](https://github.com/cunarist/rust-in-flutter/actions/workflows/build_test.yaml/badge.svg)](https://github.com/cunarist/rust-in-flutter/actions/workflows/build_test.yaml?query=branch%3Amain)
[![GitHub License](https://img.shields.io/github/license/cunarist/rust-in-flutter)](https://github.com/cunarist/rust-in-flutter/blob/main/LICENSE)

# 🆎 Rust-In-Flutter

**「以 Rust 作为您的 Flutter 后端，以 Flutter 作为您的 Rust 前端。」**

![preview](https://github.com/cunarist/rust-in-flutter/assets/66480156/be85cf04-2240-497f-8d0d-803c40536d8e)

旨在提供易用性、未来可扩展性和无与伦比的性能，这个轻量级框架在幕后处理所有复杂性。无需操心敏感的构建文件，开发过程中也不会出现过多的代码生成。只需将此包添加到您的应用项目中，您就可以轻松编写 Flutter 和 Rust 代码！

## 平台支持

Flutter 支持的所有平台都经过了[测试](https://github.com/cunarist/rust-in-flutter/actions/workflows/build_test.yaml?query=branch%3Amain)并得到支持。具有挑战性的构建设置会由该软件包自动处理。

| Dev OS  | Linux | Android | Windows | macOS | iOS | Web |
| ------- | ----- | ------- | ------- | ----- | --- | --- |
| Linux   | ✅    | ✅      | -       | -     | -   | ✅  |
| Windows | -     | ✅      | ✅      | -     | -   | ✅  |
| macOS   | -     | ✅      | -       | ✅    | ✅  | ✅  |

## 好处

- Rust 与 Flutter 集成，能够使用任意数量的库 crate
- 异步交互，无阻塞
- RESTful API，Dart 发起请求，Rust 作出响应，简单易用
- 基于 Protobuf 的类型安全且灵活的消息处理
- 从 Rust 流式传输到 Dart
- 在 Dart 的热重启上重新启动 Rust 逻辑
- 发送本地数据时无需进行内存复制

## 为什么使用 Flutter？

虽然 Rust 是用于高性能本地编程的强大语言，但其构建图形用户界面的生态系统尚不成熟。尽管它已经拥有一些 GUI 框架，如`iced`、`egui`、`gtk-rs`等，但它与 Flutter 提供的广泛支持和流畅的开发体验相比不具竞争力。唯有 Flutter 可以从单一代码库编译出适用于所有 6 个主要平台。

Flutter 是一个功能强大且多用途的框架，以其构建具有惊人用户界面的跨平台应用程序而广受欢迎。它提供声明性模式、美观的小部件、热重载、方便的调试工具，以及专门用于用户界面的预置包。

## 为什么使用 Rust？

虽然 Dart 是一种出色的、面向对象的、现代化的语言，但由于它具有垃圾回收等特性，性能并不是极致的。这就是 Rust 的用武之地。Rust 的性能被认为比 Dart 快大约[2~40 倍](https://programming-language-benchmarks.vercel.app/dart-vs-rust)(甚至无需使用多线程)。

Rust 在 Stack Overflow 上被评为[最受喜爱的编程语言](https://survey.stackoverflow.co/2022#section-most-loved-dreaded-and-wanted-programming-scripting-and-markup-languages)，其原生性能得益于零转换抽象哲学，确保高生产力。许多开发者认为 Rust 有望在未来取代 C++。Rust 的简单性、内存安全性、在各种场景下的优异性能、充满活力的社区以及强大的工具支持共同促使其日益受欢迎。

# 🛠️ 安装组件

> 这部分假设您的系统已安装[Flutter SDK](https://docs.flutter.dev/get-started/install)。

安装 Rust 工具链非常简单。只需前往[官方安装页面](https://www.rust-lang.org/tools/install)并按照说明进行操作即可。

同时，您还需要在您的系统上安装[Protobuf](https://protobuf.dev/)编译器。对于那些不太熟悉的人，Protobuf 是由 Google 制作的一种流行的、与语言无关的用于结构化消息的二进制序列化格式。按照[官方文档](https://grpc.io/docs/protoc-installation/)中的描述安装 Protobuf 编译器也非常简单。

安装完成后，请检查您的系统是否准备就绪。Flutter SDK 可能需要一些额外的组件来定位不同的平台。如果输出没有问题，您就可以开始使用了！

```bash
rustc --version
protoc --version
flutter doctor
```

# 👜 应用 Rust 模板

> 本部分假设您已经创建了一个 Flutter 项目。如果尚未创建，请按照[这个很棒的官方教程](https://docs.flutter.dev/get-started/codelab)进行创建。

首先，将此包添加到您的 Flutter 项目中。

```bash
flutter pub add rust_in_flutter
```

只需在命令行中运行以下命令：

```bash
dart run rust_in_flutter template
```

运行命令后，会出现一些新的文件和文件夹，它们将成为 Rust 项目的初始模板。

```diff
    my_flutter_project/
    ├── android/
    ├── ios/
    ├── lib/
*   │   ├── main.dart
    │   └── ...
    ├── linux/
+   ├── messages/
+   │   ├── interaction.proto
+   │   └── sample_schemas.proto
+   ├── native/
+   │   ├── hub/
+   │   │   ├── src/
+   │   │   ├── build.rs
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

别忘了先阅读`./native/README.md`文件。代码里提供了大量的注释说明，以帮助您理解代码的结构。此外，您可能需要在生产环境中移除`sample_crate`。如果您已经有了要在这里使用的 Rust crate，只需把它放在`./native`目录中，并将其设置为 `hub` crate 的一个依赖。

现在请前往 `./native/hub/src/lib.rs`，您可以开始编写 Rust 代码了！

# 👟 运行与构建

## 适用于原生平台

以下命令可用于运行和构建适用于原生平台的 Flutter 应用程序。

运行应用程序：

```bash
flutter run
```

为特定平台构建应用程序：

```bash
flutter build (platform)  # Replace it with a platform name
```

## 适用于 Web

在运行或构建 Web 上的应用程序之前，您需要从 Rust 手动构建 WebAssembly 模块。请注意，已知 Flutter 应用程序在 Web 上的调试模式下可能会相当慢。

启动 Web 应用程序：

```bash
dart run rust_in_flutter wasm
flutter run --profile  # Choose a browser
```

构建优化的发布版本 Web 应用程序：

```bash
dart run rust_in_flutter wasm --release
flutter build web
```

# 🧱 如何编写代码

## 从 Dart 发送请求，从 Rust 接收响应

随着您的应用程序规模的扩大，您将需要定义新的 Rust API 端点。

假设您想要创建一个新的按钮，该按钮可以将一组数字和一个字符串从 Dart 发送到 Rust 以进行一些计算。您可以按照以下步骤来了解如何发送请求并等待响应。

让我们从我们的[默认示例](https://github.com/cunarist/rust-in-flutter/tree/main/example)开始。在 Dart 中创建一个能够接受用户输入的按钮小部件。

```dart
// lib/main.dart
...
child: Column(
  mainAxisAlignment: MainAxisAlignment.center,
  children: [
    ElevatedButton(
      onPressed: () async {},
      child: Text("Request to Rust"),
    ),
...
```

然后使用 Protobuf 创建消息模式。

```proto
// messages/interaction.proto
...

message SomeDataGetRequest {
  repeated int32 input_numbers = 1;
  string input_string = 2;
}

message SomeDataGetResponse {
  repeated int32 output_numbers = 1;
  string output_string = 2;
}
```

接下来，从`.proto`文件生成 Dart 和 Rust 的消息代码。此命令会调用`./native/hub/build.rs`文件。

```bash
cargo check
```

`onPressed` 函数应该向 Rust 发送一个请求。让我们首先创建一个 `RustRequest` 对象。

```dart
// lib/main.dart
...
import 'package:my_flutter_project/messages/interaction.pbserver.dart';
import 'package:rust_in_flutter/rust_in_flutter.dart';
...
ElevatedButton(
  onPressed: () async {
    final requestMessage = SomeDataGetRequest(
      inputNumbers: [3, 4, 5],
      inputString: 'Zero-cost abstraction',
    );
    final rustRequest = RustRequest(
      address: 'my-category/some-data',
      operation: RustOperation.Read,
      bytes: requestMessage.writeToBuffer(),
    );
    final rustResponse = await requestToRust(rustRequest)
  },
  child: Text("Request to Rust"),
),
...
```

`address` 可以是指向适合您应用程序设计的虚拟资源的任何字符串，表示为由斜杠分隔的 kebabcase 格式的字符串层级。`operation` 可以是 create、read、update 和 delete 中的一个，因为此系统遵循 RESTful API 的定义。正如名称所示，`bytes` 只是一个简单的字节数组，通常由 Protobuf 序列化创建。

`requestToRust` 函数将请求发送给 Rust，并返回一个 `RustResponse` 对象。

因此，我们的新 API 地址是 `my-category/some-data`。请确保 Rust 中的请求处理函数接受这个地址。

```rust
// native/hub/src/with_request.rs
...
use crate::bridge::api::RustResponse;
use crate::sample_functions;
...
let layered: Vec<&str> = rust_request.address.split('/').collect();
let rust_response = if layered.is_empty() {
    RustResponse::default()
} else if layered[0] == "basic-category" {
    if layered.len() == 1 {
        RustResponse::default()
    } else if layered[1] == "counter-number" {
        sample_functions::calculate_something(rust_request).await
    } else {
        RustResponse::default()
    }
} else if layered[0] == "my-category" {
    if layered.len() == 1 {
        RustResponse::default()
    } else if layered[1] == "some-data" {
        sample_functions::some_data(rust_request).await
    } else {
        RustResponse::default()
    }
} else {
    RustResponse::default()
};
...
```

这里的 `sample_functions::some_data` 是我们的新端点 Rust 函数。这个简单的 API 端点将在数组中的每个元素上加一，将字符串中的所有字母转换为大写，然后将它们返回。消息模式在 `match` 语句中被导入，因为它会根据操作类型而不同。

```rust
// native/hub/src/sample_functions.rs
...
use crate::bridge::api::RustOperation;
use crate::bridge::api::RustRequest;
use crate::bridge::api::RustResponse;
...
pub async fn some_data(rust_request: RustRequest) -> RustResponse {
    match rust_request.operation {
        RustOperation::Create => RustResponse::default(),
        RustOperation::Read => {
            use crate::messages::interaction::{SomeDataGetRequest, SomeDataGetResponse};

            let request_message = SomeDataGetRequest::decode(&rust_request.bytes[..]).unwrap();

            let new_numbers: Vec<i32> = request_message
                .input_numbers
                .into_iter()
                .map(|x| x + 1)
                .collect();
            let new_string = request_message.input_string.to_uppercase();

            let response_message = SomeDataGetResponse {
                output_numbers: new_numbers,
                output_string: new_string,
            };
            RustResponse {
                successful: true,
                bytes: response_message.encode_to_vec(),
            }
        }
        RustOperation::Update => RustResponse::default(),
        RustOperation::Delete => RustResponse::default(),
    }
}
...
```

最后，在 Dart 中收到来自 Rust 的响应后，您可以对其中的字节数据进行任何操作。

```dart
// lib/main.dart
...
import 'package:my_flutter_project/messages/interaction.pbserver.dart';
import 'package:rust_in_flutter/rust_in_flutter.dart';
...
    final rustResponse = await requestToRust(rustRequest);
    final responseMessage = SomeDataGetResponse.fromBuffer(
      rustResponse.bytes,
    );
    print(responseMessage.outputNumbers);
    print(responseMessage.outputString);
  },
  child: Text("Request to Rust"),
),
...
```

而且我们可以在命令行中看到打印输出！

```
flutter: [4, 5, 6]
flutter: ZERO-COST ABSTRACTION
```

我们在这里仅仅简单地打印了消息，但实际应用中，响应数据将用于重建 Flutter 的 Widget。

您可以扩展这种 RESTful API 模式，并根据需要创建成百上千个端点函数。如果您具有 Web 开发背景，这种编写代码的方式可能会让您觉得很熟悉。

## 从 Rust 到 Dart 的数据流

假设您希望每秒从 Rust 发送递增的数字到 Dart。在这种情况下，Dart 重复发送请求是低效的，这时就需要使用数据流(streaming)。

还是让我们从[官方案例](https://github.com/cunarist/rust-in-flutter/tree/main/example)开始，在 Rust 中创建一个异步函数：

```rust
// native/hub/src/lib.rs
...
mod sample_functions;
...
crate::spawn(sample_functions::keep_drawing_mandelbrot());
crate::spawn(sample_functions::keep_sending_numbers()); // ADD THIS LINE
while let Some(request_unique) = request_receiver.recv().await {
...
```

定义消息模式。

```proto
// messages/interaction.proto
...
message IncreasingNumbersSignal { int32 current_number = 1; }
...
```

从`.proto`文件生成 Dart 和 Rust 消息代码。

```bash
cargo check
```

定义一个异步的 Rust 函数，它会持续运行，每秒向 Dart 发送数字。

```rust
// native/hub/src/sample_functions.rs
...
use crate::bridge::api::RustSignal;
use crate::bridge::send_rust_signal;
...
pub async fn keep_sending_numbers() {
    let mut current_number: i32 = 1;
    loop {
        use crate::messages::interaction::IncreasingNumbersSignal;

        crate::time::sleep(std::time::Duration::from_secs(1)).await;

        let signal_message = IncreasingNumbersSignal { current_number };
        let rust_signal = RustSignal {
            address: String::from("my-category/increasing-numbers"),
            bytes: signal_message.encode_to_vec(),
        };
        send_rust_signal(rust_signal);

        current_number += 1;
    }
}
...
```

最后，在 Dart 中使用`StreamBuilder`接收信号，使用`where`方法按地址进行过滤，并重新构建小部件：

```dart
// lib/main.dart
...
import 'package:my_flutter_project/messages/interaction.pbserver.dart';
import 'package:rust_in_flutter/rust_in_flutter.dart';
...
children: [
  StreamBuilder<RustSignal>(
    stream: rustBroadcaster.stream.where((rustSignal) {
      return rustSignal.address == "my-category/increasing-numbers";
    }),
    builder: (context, snapshot) {
      final received = snapshot.data;
      if (received == null) {
        return Text("Nothing received yet");
      } else {
        final singal = IncreasingNumbersSignal.fromBuffer(
          received.bytes,
        );
        final currentNumber = singal.currentNumber;
        return Text(currentNumber.toString());
      }
    },
  ),
...
```

## ✋ 常见问题

**Q**. 我应该在何时使用 Rust?

**A**. 在理想情况下，**Flutter** 将处理 GUI 界面，而 **Rust** 负责底层业务逻辑。前端和后端可以完全分离，这意味着 Dart 和 Rust 代码可以相互独立。

**Q**. Dart 和 Rust 代码之间通过什么数据类型传递数据?

**A**. 在 Dart 和 Rust 之间传递的数据一般是字节数组(bytes array)，Dart 中称之为 `Uint8List`，而 Rust 中称之为`Vec<u8>`。虽然我们推荐使用`MessagePack`进行序列化，但您也可以发送任何类型的字节数据，例如高分辨率图像或某种文件。若您不需要发送额外的数据信息，可以直接发送一个空的字节数组。

**Q**. Rust 项目生成的动态链接库在哪儿?

**A**. Rust-In-Flutter 确保了从 Rust crates 编译的所有库文件能被正确地包含在最终构建的产物中，并已准备好通过 Flutter 应用进行分发。因此，您无需考虑如何打包出动态库以及应该把它放到哪儿的问题。

**Q**. 打包 Android 应用时出现了问题?

**A**. 对于 Android 应用，您应该使用 Rust 1.68 或更高版本，具体原因可以查看[这里](https://github.com/rust-lang/rust/pull/85806)。另外，`./android/app/build.gradle`中的`ndkVersion`变量可能需要修改。如果您使用 Flutter SDK 3.7 或更早的版本创建了 Flutter 项目，也可能会缺少该变量。请访问[这里](https://github.com/cunarist/rust-in-flutter/discussions/60)来解决这个问题。

**Q**. 您遇到了其他的问题?

**A**. 在 Rust 中使用各种不同的构建目标时，也许会遇到问题。不管遇到任何情况，您可以随时到[讨论页](https://github.com/cunarist/rust-in-flutter/discussions)发起一个 Q&A 来寻求帮助！请访问该页面以阅读额外的指南并提问。

**Q**. 并发在底层是如何工作的？

**A**. 在本地平台上，Dart 像往常一样在单个线程中运行，而 Rust 则利用异步的`tokio`运行时，以充分利用计算机上的所有核心，使得异步任务能够高效地在该运行时中运行。在 Web 上，Dart 仍然在主线程中运行，但 Rust 仅在单个 Web Worker（线程）中运行。这是一个必要的限制，因为 Web Worker 不共享内存，但仍然可以通过将 Rust 的`Future`转换为 JavaScript 的`Promise`并将它们传递到 JavaScript 事件循环中，在这一个专用线程内执行并发操作。

**Q**. 如何使消息完全类型安全？

**A**. 当使用 MessagePack 序列化时，诸如 VSCode 等 IDE 可能无法提供完整的智能感知支持，从而可能导致类型相关的问题。为了确保在 IDE 中进行完全类型安全的检查，您可以选择使用 Protobuf 替代 MessagePack。Protobuf 是一种类型安全的序列化方法，有助于避免数据类型相关的错误。尽管将 Protobuf 集成到此包之外超出了范围，但您可以参考其[官方文档](https://protobuf.dev/)来了解实现细节。

**Q**. 构建后的 Web 版本在浏览器控制台显示与跨域策略有关的错误。

**A**. 在构建好二进制文件并准备部署之后，请确保配置您的网络服务器以在其响应中包含与跨域相关的 HTTP 标头。将 `cross-origin-opener-policy` 设置为 `same-origin`，`cross-origin-embedder-policy` 设置为 `require-corp` 或 `credentialless`。这些标头使得使用您的网站的客户端能够访问所需的 `SharedArrayBuffer` 网络 API，而该 API 是该框架所必需的。`SharedArrayBuffer` 在网络上类似于共享内存的功能。

**Q**. Rust 代码的更改会在 Dart 的热重启中生效吗？

**A**. 不会，在 Dart 的热重启中，更新的 Rust 代码无法加载。为了使更改生效，需要重新编译应用程序，因为必须将应用程序二进制文件链接到新编译的 Rust 库文件。这是由 Rust 的编译过程引起的限制，因为 Rust 本身不支持热重启特性。不过，Dart 的热重启会重新启动 Rust 逻辑，换句话说，会重新启动`main()`函数。

# 🌟 贡献者们

我们感谢您对这个项目开发的贡献！

[![GitHub 贡献者（通过 allcontributors.org）](https://contrib.rocks/image?repo=cunarist/rust-in-flutter)](https://github.com/cunarist/rust-in-flutter/graphs/contributors)

# ☕ 支持我们

如果您从 Rust-In-Flutter 的功能中受益，并认为它对您非常有帮助，为什么不考虑下支持这个项目呢？您的慷慨捐助将有助于 Rust-In-Flutter 项目的维护和开发，确保其不断改进、发展！ 😉

若有此想法，您可以[打赏一下](https://www.buymeacoffee.com/cunarist)我们。
