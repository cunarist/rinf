use lazy_static::lazy_static;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::task;

struct TaskDetail {
    order: String,
    json: String,
}

lazy_static! {
    static ref SENDER_HOLDER: Arc<Mutex<Option<Sender<TaskDetail>>>> = Arc::new(Mutex::new(None));
}

pub fn send_task(order: String, json: String) {
    // Dart's front-end main thread

    let guard = SENDER_HOLDER.try_lock().unwrap();
    let sender = guard.clone().unwrap();
    let _ = sender.send(TaskDetail { order, json });
}

#[tokio::main]
pub async fn main() {
    // Rust's back-end sub thread

    let (sender, receiver) = channel();
    *SENDER_HOLDER.lock().unwrap() = Some(sender);

    while let Ok(task_detail) = receiver.recv() {
        task::spawn(handle_task(task_detail));
    }
}

async fn handle_task(task_detail: TaskDetail) {
    // Tokio's basic threadpool

    let order = task_detail.order;
    let json = task_detail.json;
    let layered_order = order.split('.').collect::<Vec<&str>>();

    if layered_order.is_empty() {
    } else if layered_order[0] == "someCategory" {
        if layered_order.len() == 1 {
        } else if layered_order[1] == "addOne" {
            task::spawn_blocking(move || sample_crate_first::add_one(json));
        } else if layered_order[1] == "multiplyTwo" {
            task::spawn_blocking(move || sample_crate_second::multiply_two(json));
        } else {
        }
    } else {
    }
}
