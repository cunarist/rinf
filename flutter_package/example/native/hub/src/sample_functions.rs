//! This crate is written for Rinf demonstrations.

use crate::common::*;
use crate::messages;
use crate::tokio;
use rinf::debug_print;
use std::time::Duration;

// Using the `cfg` macro enables conditional statement.
#[cfg(debug_assertions)]
static IS_DEBUG_MODE: bool = true;
#[cfg(not(debug_assertions))]
static IS_DEBUG_MODE: bool = false;

// Business logic for the counter widget.
pub async fn tell_numbers() -> Result<()> {
    use messages::counter_number::*;

    let mut vector = Vec::new();

    // Stream getter is generated from a marked Protobuf message.
    let receiver = SampleNumberInput::get_dart_signal_receiver()?;
    while let Some(dart_signal) = receiver.recv().await {
        // Extract values from the message received from Dart.
        // This message is a type that's declared in its Protobuf file.
        let number_input = dart_signal.message;
        let letter = number_input.letter;
        debug_print!("{letter}");

        // Perform a simple calculation.
        vector.push(true);
        let current_number = (vector.len() as i32) * 7;

        // The send method is generated from a marked Protobuf message.
        SampleNumberOutput {
            current_number,
            dummy_one: number_input.dummy_one,
            dummy_two: number_input.dummy_two,
            dummy_three: number_input.dummy_three,
        }
        .send_signal_to_dart();
    }

    Ok(())
}

// Business logic for the fractal image.
pub async fn stream_fractal() {
    use messages::counter_number::*;
    use messages::fractal_art::*;

    let mut current_scale: f64 = 1.0;

    let (sender, mut receiver) = tokio::sync::mpsc::channel(5);

    // Send frame join handles in order.
    tokio::spawn(async move {
        loop {
            // Wait for 40 milliseconds on each frame
            tokio::time::sleep(Duration::from_millis(40)).await;
            if sender.capacity() == 0 {
                continue;
            }

            current_scale *= 1.02;
            if current_scale > 1e+7 {
                current_scale = 1.0
            };

            // Calculate the fractal image
            // parallelly in a separate thread pool.
            let join_handle = tokio::task::spawn_blocking(move || {
                sample_crate::draw_fractal_image(current_scale)
            });
            let _ = sender.send(join_handle).await;
        }
    });

    // Receive frame join handles in order.
    tokio::spawn(async move {
        loop {
            let join_handle = match receiver.recv().await {
                Some(inner) => inner,
                None => continue,
            };
            let received_frame = match join_handle.await {
                Ok(inner) => inner,
                Err(_) => continue,
            };
            if let Ok(fractal_image) = received_frame {
                // Stream the image data to Dart.
                SampleFractal {
                    current_scale,
                    dummy: Some(SampleSchema {
                        sample_field_one: true,
                        sample_field_two: false,
                    }),
                }
                .send_signal_to_dart(fractal_image);
            };
        }
    });
}

// A dummy function that uses sample messages to eliminate warnings.
#[allow(dead_code)]
async fn use_messages() -> Result<()> {
    use messages::sample_folder::sample_file::*;
    let _ = SampleInput::get_dart_signal_receiver()?;
    SampleOutput {
        kind: 3,
        oneof_input: Some(sample_output::OneofInput::Age(25)),
    }
    .send_signal_to_dart();
    Ok(())
}

// Business logic for testing various crates.
pub async fn run_debug_tests() -> Result<()> {
    if !IS_DEBUG_MODE {
        return Ok(());
    }

    tokio::time::sleep(Duration::from_secs(1)).await;
    debug_print!("Starting debug tests.");

    // Get the current time.
    let current_time = sample_crate::get_current_time();
    debug_print!("System time: {current_time}");

    // Fetch data from a web API.
    let url = "http://jsonplaceholder.typicode.com/todos/1";
    let web_response = sample_crate::fetch_from_web_api(url).await?;
    debug_print!("Response from a web API: {web_response:?}");

    // Use a crate that accesses operating system APIs.
    let hwid = sample_crate::get_hardward_id()?;
    debug_print!("Hardware ID: {hwid:?}");

    // Test `tokio::join!` for futures.
    let join_first = async {
        tokio::time::sleep(Duration::from_secs(1)).await;
        debug_print!("First future finished.");
    };
    let join_second = async {
        tokio::time::sleep(Duration::from_secs(2)).await;
        debug_print!("Second future finished.");
    };
    let join_third = async {
        tokio::time::sleep(Duration::from_secs(3)).await;
        debug_print!("Third future finished.");
    };
    join_first.await;
    join_second.await;
    join_third.await;

    // Avoid blocking the async event loop by yielding.
    let mut last_time = sample_crate::get_current_time();
    let mut count = 0u64;
    let mut steps_finished = 0;
    loop {
        for _ in 0..1000000 {
            count += 1;
        }
        tokio::task::yield_now().await;
        let time_passed = sample_crate::get_current_time() - last_time;
        if time_passed.num_milliseconds() > 1000 {
            debug_print!("Counted to {count}, yielding regularly.");
            last_time = sample_crate::get_current_time();
            steps_finished += 1;
            if steps_finished == 10 {
                break;
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
        if let Ok(text) = join_handle.await {
            debug_print!("{text}");
        }
    }

    debug_print!("Debug tests completed!");

    tokio::spawn(async {
        // Panic in a separate task
        // to avoid memory leak on the web.
        // On the web (`wasm32-unknown-unknown`),
        // catching and unwinding panics is not possible.
        // It is better to avoid panicking code at all costs on the web.
        panic!("INTENTIONAL DEBUG PANIC");
    });

    Ok(())
}
