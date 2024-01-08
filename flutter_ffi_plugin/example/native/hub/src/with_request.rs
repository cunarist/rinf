//! This module runs the corresponding function
//! when a `RustRequest` was received from Dart
//! and returns a `RustResponse`.

use crate::bridge::{RustRequestUnique, RustResponseUnique};
use crate::messages;
use crate::sample_functions;
use tokio_with_wasm::tokio;

pub async fn handle_request(request_unique: RustRequestUnique) -> RustResponseUnique {
    // Get the request data from Dart.
    let rust_request = request_unique.request;
    let interaction_id = request_unique.id;

    // Run the function that handles the Rust resource.
    let rust_resource = rust_request.resource;
    let operation_result = tokio::spawn(async move {
        match rust_resource {
            messages::counter_number::ID => {
                sample_functions::handle_counter_number(rust_request).await
            }
            messages::sample_folder::sample_resource::ID => {
                sample_functions::handle_sample_resource(rust_request).await
            }
            messages::sample_folder::deeper_folder::deeper_resource::ID => {
                sample_functions::handle_deeper_resource(rust_request).await
            }
            _ => None,
        }
    })
    .await;

    // Return the response to Dart.
    if let Ok(response_option) = operation_result {
        // When the handler function returned `Some` or `None`.
        RustResponseUnique {
            id: interaction_id,
            response: response_option,
        }
    } else {
        // When the handler function panicked.
        RustResponseUnique {
            id: interaction_id,
            response: None,
        }
    }
}
