#[cfg(not(target_family = "wasm"))]
pub use std::thread::spawn;

#[cfg(target_family = "wasm")]
mod web {
    use crate::bridge::bridge_engine::{script_path, wasm_bindgen_src::pool::WorkerPool};

    thread_local! {
        pub static WORKER_POOL: Option<WorkerPool> = WorkerPool::new(
            1, script_path().unwrap())
                .map_err(|err| crate::console_error!("Failed to spawn worker: {:?}", err)).ok()
    }
}

#[cfg(target_family = "wasm")]
pub use web::WORKER_POOL;
