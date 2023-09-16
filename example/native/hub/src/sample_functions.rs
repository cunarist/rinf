//! This module is only for demonstration purposes.
//! You might want to remove this module in production.

use crate::bridge::api::{RustOperation, RustRequest, RustResponse, RustSignal};
use crate::bridge::send_rust_signal;
use prost::Message;

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
    use crate::messages::mandelbrot::{Signal, ID};

    let mut scale: f64 = 1.0;

    loop {
        crate::sleep(std::time::Duration::from_millis(15)).await;

        scale *= 0.98;
        if scale < 1e-7 {
            scale = 1.0
        };

        let calculated = sample_crate::mandelbrot(
            sample_crate::Size {
                width: 128,
                height: 128,
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
            let signal_message = Signal {
                id: 0,
                current_scale: scale,
            };
            let rust_signal = RustSignal {
                resource: ID,
                message: Some(signal_message.encode_to_vec()),
                blob: Some(mandelbrot),
            };
            send_rust_signal(rust_signal);
        }
    }
}
