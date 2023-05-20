//! This module is the model part of MVVM pattern.
//! To be more clear, this module was named `data_model` instead of just `model`.

use ref_thread_local::{ref_thread_local, RefThreadLocal};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// This function is meant to be called when Dart's hot restart is triggered in debug mode.
pub fn clean_model() {
    let mut borrowed = SAMPLE_NUMBERS.borrow_mut();
    *borrowed = HashMap::new();
}

ref_thread_local! {
    // With this macro, it is ensured
    // that each global static variable is only valid on the main thread.
    // When it is accessed from some other thread,
    // The collection will appear to be something completely different.
    pub static managed SAMPLE_NUMBERS: HashMap<String, RwLock<i32>> = HashMap::new();
}
