//! This module is only for demonstration purposes.
//! You might want to remove this module in production.

use crate::bridge::send_rust_signal;
use crate::bridge::{RustOperation, RustRequest, RustResponse, RustSignal};
use prost::Message;

const SHOULD_DEMONSTRATE: bool = true; // Disabled when applied as template

pub async fn handle_sample_resource(rust_request: RustRequest) -> RustResponse {
    match rust_request.operation {
        RustOperation::Create => RustResponse::default(),
        RustOperation::Read => RustResponse::default(),
        RustOperation::Update => RustResponse::default(),
        RustOperation::Delete => RustResponse::default(),
    }
}

pub async fn handle_deeper_resource(rust_request: RustRequest) -> RustResponse {
    match rust_request.operation {
        RustOperation::Create => RustResponse::default(),
        RustOperation::Read => RustResponse::default(),
        RustOperation::Update => RustResponse::default(),
        RustOperation::Delete => RustResponse::default(),
    }
}

pub async fn handle_counter_number(rust_request: RustRequest) -> RustResponse {
    use crate::messages::counter_number::{ReadRequest, ReadResponse};
    // We import message structs in this handler function
    // because schema will differ by Rust resource.

    match rust_request.operation {
        RustOperation::Create => RustResponse::default(),
        RustOperation::Read => {
            // Decode raw bytes into a Rust message object.
            let message_bytes = rust_request.message.unwrap();
            let request_message = ReadRequest::decode(message_bytes.as_slice()).unwrap();
            let letter = request_message.letter;
            crate::debug_print!("{letter}");

            // Perform a simple calculation.
            let after_value: i32 = sample_crate::add_seven(request_message.before_number);

            // Return the response that will be sent to Dart.
            let response_message = ReadResponse {
                after_number: after_value,
                dummy_one: request_message.dummy_one,
                dummy_two: request_message.dummy_two,
                dummy_three: request_message.dummy_three,
            };
            RustResponse {
                successful: true,
                message: Some(response_message.encode_to_vec()),
                blob: None,
            }
        }
        RustOperation::Update => RustResponse::default(),
        RustOperation::Delete => RustResponse::default(),
    }
}

pub async fn stream_mandelbrot() {
    use crate::messages::mandelbrot::{StateSignal, ID};

    if !SHOULD_DEMONSTRATE {
        return;
    }

    let mut scale: f64 = 1.0;

    let (frame_sender, mut frame_receiver) = tokio::sync::mpsc::channel(5);

    // Send frame join handles in order.
    crate::spawn(async move {
        loop {
            // Wait for 40 milliseconds on each frame
            crate::sleep(std::time::Duration::from_millis(40)).await;
            if frame_sender.capacity() == 0 {
                continue;
            }

            scale *= 0.98;
            if scale < 1e-7 {
                scale = 1.0
            };

            // Calculate the mandelbrot image
            // parallelly in a separate thread pool.
            let join_handle = crate::spawn_blocking(move || {
                sample_crate::mandelbrot(
                    sample_crate::Size {
                        width: 384,
                        height: 384,
                    },
                    sample_crate::Point {
                        x: 0.360,
                        y: -0.641,
                    },
                    scale,
                )
            });
            let _ = frame_sender.send(join_handle).await;
        }
    });

    // Receive frame join handles in order.
    crate::spawn(async move {
        loop {
            let join_handle = frame_receiver.recv().await.unwrap();
            let received_frame = join_handle.await.unwrap();
            if let Some(mandelbrot) = received_frame {
                // Stream the signal to Dart.
                let signal_message = StateSignal {
                    id: 0,
                    current_scale: scale,
                };
                let rust_signal = RustSignal {
                    resource: ID,
                    message: Some(signal_message.encode_to_vec()),
                    blob: Some(mandelbrot),
                };
                send_rust_signal(rust_signal);
            };
        }
    });
}

pub async fn run_debug_tests() {
    #[cfg(debug_assertions)]
    const IS_DEBUG_MODE: bool = true;
    #[cfg(not(debug_assertions))]
    const IS_DEBUG_MODE: bool = false;

    if !SHOULD_DEMONSTRATE || !IS_DEBUG_MODE {
        return;
    }

    crate::sleep(std::time::Duration::from_secs(1)).await;
    crate::debug_print!("Starting debug tests.");

    // Get the current time.
    let current_time = sample_crate::get_current_time();
    crate::debug_print!("System time: {current_time}");

    // Fetch data from a web API.
    let url = "http://jsonplaceholder.typicode.com/todos/1";
    let web_response = sample_crate::fetch_from_web_api(url).await;
    crate::debug_print!("Response from a web API: {web_response}");

    // Use a crate that accesses operating system APIs.
    let option = sample_crate::get_hardward_id();
    if let Some(hwid) = option {
        crate::debug_print!("Hardware ID: {hwid}");
    } else {
        crate::debug_print!("Hardware ID is not available on this platform.");
    }

    // Test `tokio::join!` for futures.
    let join_first = async {
        crate::sleep(std::time::Duration::from_secs(1)).await;
        crate::debug_print!("First future finished.");
    };
    let join_second = async {
        crate::sleep(std::time::Duration::from_secs(2)).await;
        crate::debug_print!("Second future finished.");
    };
    let join_third = async {
        crate::sleep(std::time::Duration::from_secs(3)).await;
        crate::debug_print!("Third future finished.");
    };
    tokio::join!(join_first, join_second, join_third);

    // Avoid blocking the async event loop by yielding.
    let mut last_time = sample_crate::get_current_time();
    let mut count = 0u64;
    let mut steps_finished = 0;
    loop {
        count += 1;
        if count % 10000 == 0 {
            crate::yield_now().await;
            let time_passed = sample_crate::get_current_time() - last_time;
            if time_passed.num_milliseconds() > 1000 {
                crate::debug_print!("Counted to {count}, yielding regularly.");
                last_time = sample_crate::get_current_time();
                steps_finished += 1;
                if steps_finished == 10 {
                    break;
                }
            }
        }
    }

    // Test `spawn_blocking` with multicore parallelization.
    let mut join_handles = Vec::new();
    let chunk_size = 10_i32.pow(6);
    for level in 0..10 {
        let join_handle = crate::spawn_blocking(move || {
            let mut prime_count = 0;
            let count_from = level * chunk_size + 1;
            let count_to = (level + 1) * chunk_size;
            for number in count_from..=count_to {
                let mut is_prime = true;
                let square_root = (number as f64).sqrt() as i32;
                if number <= 1 {
                    is_prime = false;
                } else {
                    let mut i = 2;
                    while i <= square_root {
                        if number % i == 0 {
                            is_prime = false;
                            break;
                        }
                        i += 1;
                    }
                }
                if is_prime {
                    prime_count += 1;
                }
            }
            format!("There are {prime_count} primes from {count_from} to {count_to}.")
        });
        join_handles.push(join_handle);
    }
    for join_handle in join_handles {
        let text = join_handle.await.unwrap();
        crate::debug_print!("{text}");
    }

    crate::debug_print!("Debug tests completed!");
    panic!("INTENTIONAL DEBUG PANIC");
}
