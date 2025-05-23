# State Management

This section offers a general guide to managing application state effectively with Rinf, rather than introducing a specific Rinf feature.

Rinf performs best when the application logic is written entirely in Rust, with Flutter used solely for the GUI. Given that, you might want to store the application state in Rust.

## Actor Model

The actor model is highly recommended for managing asynchronous state in Rust. By encapsulating state and behavior within actor structs, which maintain ownership and handle their own async tasks, the actor model provides a scalable and modular way to manage complex state interactions.

The default Rust template provided by Rinf provides a good starting point. Below is a short example of how actors work:

```{code-block} rust
:caption: native/hub/src/lib.rs
use async_trait::async_trait;
use messages::prelude::{Actor, Address, Context, Notifiable};
use rinf::{dart_shutdown, debug_print, write_interface};
use tokio::spawn;

write_interface!();

/// Represents a message to calculate the sum of two numbers.
struct Sum(usize, usize);

/// Actor definition that will hold state in real apps.
struct MyActor {
  count: i32,
}

impl Actor for MyActor {}

impl MyActor {
  pub fn new() -> Self {
    Self { count: 0 }
  }
}

#[async_trait]
impl Notifiable<Sum> for MyActor {
  async fn notify(&mut self, msg: Sum, _: &Context<Self>) {
    self.count += 1;
    debug_print!("{}: {}", msg.0 + msg.1, self.count);
  }
}

fn create_actors() -> Address<MyActor> {
  let context = Context::new();
  let addr = context.address();
  let actor = MyActor::new();
  spawn(context.run(actor));
  addr
}

/// Main function to start the business logic.
#[tokio::main]
async fn main() {
  let mut addr = create_actors();
  let _ = addr.notify(Sum(10, 5)).await;
  dart_shutdown().await;
}
```

Please refer to the [example app](https://github.com/cunarist/rinf/tree/main/flutter_package/example) for detailed usage.

## Static Variables

Generally, it's advisable to avoid static variables due to their characteristics, which can lead to issues such as difficulties in testing and managing lifetimes. If you must use static variables, you can declare them as shown below, ensuring they span the entire duration of the app.

```{code-block} rust
:caption: Rust
use rinf::debug_print;
use tokio::sync::Mutex;

static VECTOR: Mutex<Vec<bool>> = Mutex::const_new(Vec::new());

pub async fn do_something_with_state() {
  // Use the global variable by locking it temporarily.
  VECTOR.lock().await.push(true);

  // Use the global variable by acquiring the guard.
  let guard = VECTOR.lock().await;
  let length = guard.len();
  debug_print!("{}", length);
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
