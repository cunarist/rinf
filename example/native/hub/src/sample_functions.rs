//! This module is only for demonstration purposes.
//! You might want to remove this module in production.

use crate::bridge::api::RustOperation;
use crate::bridge::api::RustRequest;
use crate::bridge::api::RustResponse;
use crate::bridge::api::RustSignal;
use crate::bridge::send_rust_signal;
use rmp_serde::from_slice;
use rmp_serde::to_vec_named;
use serde::Deserialize;
use serde::Serialize;

pub async fn calculate_something(rust_request: RustRequest) -> RustResponse {
    match rust_request.operation {
        RustOperation::Create => RustResponse::default(),
        RustOperation::Read => {
            // We declare MessagePack structs in this match condition
            // because schema will differ by the operation type.
            #[allow(dead_code)]
            #[derive(Deserialize)]
            struct RustRequestSchema {
                letter: String,
                before_number: i32,
                dummy_one: i32,
                dummy_two: i32,
                dummy_three: Vec<i32>,
            }
            let slice = rust_request.bytes.as_slice();
            let received: RustRequestSchema = from_slice(slice).unwrap();
            crate::print!("{:?}", received.letter);

            let before_value = received.before_number;
            let after_value = sample_crate::add_seven(before_value);

            #[derive(Serialize)]
            struct RustResponseSchema {
                after_number: i32,
                dummy_one: i32,
                dummy_two: i32,
                dummy_three: Vec<i32>,
            }
            RustResponse {
                successful: true,
                // Use `to_vec_named` from `rmp_serde`
                // to serialize the message.
                bytes: to_vec_named(&RustResponseSchema {
                    after_number: after_value,
                    dummy_one: 1,
                    dummy_two: 2,
                    dummy_three: vec![3, 4, 5],
                })
                .unwrap(),
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
            let rust_signal = RustSignal {
                address: String::from("sampleCategory.mandelbrot"),
                bytes: mandelbrot,
            };
            send_rust_signal(rust_signal);
        }
    }
}
