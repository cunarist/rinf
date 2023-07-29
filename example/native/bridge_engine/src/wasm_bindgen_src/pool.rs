//! Original: wasm_bindgen raytrace-parallel example
//!
//! File: https://github.com/rustwasm/wasm-bindgen/blob/main/examples/raytrace-parallel/src/pool.rs

use crate::TransferClosure;
use js_sys::Array;
use std::cell::RefCell;
use std::iter::FromIterator;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::BlobPropertyBag;
use web_sys::ErrorEvent;
use web_sys::{Blob, Url};
use web_sys::{Event, Worker};

#[wasm_bindgen]
pub struct WorkerPool {
    state: Rc<PoolState>,
    script_src: String,
}

struct PoolState {
    workers: RefCell<Vec<Worker>>,
    callback: Closure<dyn FnMut(Event)>,
}

#[wasm_bindgen]
impl WorkerPool {
    /// Creates a new `WorkerPool` which immediately creates `initial` workers.
    ///
    /// The pool created here can be used over a long period of time, and it
    /// will be initially primed with `initial` workers. Currently workers are
    /// never released or gc'd until the whole pool is destroyed.
    ///
    /// # Errors
    ///
    /// Returns any error that may happen while a JS web worker is created and a
    /// message is sent to it.
    #[wasm_bindgen(constructor)]
    pub fn new(initial: usize, script_src: String) -> Result<WorkerPool, JsValue> {
        let pool = WorkerPool {
            script_src,
            state: Rc::new(PoolState {
                workers: RefCell::new(Vec::with_capacity(initial)),
                callback: Closure::new(|event: Event| {
                    if let Some(event) = event.dyn_ref::<ErrorEvent>() {
                        crate::console_error!("Failed to initialize: {}", event.message());
                    }
                }),
            }),
        };
        for _ in 0..initial {
            let worker = pool.spawn()?;
            pool.state.push(worker);
        }

        Ok(pool)
    }

    /// Unconditionally spawns a new worker
    ///
    /// The worker isn't registered with this `WorkerPool` but is capable of
    /// executing work for this wasm module.
    ///
    /// # Errors
    ///
    /// Returns any error that may happen while a JS web worker is created and a
    /// message is sent to it.
    fn spawn(&self) -> Result<Worker, JsValue> {
        let src = &self.script_src;
        let script = format!(
            "importScripts('{}');
            onmessage = event => {{
                let init = wasm_bindgen(...event.data).catch(err => {{
                    setTimeout(() => {{ throw err }})
                    throw err
                }})
                onmessage = async event => {{
                    await init
                    const [payload, ...transfer] = event.data
                    try {{
                        wasm_bindgen.receive_transfer_closure(payload, transfer)
                    }} catch (err) {{
                        if (transfer[0] && typeof transfer[0].postMessage === 'function') {{
                            transfer[0].postMessage([1, 'ABORT', err.toString(), err.stack])
                        }}
                        setTimeout(() => {{ throw err }})
                        postMessage(null)
                        throw err
                    }}
                }}
            }}",
            src
        );
        let blob = Blob::new_with_blob_sequence_and_options(
            &Array::from_iter([JsValue::from(script)]).into(),
            BlobPropertyBag::new().type_("text/javascript"),
        )?;
        let url = Url::create_object_url_with_blob(&blob)?;
        let worker: Worker = Worker::new(&url)?;

        // With a worker spun up send it the module/memory so it can start
        // instantiating the wasm module. Later it might receive further
        // messages about code to run on the wasm module.
        let module = wasm_bindgen::module();
        let memory = wasm_bindgen::memory();
        worker.post_message(&Array::from_iter([module, memory]))?;

        Ok(worker)
    }

    /// Executes the work `f` in a web worker, spawning a web worker if
    /// necessary.
    ///
    /// This will acquire a web worker and then send the closure `f` to the
    /// worker to execute. The worker won't be usable for anything else while
    /// `f` is executing, and no callbacks are registered for when the worker
    /// finishes.
    ///
    /// ## Errors
    ///
    /// Returns any error that may happen while a JS web worker is created and a
    /// message is sent to it.
    fn execute(&self, closure: TransferClosure<JsValue>) {
        let borrowed = self.state.workers.borrow();
        let worker = borrowed.get(0).unwrap();
        closure.apply(worker).ok();
    }
}

impl WorkerPool {
    /// Executes `f` in a web worker.
    ///
    /// This pool manages a set of web workers to draw from, and `f` will be
    /// spawned quickly into one if the worker is idle. If no idle workers are
    /// available then a new web worker will be spawned.
    ///
    /// Once `f` returns the worker assigned to `f` is automatically reclaimed
    /// by this `WorkerPool`. This method provides no method of learning when
    /// `f` completes, and for that you'll need to use `run_notify`.
    ///
    /// ## Errors
    ///
    /// If an error happens while spawning a web worker or sending a message to
    /// a web worker, that error is returned.
    ///
    /// ## Transferrables
    /// Items put inside `transfer` will have their ownership transferred from
    /// the invoking JS scope to the target, rendering the value unusable in the original
    /// scope. (This is similar to a `FnOnce` closure in Rust terms, but does not statically
    /// move items out of scope.)
    ///
    /// Certain types in [js_sys] and [web_sys] are transferrable, for which [Send]
    /// can be unsafely implemented **only if** they are passed to the transferrables of
    /// a `post_message`. Examples are `Buffer`s, `MessagePort`s, etc...
    pub fn run(&self, closure: TransferClosure<JsValue>) -> Result<(), JsValue> {
        self.execute(closure);
        // self.reclaim_on_message(worker);
        Ok(())
    }
}

impl PoolState {
    fn push(&self, worker: Worker) {
        worker.set_onmessage(Some(self.callback.as_ref().unchecked_ref()));
        worker.set_onerror(Some(self.callback.as_ref().unchecked_ref()));
        let mut workers = self.workers.borrow_mut();
        for prev in workers.iter() {
            let prev: &JsValue = prev;
            let worker: &JsValue = &worker;
            assert!(prev != worker);
        }
        workers.push(worker);
    }
}

#[cfg(feature = "wasm-start")]
#[wasm_bindgen(start)]
pub fn run_hooks() {
    console_error_panic_hook::set_once();
}
