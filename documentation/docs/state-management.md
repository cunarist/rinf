# State Management

This section is not an introduction to a specific Rinf's feature, but rather a general guide about how you can smoothly manage the application state while using Rinf.

Rinf works best when the application logic is entirely written in Rust, while Flutter is only being used for GUI. Given that, there are situations where you want to store the application state in Rust.

It is generally recommended to avoid static variables because of their characteristics, which can lead to issues such as difficulties in testing and managing lifetimes.

Try passing the state of your app down the function tree like below whenever it's acceptable.

```rust title="Rust"
use std::sync::Arc;
use tokio::sync::Mutex;

async fn main() {
    // `Mutex` prevents race conditions between threads.
    let shared_data = Arc::new(Mutex::new(Vec::new()));
    do_something_with_state(shared_data.clone()).await;
    do_something_with_state(shared_data.clone()).await;
}

pub async fn do_something_with_state(data: Arc<Mutex<Vec<i32>>>) {
    // Mutate the shared variable directly.
    data.lock().await.push(3);
}
```

If you cannot pass down the state down the function tree for any reason, you can declare a static variable like below that spans the entire duration of the app.

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
