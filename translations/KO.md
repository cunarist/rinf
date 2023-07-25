> 영어 이외의 언어로 된 문서에서는 문장이 어색할 수 있습니다. 문서 개선에 기여하고 싶으신 분은 [Pull request](https://github.com/cunarist/rust-in-flutter/pulls)를 남겨주세요. 언제나 도움에 감사드립니다.

# 🆎 Rust-In-Flutter

Rust를 활용해서 Flutter 앱의 속도를 극적으로 향상시켜 보세요!

![preview](https://github.com/cunarist/rust-in-flutter/assets/66480156/be85cf04-2240-497f-8d0d-803c40536d8e)

이 간단한 프레임워크는 가벼우면서도 사용하기 쉽고, 확장성과 성능을 모두 고려하여 설계되었습니다. 복잡한 일들은 모두 자동으로 처리됩니다. Flutter 프로젝트에 이 패키지를 추가하기만 하면 Rust를 바로 사용할 수 있습니다.

## 이점

- Rust 코드를 사용하고 원하는 만큼의 라이브러리 Crate를 사용 가능함
- 기존에 쓰인 Rust Crate들을 그대로 사용 가능함
- CMake, Gradle, Podfile 등과 같은 민감한 빌드 파일을 건드릴 필요가 없음
- 복잡한 코드 생성 절차가 없음
- 손쉽게 원하는 만큼의 RESTful API 엔드포인트를 정의할 수 있음
- 아주 쉬운 Dart에서의 요청과 Rust로부터의 응답으로 이루어진 Async 상호작용
- Rust에서 Dart로의 스트리밍
- Dart의 Hot restart 시 자동으로 재시작되는 Rust 로직
- 최소한의 오버헤드
- 네이티브 데이터 전송 시 메모리 복사가 없음

## 플랫폼 지원

이 패키지는 어려운 빌드 설정을 모두 자동으로 처리해 주며, 작업 중인 Flutter 앱 프로젝트의 파일에는 영향을 미치지 않습니다.

- ✅ Linux: 지원 및 작동 확인됨
- ✅ Android: 지원 및 작동 확인됨
- ✅ Windows: 지원 및 작동 확인됨
- ✅ macOS: 지원 및 작동 확인됨
- ✅ iOS: 지원 및 작동 확인됨
- ⏸️ Web: 현재는 미지원이나 [고려 중](https://github.com/cunarist/rust-in-flutter/issues/34)

> 제안 사항이나 버그를 신고할 때에는 [이슈](https://github.com/cunarist/rust-in-flutter/issues) 또는 [Pull request](https://github.com/cunarist/rust-in-flutter/pulls)를 작성해주세요. 가능한 한 빠르게 답변해 드리겠습니다.

## 왜 Rust를 사용해야 할까요?

Dart는 GUI 앱 개발에 최적화된 훌륭한 객체지향형 언어이지만, Garbage collection을 동원하며 네이티브가 아닌 언어이기 때문에 성능이 불충분할 때가 있습니다. 이럴 경우 Rust를 사용해야 합니다. Rust의 성능은 Dart보다 [대략 2~40배 빠르다](https://programming-language-benchmarks.vercel.app/dart-vs-rust)고 알려져 있으며, Rust에서는 멀티스레딩을 활용할 수도 있습니다.

Rust는 Stack Overflow에서 [가장 사랑받는 프로그래밍 언어](https://survey.stackoverflow.co/2022#section-most-loved-dreaded-and-wanted-programming-scripting-and-markup-languages)로 보고되기도 했으며, 무비용 추상화 철학 덕분에 높은 생산성과 네이티브 성능을 모두 제공합니다.

물론 C와 C++도 존재하지만, 많은 개발자들은 [Rust가 결국 C++을 대체할 것이라고](https://dev.to/roy8/why-rust-will-replace-c-in-the-future-49op) 믿고 있습니다. Rust가 C++보다 인기 있는 이유는 다양합니다. 먼저, Rust는 훨씬 더 간단하고 배우기 쉽습니다. 이는 간결함과 메모리 안전성에 초점을 맞춘 설계 때문입니다. 더불어 Rust는 많은 경우 C++보다 우수한 성능을 제공합니다. 또한 더 활발한 커뮤니티와 더 나은 툴 지원을 갖추고 있습니다.

Rust에 대해 더 자세히 알고 학습하기 위해선 [공식 서적](https://doc.rust-lang.org/book/foreword.html)을 참고하시기 바랍니다.

# 👜 컴포넌트 설치

이 섹션은 [Flutter SDK](https://docs.flutter.dev/get-started/install)가 시스템에 설치되어 있고 `flutter create` 명령으로 생성된 Flutter 프로젝트가 이미 준비되어 있다고 간주합니다. 터미널의 작업 디렉토리로 준비된 Flutter 프로젝트의 폴더를 사용하세요. 아직 Flutter 프로젝트를 생성하지 않았다면, [이 안내](https://docs.flutter.dev/get-started/codelab)를 따라 만드세요.

우선, 이 패키지를 Flutter 프로젝트에 추가하세요.

```bash
flutter pub add rust_in_flutter
```

그리고 Rust toolchain을 설치하세요. [공식 Rust 문서](https://doc.rust-lang.org/book/ch01-01-installation.html)에 설치 방법이 나와 있습니다.

마지막으로 시스템이 준비되었는지 확인하세요. 구성 요소를 설치하면서 시스템 상태를 확인하려면 이 명령들을 실행하면 됩니다. 터미널 출력에 아무런 문제가 포함되어 있지 않다면 다음 단계로 넘어가셔도 좋습니다.

```bash
rustc --version
flutter doctor
```

## 빌드 도구 버전 문제

- Android 앱을 빌드하기 위해선 [이 이슈](https://github.com/rust-lang/rust/pull/85806)로 인해 Rust 1.68 이상을 사용해야 합니다.
- Android 앱을 빌드하기 위해선 `./android/app/build.gradle` 파일에 `ndkVersion` 변수가 있어야 하지만, Flutter SDK 3.7 이하 버전으로 Flutter 프로젝트를 생성한 경우 이 변수가 존재하지 않을 수 있습니다. 이 문제를 해결하려면 [이 토론](https://github.com/cunarist/rust-in-flutter/discussions/60)을 참고하세요.

> Rust로 다양한 빌드 타겟을 사용하면 때로 예상치 못한 문제가 나타날 수 있습니다. 문제가 발생하면 [토론 페이지](https://github.com/cunarist/rust-in-flutter/discussions)에서 Q&A 스레드를 열어주시기 바랍니다.

# 👜 템플릿 적용

다음 명령을 터미널에서 실행하세요.

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

Dart에서 요청을 보낼 때에는 작업 및 주소를 함께 지정해야 합니다. 상호작용 방식은 RESTful API의 정의에 부합하도록 설계되었습니다.

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

Rust에서 요청을 수신하면 먼저 주소별로 분류하세요.

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

Rust에서의 엔드포인트 함수는 다음과 같습니다. 메시지 Schema는 작업 유형에 따라 다르기 때문에 Match 구문 내에서 정의되도록 쓰여 있습니다.

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

이 RESTful API 패턴을 확장하여 필요에 따라 수백, 수천 개의 엔드포인트를 생성할 수 있습니다. 웹 개발을 한 경험이 있다면 익숙한 방식일 것입니다. Rust 템플릿 내부의 실제 코드에 더 많은 주석과 정보가 포함되어 있습니다.

이상적으로는 Flutter가 크로스 플랫폼 사용자 인터페이스를 처리하고 Rust가 비즈니스 로직을 담당할 것입니다. 프론트엔드와 백엔드는 완전히 구분되어 있으며 Dart와 Rust 코드는 서로 분리되는 것이 가능합니다. 통신은 채널과 스트림을 통해 이루어집니다.

Dart와 Rust 간에 전송되는 메시지의 직렬화에는 [MessagePack](https://msgpack.org/)을 사용하세요. 이는 Rust 템플릿에서 기본적으로 제공되며, 다른 이유가 없다면 MessagePack을 사용하는 것이 좋습니다. MessagePack은 JSON과 유사한 구조지만, Binary이며 JSON보다 더 빠르고 더 작은 크기를 가집니다.

Dart와 Rust 사이에서 전송되는 데이터는 기본적으로 바이트 배열입니다. 이는 Dart에서는 `Uint8List`로, Rust에서는 `Vec<u8>`로 표현됩니다. 추천되는 메시지 직렬화 방식은 MessagePack이지만, 고해상도 이미지 또는 파일 데이터 등 다른 종류의 바이트 데이터도 전송할 수 있습니다. 아무 정보도 포함시킬 필요가 없다면 빈 바이트 배열을 담으면 됩니다.

Rust-In-Flutter의 빌드 설정은 Rust crate들로부터 컴파일된 모든 라이브러리 파일들이 최종 빌드에 올바르게 포함되어 배포 준비가 되었음을 보장합니다. 따라서 라이브러리 파일을 번들링하는 것에 대해 걱정할 필요는 없습니다.

공지사항과 자세한 정보들은 [토론 페이지](https://github.com/cunarist/rust-in-flutter/discussions)에 올라옵니다. 추가적인 설명을 읽거나 질문을 하시려면 이곳을 방문해주세요.

# ☕ 개발 지원하기

😉 Rust-In-Flutter의 기능이 도움이 되었다면, 이 프로젝트를 지원해주세요. 여러분의 너그러운 후원은 Rust-In-Flutter의 유지보수에 큰 도움이 됩니다.

만약 지원해 주시기로 결정했다면, [Buy Me a Coffee 페이지](https://www.buymeacoffee.com/cunarist)로 찾아와 주세요.
