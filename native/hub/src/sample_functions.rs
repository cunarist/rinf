//! This module is only for demonstration purposes.
//! You might want to remove this module in production.

use crate::bridge::api::Serialized;
use crate::bridge::update_viewmodel;
use crate::data_model;
use ref_thread_local::RefThreadLocal;
use serde_json::json;
use tokio::sync::RwLock;

pub async fn calculate_something(serialized: Serialized) {
    let _ = serialized;
    let key = "someValueCategory.thisNumber";
    let mut hashmap = data_model::SAMPLE_NUMBERS.borrow_mut();
    if !hashmap.contains_key(key) {
        hashmap.insert(String::from(key), RwLock::new(0));
    }
    let option = hashmap.get(key);
    let rwlock = option.unwrap();
    let mut value = rwlock.write().await;
    *value = sample_crate::add_seven(*value);
    println!("{:}", *value);
    // Use JSON objects for packing or unpacking whenever possible.
    // Its highly readable macros and native data manipulation methods are
    // considerably better than others.
    // You can pack things like complex graph data, etc.
    let json_value = json!({
        "value": *value
    });
    // Although we use JSON objects for packing,
    // use MessagePack to serialize the packed data into bytes.
    // They are cross-compatible.
    // MessagePack provides 50~60% higher serialization performance
    // and much smaller output size than those of JSON.
    let payload = Serialized {
        bytes: rmp_serde::encode::to_vec(&json_value).unwrap(),
        formula: String::from("messagePack"),
    };
    // In Rust, you update the viewmodel with
    // `update_viewmodel` function imported from module `bridge`.
    // Because Dart widgets are bound to the viewmodel items,
    // updating them from Rust will automatically trigger
    // related Dart widgets to be rebuilt.
    update_viewmodel("someItemCategory.count", payload);
}

pub async fn keep_drawing_mandelbrot() {
    let mut scale: f64 = 1.0;
    loop {
        scale *= 0.95;
        if scale < 1e-9 {
            scale = 1.0
        };
        // Because drawing a mandelbrot image is
        // a CPU-intensive blocking task,
        // we use `spawn_blocking` instead of `spawn_local`
        // to delegate this task to another thread.
        // In real-world async scenarios,
        // thread blocking tasks that take more than 10 milliseconds
        // are considered better to be sent to a separate thread.
        let join_handle = tokio::task::spawn_blocking(move || {
            sample_crate::mandelbrot(
                sample_crate::Size {
                    width: 64,
                    height: 64,
                },
                sample_crate::Point {
                    x: 0.360,
                    y: -0.641,
                },
                scale,
                4,
            )
            .unwrap()
        });
        let calculated = join_handle.await;
        if let Ok(mandelbrot) = calculated {
            let payload = Serialized {
                bytes: mandelbrot,
                formula: String::from("image"),
            };
            update_viewmodel("someItemCategory.mandelbrot", payload);
        }
        // Never use `std::thread::sleep` on the main thread
        // because it will block the whole async runtime
        // managed by `tokio`.
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    }
}
