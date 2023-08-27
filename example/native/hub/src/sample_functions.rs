//! This module is only for demonstration purposes.
//! You might want to remove this module in production.

use crate::bridge::api::RustOperation;
use crate::bridge::api::RustRequest;
use crate::bridge::api::RustResponse;
use crate::bridge::api::RustSignal;
use crate::bridge::send_rust_signal;
use serde::Deserialize;
use prost::Message;

pub mod counter {
    include!(concat!(env!("OUT_DIR"), "/counter.rs"));
}

use counter::{CounterRequest, CounterResponse};


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

            // Decode the byte array back into a CounterRequest message.
            // Decode the byte array back into a CounterRequest message.
            let counter_request = CounterRequest::decode(&rust_request.bytes[..]).unwrap();
            let after_value:i32 = counter_request.before_number as i32 + 5;

            // Encode the CounterResponse message into a byte array.
            let mut counter_response = CounterResponse::default();
            counter_response.after_number = after_value;
            counter_response.dummy_one = counter_request.dummy_one as i32;
            counter_response.dummy_two = counter_request.dummy_two as i32;
            counter_response.dummy_three = vec![3, 4, 5];

            let mut bytes = Vec::new();
            counter_response.encode(&mut bytes).unwrap();
      

            
            RustResponse {
                successful: true,
                // Use `to_vec_named` from `rmp_serde`
                bytes:  bytes,
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
