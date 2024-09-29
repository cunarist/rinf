# State Management

This section offers a general guide to managing application state effectively with Rinf, rather than introducing a specific Rinf feature.

Rinf performs best when the application logic is written entirely in Rust, with Flutter used solely for the GUI. Given that, you might want to store the application state in Rust.

## 💥 Actor Model

The actor model is highly recommended for managing asynchronous state in Rust. By encapsulating state and behavior within actor structs, which maintain ownership and handle their own async tasks, the actor model provides a scalable and modular way to manage complex state interactions.

Here’s a basic example using the [`messages`](https://crates.io/crates/messages) crate, which is a flexible and runtime-agnostic actor library that works nicely with Rinf.

```rust title="native/hub/src/lib.rs"
use messages::prelude::*;

rinf::write_interface!();

// Represents a message to calculate the sum of two numbers.
struct Sum(usize, usize);

// Actor definition.
struct Calculator;

// Implement Actor trait for Calculator.
impl Actor for Calculator {}

// Implement Handler for Calculator to handle Sum messages.
#[async_trait]
impl Handler<Sum> for Calculator {
    type Result = usize;

    async fn handle(&mut self, msg: Sum, _: &Context<Self>) -> Self::Result {
        msg.0 + msg.1
    }
}

// Implement the start method for Calculator
impl Calculator {
    pub fn start() -> Address<Self> {
        let context = Context::new();
        let actor = Self {};
        let addr = context.address();
        tokio::spawn(context.run(actor));
        addr
    }
}

// Main function to start the business logic
#[tokio::main]
async fn main() {
    let mut addr = Calculator::start();
    let res = addr.send(Sum(10, 5)).await;

    match res {
        Ok(result) => println!("SUM: {}", result),
        _ => println!("Communication to the actor has failed"),
    }
}
```

Several crates on `crates.io` provide building blocks for implementing the actor model in Rust. Consider exploring these crates to find one that aligns with your requirements.

## 🧱 Static Variables

Generally, it's advisable to avoid static variables due to their characteristics, which can lead to issues such as difficulties in testing and managing lifetimes. If you must use static variables, you can declare them as shown below, ensuring they span the entire duration of the app.

```rust title="Rust"
use rinf::debug_print;
use tokio::sync::Mutex;

static VECTOR: Mutex<Vec<bool>> = Mutex::const_new(Vec::new());

pub async fn do_something_with_state() {
    VECTOR.lock().await.push(true);

    // Use the global variable by acquiring the guard.
    let guard = VECTOR.lock().await;
    let length = guard.len();
    debug_print!("{length}");
}
```

Only use static variables in certain situations as described in the [Rust docs](https://doc.rust-lang.org/reference/items/static-items.html):

- Large amounts of data are being stored.
- The single-address property of statics is required.
- Interior mutability is required.

It's important to remember that destructors of static variables implemented by the [`Drop`](https://doc.rust-lang.org/rust-by-example/trait/drop.html) trait don't get called on app shutdown. Therefore, if you need destructors of static variables to be run, you must drop or close them explicitly before exiting.

There are also alternatives. Choose the one that you think is most appropriate for your needs:

- [`std::sync::LazyLock`](https://doc.rust-lang.org/std/sync/struct.LazyLock.html)
- [`tokio::sync::RwLock`](https://docs.rs/tokio/latest/tokio/sync/struct.RwLock.html)
