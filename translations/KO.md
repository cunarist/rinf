> 영어 이외의 언어로 된 문서에서는 문장이 어색할 수 있습니다. 문서 개선에 기여하고 싶으신 분은 [Pull request](https://github.com/cunarist/rust-in-flutter/pulls)를 남겨주세요. 언제나 도움에 감사드립니다.

# 🆎 Rust-In-Flutter

Rust를 활용해서 Flutter 앱의 속도를 극적으로 향상시켜 보세요!

![preview](https://github.com/cunarist/rust-in-flutter/assets/66480156/be85cf04-2240-497f-8d0d-803c40536d8e)

민감한 빌드 파일을 건드릴 필요도 없고, 번거로운 코드 생성 절차도 없습니다.

이 간단한 프레임워크는 가벼우면서도 사용하기 쉽고, 확장성과 성능을 모두 고려하여 설계되었습니다. 복잡한 일들은 모두 내부에서 처리됩니다. Flutter 프로젝트에 이 패키지를 추가하기만 하면 Rust 코드를 바로 작성할 수 있습니다.

## 플랫폼 지원

복잡한 빌드 설정들은 모두 자동으로 처리됩니다.

- ✅ Linux: 지원 및 작동 확인됨
- ✅ Android: 지원 및 작동 확인됨
- ✅ Windows: 지원 및 작동 확인됨
- ✅ macOS: 지원 및 작동 확인됨
- ✅ iOS: 지원 및 작동 확인됨
- ⏸️ Web: 현재는 미지원이나 [고려 중](https://github.com/cunarist/rust-in-flutter/issues/34)

## 왜 Rust를 사용해야 할까요?

Dart는 GUI 앱 개발에 최적화된 훌륭한 객체지향형 언어이지만, Garbage collection을 동원하며 네이티브가 아닌 언어이기 때문에 성능이 불충분할 때가 있습니다. 이럴 경우 Rust를 사용해야 합니다. Rust의 성능은 Dart보다 [대략 2~40배 빠르다](https://programming-language-benchmarks.vercel.app/dart-vs-rust)고 알려져 있으며, Rust에서는 멀티스레딩을 활용할 수도 있습니다.

Rust는 Stack Overflow에서 [가장 사랑받는 프로그래밍 언어](https://survey.stackoverflow.co/2022#section-most-loved-dreaded-and-wanted-programming-scripting-and-markup-languages)로 사랑받고 있습니다. 무비용 추상화 철학에 기반한 Rust의 네이티브 성능은 높은 생산성을 보장합니다. 많은 개발자들이 Rust가 미래에 C++을 대체할 가능성을 예상하고 있으며, Rust의 단순성, 메모리 안전성, 다양한 시나리오에서의 우수한 성능, 활기찬 커뮤니티, 그리고 견고한 도구 지원이 인기를 더욱 높여주고 있습니다.

Rust에 대해 더 자세히 알기 위해선 [공식 서적](https://doc.rust-lang.org/book/foreword.html)을 참고하시기 바랍니다.

# 🛠️ Rust 툴체인 설치하기

이 섹션은 [Flutter SDK](https://docs.flutter.dev/get-started/install)가 시스템에 설치되어 있다고 간주합니다.

Rust 툴체인 설치는 매우 간단합니다. [공식 설치 페이지](https://www.rust-lang.org/tools/install)의 안내를 참고하세요.

Rust 툴체인 설치가 완료되었다면, 시스템이 준비되었는지 확인하세요. 다양한 플랫폼을 대상으로 빌드하기 위해 Flutter SDK가 추가적인 설치를 요구할 수 있습니다. 터미널 출력에 아무런 문제가 포함되어 있지 않다면 다음 단계로 넘어가셔도 좋습니다.

```bash
rustc --version
flutter doctor
```

# 👜 Rust 템플릿 적용하기

이 섹션은 Flutter 프로젝트가 준비되어 있다고 간주합니다. 아직 Flutter 프로젝트를 생성하지 않았다면, [공식 안내](https://docs.flutter.dev/get-started/codelab)를 따라 만들어 주세요.

먼저, 이 패키지를 Flutter 프로젝트에 추가하세요.

```bash
flutter pub add rust_in_flutter
```

그리고 다음 명령을 터미널에서 실행하세요.

```bash
dart run rust_in_flutter:apply_template
```

이 명령을 실행하면 기본 Rust 템플릿으로서의 새로운 파일과 폴더들이 생성됩니다.

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

`./native/README.md` 파일을 반드시 먼저 읽어 보세요. 작동 구조를 이해하는 데에 필요한 주석들은 코드 안에 포함되어 있습니다. 또한, 실제 개발 시에 `sample_crate`는 필요없으니 제거하세요. 사용하고 싶은 기존의 Rust crate가 있다면 `./native` 폴더에 넣고 `hub` Crate의 Dependency로 설정하세요.

이제 `./native/hub/src/lib.rs`에서부터 Rust 코드를 작성하시면 됩니다!

# 🧱 코드 작성법

## Dart에서의 요청과 Rust에서의 응답

앱이 점점 커지면 새로운 Rust API 엔드포인트들을 정의해야 합니다.

Dart에서 Rust로 숫자 배열과 문자열을 보내어 계산을 수행하는 새로운 버튼을 만들고자 한다고 가정해봅시다. 이 과정을 한 번 따라해 보면 요청을 보내고 응답을 기다리는 방법을 이해하는 데에 도움이 될 것입니다.

[기본 예제](https://github.com/cunarist/rust-in-flutter/tree/main/example)로부터 시작해 봅시다. Dart에서 사용자 입력을 받아들일 버튼 위젯을 만드세요.

```diff
  // lib/main.dart
  ...
  child: Column(
    mainAxisAlignment: MainAxisAlignment.center,
    children: [
+     ElevatedButton(
+       onPressed: () async {},
+       child: Text("Rust로 요청하기"),
+     ),
  ...
```

`onPressed` 함수는 Rust로 요청을 보내야 합니다. 먼저 `RustRequest` 객체를 생성하세요.

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

`address`는 앱의 기능에 부합하는 주소로, 점으로 구분된 카멜케이스(camelCase) 문자열로 표현됩니다. `operation`은 RESTful API의 정의에 따라 create, read, update, delete 중 하나의 값을 가집니다. `bytes`는 단순한 바이트 배열로, 보통 [MessagePack](https://msgpack.org/) 직렬화 데이터입니다.

이제 이 요청을 Rust로 보내야 합니다. 이 작업을 수행하는 `requestToRust` 함수는 `RustResponse` 객체를 반환합니다.

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

우리의 새로운 API 주소는 `myCategory.someData`였습니다. Rust에서 이 주소를 받아들이도록 만들어 주세요.

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

`sample_functions::some_data`가 새로운 엔드포인트로서의 Rust 함수입니다. 이 간단한 API 엔드포인트는 배열의 각 요소에 1을 더하고 문자열의 모든 문자를 대문자로 변환하여 반환합니다. 메시지 스키마는 작업 유형에 따라 다르기 때문에 match 문에서 정의됩니다.

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

좋습니다! 이제 Dart에서 Rust로부터의 응답을 받았을 때, 그 안에 있는 바이트 데이터를 원하는 대로 처리할 수 있습니다.

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

이제 콘솔에 결과가 출력될 것입니다!

```
flutter: [4, 5, 6]
flutter: ZERO-COST ABSTRACTION
```

우리는 여기서 단순히 메시지를 출력하는 것으로 끝냈지만, 실제 앱에서는 이 응답 데이터가 Flutter 위젯을 다시 빌드하는 데에 활용될 것입니다.

이러한 RESTful API 패턴을 확장하여 필요에 따라 수백 개 또는 수천 개의 엔드포인트를 만들 수 있습니다. 웹 개발 경험이 있다면 이 방식이 익숙할 것입니다.

## Rust에서 Dart로의 스트리밍

Rust에서 Dart로 매 초마다 증가하는 숫자를 보내고 싶다고 가정해봅시다. 이 경우, Dart가 반복해서 요청을 보내는 것은 비효율적입니다. 이럴 때 스트리밍이 필요합니다.

[기본 예제](https://github.com/cunarist/rust-in-flutter/tree/main/example)로부터 시작해 봅시다. Rust에서 비동기 함수를 생성하세요.

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

매초마다 숫자를 Dart로 보내는 비동기 Rust 함수를 정의하세요.

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

마지막으로, `StreamBuilder`를 사용하여 Dart에서 신호를 받고, `where` 메서드로 주소를 필터링하고 위젯을 다시 빌드하도록 해 주세요.

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
+         final singal = deserialize(received.bytes) as Map;
+         final currentNumber = singal["current_number"] as int;
+         return Text(currentNumber.toString());
+       }
+     },
+   ),
  ...
```

# ✋ 자주 묻는 질문 (FAQ)

1. Rust를 언제 사용해야 하나요?

   이상적으로는 Flutter가 크로스 플랫폼 사용자 인터페이스를 처리하고 Rust가 비즈니스 로직을 담당할 것입니다. 프론트엔드와 백엔드는 완전히 구분되어 있으며 Dart와 Rust 코드는 서로 분리되는 것이 가능합니다.

2. Dart와 Rust 사이에서 데이터는 어떻게 전달되나요?

   Dart와 Rust 사이에서 전송되는 데이터는 기본적으로 바이트 배열입니다. 이는 Dart에서는 `Uint8List`로, Rust에서는 `Vec<u8>`로 표현됩니다. 추천되는 메시지 직렬화 방식은 MessagePack이지만, 고해상도 이미지 또는 파일 데이터 등 다른 종류의 바이트 데이터도 전송할 수 있습니다. 아무 정보도 포함시킬 필요가 없다면 빈 바이트 배열을 담으면 됩니다.

3. "MessagePack"은 무엇이며, 왜 권장되나요?

   MessagePack은 JSON과 유사한 구조지만, Binary이며 JSON보다 더 빠르고 더 작은 크기를 가집니다. 또한, MessagePack는 JSON에 비해 [더 많은 타입](https://github.com/msgpack/msgpack/blob/master/spec.md#type-system)을 지원합니다. Dart와 Rust 간에 전송되는 메시지의 직렬화에는 [MessagePack](https://msgpack.org/)을 사용하세요. 이는 Rust 템플릿에서 기본적으로 제공되며, 다른 이유가 없다면 MessagePack을 사용하는 것이 좋습니다.

4. Rust crate들로부터 생성된 라이브러리 파일들은 어디에 있나요?

   Rust-In-Flutter의 빌드 설정은 Rust crate들로부터 컴파일된 모든 라이브러리 파일들이 최종 빌드에 올바르게 포함되어 배포 준비가 되었음을 보장합니다. 따라서 라이브러리 파일을 번들링하는 것에 대해 걱정할 필요는 없습니다.

5. 안드로이드 앱을 빌드하는 데에 실패했습니다. 어떻게 해야 하나요?

   안드로이드 앱을 빌드하려면 [이 이슈](https://github.com/rust-lang/rust/pull/85806)로 인해 Rust 1.68 이상을 사용해야 합니다. 또한 `./android/app/build.gradle` 파일에 `ndkVersion` 변수가 존재해야 하지만, 해당 Flutter 프로젝트를 Flutter SDK 3.7 이하 버전으로 생성했다면 이 변수가 누락되어 있을 수 있습니다. 이 문제를 해결하려면 [이 토론](https://github.com/cunarist/rust-in-flutter/discussions/60)를 참고하세요.

6. 도움을 요청할 곳은 어디인가요?

   문제가 발생했다면 [토론 페이지](https://github.com/cunarist/rust-in-flutter/discussions)에서 Q&A 스레드를 열고 도움을 받으실 수 있습니다. 추가적인 안내를 읽거나 소통이 필요할 땐 이곳을 방문해 주세요.

# ☕ 개발 지원하기

😉 Rust-In-Flutter의 기능이 도움이 되었다면, 이 프로젝트를 지원해주세요. 여러분의 너그러운 후원은 Rust-In-Flutter의 유지보수에 큰 도움이 됩니다.

만약 지원해 주시기로 결정했다면, [Buy Me a Coffee 페이지](https://www.buymeacoffee.com/cunarist)로 찾아와 주세요.
