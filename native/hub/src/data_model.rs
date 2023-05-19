//! This module is the model part of MVVM pattern.
//! To be more clear, this module was named `data_model` instead of just `model`.

use ref_thread_local::ref_thread_local;
use std::collections::HashMap;
use tokio::sync::RwLock;

ref_thread_local! {
    // With this macro, it is ensured
    // that each global static variable is only valid on the main thread.
    // When it is accessed from some other thread,
    // The collection will appear to be something completely different.
    pub static managed SAMPLE_NUMBERS: HashMap<String, RwLock<i32>> = HashMap::new();
}
