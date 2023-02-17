use once_cell::sync::Lazy;
use std::sync::RwLock;

pub static COUNT: Lazy<RwLock<i32>> = Lazy::new(|| {
    let value = 0;
    RwLock::new(value)
});

// Add more using `Lazy<RwLock<T>>`
