# Error Handling

Effective error handling is crucial in applications to ensure predictable application behavior.

Rinf expects developers to use Flutter exclusively for the UI layer while keeping all business logic in Rust. This approach encourages handling errors and logging directly in Rust without crossing the language boundary.[^1]

[^1]: Rinf doesn't automatically handle Rust errors for you. By explicitly managing these errors, you can make your app clearer and more robust. Rinf is designed to be a reliable framework without excessive abstractions or implicit behaviors.

Below are recommended practices for managing errors in real-world applications.

## No Panicking

We recommend that you _not_ write panicking code at all, as Rust provides the idiomatic `Result<T, E>`. Additionally, Rust _cannot_ catch panics on the web platform (`wasm32-unknown-unknown`), which can cause callers to wait forever.

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

Developing an app differs from creating a library, as an app may encounter a wide range of error situations. Declaring a distinct error enum variant for each potential failure can be overwhelming unless the error cases are simple enough.

Therefore, it is advisable to utilize a single, flexible error type. You can define your own or use one from `crates.io`:

- [anyhow](https://crates.io/crates/anyhow)

```{code-block} rust
:caption: Rust
use anyhow::{Context, Result};

fn get_cluster_info() -> Result<ClusterMap> {
  // `anyhow::Error` can be created from any error type.
  // By using the `?` operator, the conversion happens automatically.
  let config = std::fs::read_to_string("cluster.json")?;
  // By using the `context` method, you can wrap the original error
  // with additional information.
  let map: ClusterMap = serde_json::from_str(&config)
    .context("Failed to parse cluster configuration as JSON")?;
  Ok(map)
}
```

## Logging

You may want to log errors to the console or a file. Several crates can help with this process:

- [tracing](https://crates.io/crates/tracing)
- [async-log](https://crates.io/crates/async-log)

Using a centralized trait for logging errors can be helpful. By calling a common method for logging, you can handle the propagated error consistently. Rust automatically warns you about unused `Result`s, making it easier to handle all errors in your code.

The trait below demonstrates how to consume only the error variant for logging. It works similarly to the `Result::ok` method, but with extra logging functionality.

```{code-block} rust
:caption: Rust
use anyhow::Result;
use tracing::error;

pub trait ReportError<T> {
  fn report(self) -> Option<T>;
}

impl<T> ReportError<T> for Result<T> {
  fn report(self) -> Option<T> {
    match self {
      Ok(inner) => Some(inner),
      Err(err) => {
        error!("{:?}", err);
        None
      }
    }
  }
}
```

```{code-block} rust
:caption: Rust
fn example_function() {
  // Report from a top-level function.
  let empty_result: Result<()> = returns_empty_result();
  empty_result.report();

  // Report before consuming the result value.
  for _ in 0..5 {
    let number_result: Result<i32> = returns_number_result();
    let number = match number_result.report() {
      Some(inner) => inner,
      None => continue,
    };
    use_number(number);
  }
}
```
