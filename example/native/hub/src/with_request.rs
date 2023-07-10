//! This module runs the corresponding function
//! with the received requests
//! and returns the response.

use crate::bridge::api::RustRequestUnique;
use crate::bridge::api::RustResponse;
use crate::bridge::api::RustResponseUnique;
use crate::bridge::respond_to_dart;
use crate::sample_functions;

pub async fn handle_request(request_unique: RustRequestUnique) {
    // Get the request data.
    let rust_request = request_unique.request;
    let interaction_id = request_unique.id;

    // Run the function that corresponds to the address.
    let layered: Vec<&str> = rust_request.address.split('.').collect();
    let rust_response = if layered.is_empty() {
        RustResponse::default()
    } else if layered[0] == "basicCategory" {
        if layered.len() == 1 {
            RustResponse::default()
        } else if layered[1] == "counterNumber" {
            sample_functions::calculate_something(rust_request).await
        } else {
            RustResponse::default()
        }
    } else {
        RustResponse::default()
    };

    // Return the response.
    let response_unique = RustResponseUnique {
        id: interaction_id,
        response: rust_response,
    };
    respond_to_dart(response_unique);
}
