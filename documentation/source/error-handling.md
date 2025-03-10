# Error Handling

Effective error handling is crucial in applications to ensure predictable application behavior.

Rinf expects developers to use Flutter exclusively for the UI layer while keeping all business logic in Rust. This approach encourages handling errors and logging directly in Rust without crossing the language boundary.[^1]

[^1]: Rinf doesn't automatically handle Rust errors for you. By explicitly managing these errors, you can make your app clearer and more robust. Rinf was designed to be a reliable framework without excessive abstractions or implicit behaviors.

There are recommended practices for managing errors in real-world applications.

## No Panicking

We recommend that you _not_ write panicking code at all, since Rust has the idiomatic `Result<T, E>`. Additionally, Rust _cannot_ catch panics on the web platform (`wasm32-unknown-unknown`), which can cause callers to wait forever.

```{code-block} rust
:caption: Rust
fn not_good() {
    let option = get_option();
    let value_a = option.unwrap(); // This code can panic
    let result = get_result();
    let value_b = result.expect("This code can panic");
}

fn good() -> Result<(), SomeError> {
    let option = get_option();
    let value_a = option.ok_or(SomeError)?;
    let result = get_result();
    let value_b = result?;
    Ok(())
}
```

As the Rust documentation states, [most errors aren't serious enough](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html) to require the program or task to stop entirely.

## Flexible Error Type

To manage Rust errors effectively, using a flexible error type is beneficial.

Developing an app differs from creating a library, as an app may encounter a wide range of error situations. Declaring a distinct error type for each potential failure can be overwhelming, unless the error cases are simple enough.

Therefore, it is advisable to utilize a single, flexible error type. You can define your own or simply use one from `crates.io`:

- [anyhow](https://crates.io/crates/anyhow)

```{code-block} rust
use anyhow::Result;

fn get_cluster_info() -> Result<ClusterMap> {
    // `anyhow::Error` can be created from any error type.
    // By using the `?` operator, the conversion happens automatically.
    let config = std::fs::read_to_string("cluster.json")?;
    let map: ClusterMap = serde_json::from_str(&config)?;
    Ok(map)
}
```

## Reporting to Dart

You might need to notify the user through the UI that an error occurred in Rust, providing hints on how to mitigate the problem, such as checking the network connection, trying again later, or verifying the username and password. The code below can serve as a good starting point for reporting Rust errors to Dart.

```{code-block} proto
:caption: Protobuf
// [RUST-SIGNAL]
// Your implementation of an error.
// It might include a message, an error code, etc.
message ErrorReport  {
    string message = 1; // Human-readable explanation of the error
    int32 error_code = 2; // An error code to distinguish problems
}
```

```{code-block} rust
:caption: Rust
use anyhow::Result;
use crate::messages::*;

// Define a trait for reporting errors.
pub trait Report {
    fn report(self);
}

// Implement the Report trait for `Result<()>`.
impl Report for Result<()>
{
    // Report the error to Dart.
    // A Flutter widget might draw the error info on the screen.
    fn report(self) {
        if let Err(err) = self {
            ErrorReport {
                message: format!("Rust error: {:?}", err),
                error_code: 25,
            }
            .send_signal_to_dart()
        }
    }
}

async fn make_http_request() -> Result<MyResponse> {
    // Implementation for making an HTTP request.
    // It can be any kind of failable function.
}

async fn top_level() -> Result<()> {
    let my_response = make_http_request().await?;
    // Do something with `my_response`.
    // Additional processing may be written here.
    Ok(())
}

async fn main_task() {
    let result = top_level().await;
    result.report();
}
```

This is how to use a top-level function to report the propagated error. You will almost always use the `.report()` method because Rust automatically warns you about unused `Result`s.

## Logging

You may want to log errors to the console or a file. Several crates can help with this process:

- [tracing](https://crates.io/crates/tracing)
- [async-log](https://crates.io/crates/async-log)
