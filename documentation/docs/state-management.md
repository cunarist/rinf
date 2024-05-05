# State Management

This section is not an introduction to a specific Rinf feature, but rather a general guide about how you can smoothly manage the application state while using Rinf.

Rinf works best when the application logic is entirely written in Rust, while Flutter is only being used for GUI. Given that, there are situations where you want to store the application state in Rust.

You can set a simple mutable global state variable like this that only involves constant `new` function call. Note that `tokio::sync::RwLock` can be a better option when there are multiple concurrent readers.

```rust title="Rust"
use rinf::debug_print;
use tokio::sync::Mutex;

// `Mutex` prevents race conditions between threads.
static VECTOR: Mutex<Vec<bool>> = Mutex::const_new(Vec::new());

pub async fn do_something_with_state() {
    // Get the mutex guard.
    let mut vector = VECTOR.lock().await;

    // Custom logic here.
    vector.push(true);
    let length = vector.len();
    debug_print!("{length}");
}
```

After that, the latest state can be streamed to a Flutter widget to be shown on the screen.

```rust title="Rust"
pub async fn do_something_with_state() {
    ...
    MyMessage {
       some_latest_state: vector.len() as i32,
    }
    .send_signal_to_dart();
}
```

```dart title="Dart"
StreamBuilder(
  stream: MyMessage.rustSignalStream,
  builder: (context, snapshot) {
    final rustSignal = snapshot.data;
    if (rustSignal == null) {
      return SomeWidget(null);
    }
    final myMessage = rustSignal.message;
    final someLatestState = myMessage.someLatestState;
    return SomeWidget(someLatestState);
  },
),
```

If initialization logic is required to fill in the global state, you can use the singleton getter pattern just like below. This might be useful when the app involves some IO operations, which means that the initial resource size is not known at compile time.

```rust title="Rust"
use rinf::debug_print;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use tokio::sync::OnceCell;

// `OnceCell` can be assigned a value only once.
static DB_POOL: OnceCell<Pool<Sqlite>> = OnceCell::const_new();

async fn get_db_pool<'a>() -> &'a Pool<Sqlite> {
    DB_POOL.get_or_init(|| async {
        SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap()
    }).await
}

pub async fn do_something_with_state() {
    // Use the getter function.
    let db_pool = get_db_pool().await;

    // Custom logic here.
    let fetched = db_pool.
        fetch_one("SELECT * FROM sample_table").
        await.
        unwrap();
    debug_print!("{fetched:?}");
}
```
