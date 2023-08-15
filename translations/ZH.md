> 感谢您的帮助！非英语语言的文档可能会有语法不太通顺的问题。如果您想要为文档的改进做出贡献，请在 [Pull request](https://github.com/cunarist/rust-in-flutter/pulls) 留下您的意见。我们随时欢迎您的帮助，再次感谢！

[![Pub Version (including pre-releases)](https://img.shields.io/pub/v/rust_in_flutter)](https://pub.dev/packages/rust_in_flutter)
[![Pub Popularity](https://img.shields.io/pub/popularity/rust_in_flutter)](https://pub.dev/packages/rust_in_flutter)
[![Pub Points](https://img.shields.io/pub/points/rust_in_flutter)](https://pub.dev/packages/rust_in_flutter)
![GitHub stars](https://img.shields.io/github/stars/cunarist/rust-in-flutter)
[![Build Test](https://img.shields.io/github/actions/workflow/status/cunarist/rust-in-flutter/ci.yaml)](https://github.com/cunarist/rust-in-flutter/actions/workflows/ci.yaml)
[![GitHub License](https://img.shields.io/github/license/cunarist/rust-in-flutter)](./LICENSE)

# 🆎 Rust-In-Flutter

快速集成 Rust 代码到您的 Flutter 项目当中！

![preview](https://github.com/cunarist/rust-in-flutter/assets/66480156/be85cf04-2240-497f-8d0d-803c40536d8e)

无需烦恼于敏感的构建文件，无需在开发过程中进行复杂的代码生成。一切都已事先准备好，即插即用。

兼具易用性、可拓展性和强悍性能的轻量级框架，在幕后为您默默处理所有问题！只需要在项目依赖中加入这个库，就可以开始编写您的 Rust 代码！

## 平台支持

Flutter 支持的所有平台都经过了[测试](https://github.com/cunarist/rust-in-flutter/actions/workflows/ci.yaml)并得到支持。此外，所有复杂的构建设置都会自动处理。

| Dev OS  | Linux | Android | Windows | macOS | iOS | Web |
| ------- | ----- | ------- | ------- | ----- | --- | --- |
| Linux   | ✅    | -       | ✅      | -     | -   | ✅  |
| Windows | ✅    | -       | -       | -     | ✅  | ✅  |
| macOS   | ✅    | ✅      | -       | ✅    | -   | ✅  |

## 好处

- Rust 与 Flutter 集成，能够使用任意数量的库 crate
- 异步交互，无阻塞
- RESTful API，Dart 发起请求，Rust 作出响应，简单易用
- 从 Rust 流式传输到 Dart
- 在 Dart 的热重启上重新启动 Rust 逻辑
- 发送本地数据时无需进行内存复制

这是中文版本。

## 为什么使用 Rust？

虽然 Dart 是一种出色的、面向对象的、现代化的语言，但由于它具有垃圾回收等特性，性能并不是极致的。这就是 Rust 的用武之地。Rust 的性能被认为比 Dart 快大约[2~40 倍](https://programming-language-benchmarks.vercel.app/dart-vs-rust)(甚至无需使用多线程)。

Rust 在 Stack Overflow 上被评为[最受喜爱的编程语言](https://survey.stackoverflow.co/2022#section-most-loved-dreaded-and-wanted-programming-scripting-and-markup-languages)，其原生性能得益于零转换抽象哲学，确保高生产力。许多开发者认为 Rust 有望在未来取代 C++。Rust 的简单性、内存安全性、在各种场景下的优异性能、充满活力的社区以及强大的工具支持共同促使其日益受欢迎。

要深入了解 Rust 的世界，请查阅官方书籍：[https://doc.rust-lang.org/book/foreword.html](https://doc.rust-lang.org/book/foreword.html)。

# 🛠️ 安装 Rust 工具链

> 这部分假设您的系统已安装[Flutter SDK](https://docs.flutter.dev/get-started/install)。

安装 Rust 工具链非常简单。只需前往[官方安装页面](https://www.rust-lang.org/tools/install)并按照说明进行操作即可。

Rust 工具链安装完成后，请检查系统是否准备就绪。Flutter SDK 可能需要一些额外的组件来定位各种平台。如果输出没有问题，你就可以开始使用了！

```bash
rustc --version
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

随着您的应用程序变得越来越大，您将需要定义新的 Rust API 端点(函数)。假设您想在 Flutter 页面中创建一个新的按钮，点击按钮后在 Dart 中将一个 int 类型的数组和一个字符串发送到 Rust，以便在 Rust 中执行一些计算。您可以按照以下步骤来了解如何发送请求并等待响应。

让我们从[官方案例](https://github.com/cunarist/rust-in-flutter/tree/main/example)开始。在 Dart 中创建一个接受用户输入的按钮小部件：

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

`onPressed`函数应该向 Rust 发送请求。让我们首先创建一个`RustRequest`对象：

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

`address`的值可以是任何适合您的应用程序 API 的字符串，表示为由点分隔的驼峰命名的字符串组合。`operation`可以是 create、read、update 和 delete 中的一个，因为`rust_in_flutter`遵循`RESTful API`的定义。正如其名称所示，`bytes`只是一个简单的字节数组，通常由[MessagePack](https://msgpack.org/)序列化创建。

现在我们应该将此请求发送到 Rust。`requestToRust`函数完成了这个工作，它返回一个`RustResponse`对象。

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

根据之前的命名，我们的新 API 地址是`myCategory.someData`。确保 Rust 中的请求处理程序函数接受此`address`：

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

被调用的`sample_functions::some_data`就是我们新的端点 Rust 函数。这个简单的 API 端点会将数组中的每个元素加一，将字符串中的所有字母转换为大写，然后将它们返回。消息模式在匹配语句中定义，因为它会根据操作类型而有所不同：

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

当您在 Dart 中收到 Rust 的响应后，您可以对其中的字节数据进行任意处理：

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

然后可以在命令行中看到打印输出：

```
flutter: [4, 5, 6]
flutter: ZERO-COST ABSTRACTION
```

我们在这里仅仅简单地打印了消息，但实际应用中，响应数据将用于重建 Flutter 的 Widget。

您可以扩展这种 RESTful API 模式，并根据需要创建成百上千个端点函数。如果您具有 Web 开发背景，这种编写代码的方式可能会让您觉得很熟悉。

## 从 Rust 到 Dart 的数据流

假设您希望每秒从 Rust 发送递增的数字到 Dart。在这种情况下，Dart 重复发送请求是低效的，这时就需要使用数据流(streaming)。

还是让我们从[官方案例](https://github.com/cunarist/rust-in-flutter/tree/main/example)开始，在 Rust 中创建一个异步函数：

```diff
    // native/hub/src/lib.rs
    ...
    mod sample_functions;
    ...
    crate::spawn(sample_functions::keep_drawing_mandelbrot());
+   crate::spawn(sample_functions::keep_sending_numbers());
    while let Some(request_unique) = request_receiver.recv().await {
    ...
```

定义一个异步的 Rust 函数，它会无限运行，每秒向 Dart 发送数字：

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
+           crate::time::sleep(std::time::Duration::from_secs(1)).await;
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

最后，在 Dart 中使用`StreamBuilder`接收信号，使用`where`方法按地址进行过滤，并重新构建小部件：

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

## ✋ 常见问题

**Q**. 我应该在何时使用 Rust?

**A**. 在理想情况下，**Flutter** 将处理 GUI 界面，而 **Rust** 负责底层业务逻辑。前端和后端可以完全分离，这意味着 Dart 和 Rust 代码可以相互独立。

**Q**. Dart 和 Rust 代码之间通过什么数据类型传递数据?

**A**. 在 Dart 和 Rust 之间传递的数据一般是字节数组(bytes array)，Dart 中称之为 `Uint8List`，而 Rust 中称之为`Vec<u8>`。虽然我们推荐使用`MessagePack`进行序列化，但您也可以发送任何类型的字节数据，例如高分辨率图像或某种文件。若您不需要发送额外的数据信息，可以直接发送一个空的字节数组。

**Q**. 什么是 MessagePack?我们的项目为何使用它?

**A**. 我们使用[MessagePack](https://msgpack.org/)来序列化 Dart 和 Rust 之间发送的消息(正如 Rust 模板代码中所呈现的那样)，除非您有其他理由不这么做。MessagePack 是一种嵌套的二进制结构，类似于 JSON，但速度更快、体积更小。MessagePack 也支持比 JSON 更多类型的内部数据，包括二进制数据。您可以在 [这个链接](https://github.com/msgpack/msgpack/blob/master/spec.md#type-system) 里查看详细的类型系统规范。

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
