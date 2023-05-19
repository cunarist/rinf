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
    // Use JSON objects in Rust but use MessagePack to serialize them.
    // They are cross-compatible.
    let json_value = json!({
        "value": *value
    });
    let payload = Serialized {
        data: rmp_serde::encode::to_vec(&json_value).unwrap(),
        formula: String::from("messagePack"),
    };
    update_viewmodel("someItemCategory.count", payload);
}

#[allow(dead_code)]
pub async fn keep_drawing_mandelbrot() {
    let mut scale: f64 = 1.0;
    loop {
        scale *= 0.95;
        if scale < 1e-9 {
            scale = 1.0
        };
        let (sender, receiver) = tokio::sync::oneshot::channel();
        tokio::task::spawn_blocking(move || {
            let mandelbrot = sample_crate::mandelbrot(
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
            .unwrap();
            sender.send(mandelbrot).ok();
        })
        .await
        .ok();
        let received = receiver.await;
        if let Ok(mandelbrot) = received {
            let payload = Serialized {
                data: mandelbrot,
                formula: String::from("image"),
            };
            update_viewmodel("someItemCategory.mandelbrot", payload);
        }
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    }
}
