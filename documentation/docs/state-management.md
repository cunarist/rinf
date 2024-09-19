# State Management

This section provides a general guide on effectively managing application state while using Rinf. It is not an introduction to a specific feature of Rinf.

Rinf performs best when the application logic is written entirely in Rust, with Flutter used solely for the GUI. In such cases, you might want to store the application state in Rust.

## 💥 Actor Model

The actor model is highly recommended for managing asynchronous state in Rust. By encapsulating state and behavior within actor structs, which maintain ownership and handle their own async tasks, the actor model provides a scalable and reliable way to manage complex state interactions.

1. **Encapsulation**: Actors encapsulate state and behavior, allowing for modular and maintainable code.
2. **Concurrency**: Each actor operates independently, making it easier to handle concurrent tasks without manual synchronization.
3. **Scalability**: Actors are well-suited for scalable systems where tasks and state management need to be handled in parallel.

Several crates on `crates.io` provide building blocks for implementing the actor model in Rust. Although Rinf uses `tokio` by default, you can choose any async Rust runtime that fits your needs. Consider exploring these crates to find one that aligns with your requirements.

Here’s a basic example using the [`actix`](https://github.com/actix/actix) crate, a popular choice for the actor model:

```rust title="native/hub/src/lib.rs"
use actix::prelude::*;

rinf::write_interface!()

// this is our Message
// we have to define the response type (rtype)
#[derive(Message)]
#[rtype(usize)]
struct Sum(usize, usize);

// Actor definition
struct Calculator;

impl Actor for Calculator {
    type Context = Context<Self>;
}

// now we need to implement `Handler` on `Calculator` for the `Sum` message.
impl Handler<Sum> for Calculator {
    type Result = usize; // <- Message response type

    fn handle(&mut self, msg: Sum, _ctx: &mut Context<Self>) -> Self::Result {
        msg.0 + msg.1
    }
}

#[actix::main] // <- starts the system and block until future resolves
async fn main() {
    let addr = Calculator.start();
    let res = addr.send(Sum(10, 5)).await; // <- send message and get future for result

    match res {
        Ok(result) => println!("SUM: {}", result),
        _ => println!("Communication to the actor has failed"),
    }
}
```

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
- [`tokio::sync::Mutex`](https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html)
- [`tokio::sync::RwLock`](https://docs.rs/tokio/latest/tokio/sync/struct.RwLock.html)
