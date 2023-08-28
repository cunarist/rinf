//! This module is only for demonstration purposes.
//! You might want to remove this module in production.

use crate::bridge::api::RustOperation;
use crate::bridge::api::RustRequest;
use crate::bridge::api::RustResponse;
use crate::bridge::api::RustSignal;
use crate::bridge::send_rust_signal;
use prost::Message;

pub async fn calculate_something(rust_request: RustRequest) -> RustResponse {
    match rust_request.operation {
        RustOperation::Create => RustResponse::default(),
        RustOperation::Read => {
            // We import message structs in this match condition
            // because schema will differ by the operation type.
            use crate::messages::entry::{CounterGetRequest, CounterGetResponse};

            // Decode raw bytes into a Rust message object.
            let request_message = CounterGetRequest::decode(&rust_request.bytes[..]).unwrap();

            // Perform a simple calculation.
            let after_value: i32 = sample_crate::add_seven(request_message.before_number);

            // Return the response that will be sent to Dart.
            let response_message = CounterGetResponse {
                after_number: after_value,
                dummy_one: request_message.dummy_one,
                dummy_two: request_message.dummy_two,
                dummy_three: request_message.dummy_three,
            };
            RustResponse {
                successful: true,
                bytes: response_message.encode_to_vec(),
            }
        }
        RustOperation::Update => RustResponse::default(),
        RustOperation::Delete => RustResponse::default(),
    }
}

pub async fn keep_drawing_mandelbrot() {
    let mut scale: f64 = 1.0;
    let mut interval = crate::time::interval(std::time::Duration::from_millis(50));

    loop {
        interval.tick().await;

        scale *= 0.95;
        if scale < 1e-7 {
            scale = 1.0
        };

        let calculated = sample_crate::mandelbrot(
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
        );

        if let Ok(mandelbrot) = calculated {
            // Stream the signal to Dart.
            let rust_signal = RustSignal {
                address: String::from("sample-category/mandelbrot"),
                bytes: mandelbrot,
            };
            send_rust_signal(rust_signal);
        }
    }
}
