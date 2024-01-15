//! This module is only for demonstration purposes.
//! You might want to remove this module in production.

use crate::messages::counter_number;
use crate::messages::fractal;
use crate::tokio;

const SHOULD_DEMONSTRATE: bool = true; // Disabled when applied as template

pub async fn handle_number(received: counter_number::NumberInput) {
    // Decode raw bytes into a Rust message object.
    let letter = received.letter;
    crate::debug_print!("{letter}");

    // Perform a simple calculation.
    let after_value: i32 = sample_crate::add_seven(received.before_number);

    // Return the message that will be sent to Dart.
    let message = counter_number::NumberOutput {
        after_number: after_value,
        dummy_one: received.dummy_one,
        dummy_two: received.dummy_two,
        dummy_three: received.dummy_three,
    };
    counter_number::number_output_send(message, None);
}

pub async fn stream_fractal() {
    if !SHOULD_DEMONSTRATE {
        return;
    }

    let mut scale: f64 = 1.0;

    let (frame_sender, mut frame_receiver) = tokio::sync::mpsc::channel(5);

    // Send frame join handles in order.
    tokio::spawn(async move {
        loop {
            // Wait for 40 milliseconds on each frame
            tokio::time::sleep(std::time::Duration::from_millis(40)).await;
            if frame_sender.capacity() == 0 {
                continue;
            }

            scale *= 1.02;
            if scale > 1e+7 {
                scale = 1.0
            };

            // Calculate the fractal image
            // parallelly in a separate thread pool.
            let join_handle = tokio::task::spawn_blocking(move || sample_crate::fractal(scale));
            let _ = frame_sender.send(join_handle).await;
        }
    });

    // Receive frame join handles in order.
    tokio::spawn(async move {
        loop {
            let join_handle = frame_receiver.recv().await.unwrap();
            let received_frame = join_handle.await.unwrap();
            if let Some(fractal) = received_frame {
                // Stream the image data to Dart.
                fractal::scale_state_send(
                    fractal::ScaleState {
                        current_scale: scale,
                    },
                    Some(fractal),
                );
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

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
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
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        crate::debug_print!("First future finished.");
    };
    let join_second = async {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        crate::debug_print!("Second future finished.");
    };
    let join_third = async {
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
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
            tokio::task::yield_now().await;
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
        let join_handle = tokio::task::spawn_blocking(move || {
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
