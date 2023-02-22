use crate::api::DotAddress;
use crate::api::VIEWMODEL_UPDATE_SENDER;
use crate::model;
use serde_json::json;

pub fn calculate_something(json_value: serde_json::Value) {
    let _ = json_value;

    let mut value = model::COUNT.write().unwrap();
    *value = sample_crate::add_seven(*value);
    println!("{:}", *value);
    let json_value = json!({ "value": *value });

    let viewmodel_update_sender = VIEWMODEL_UPDATE_SENDER.get().unwrap().lock().unwrap();
    viewmodel_update_sender
        .send((
            DotAddress::from("someDataCategory.count"),
            json_value.to_string().as_bytes().to_vec(),
        ))
        .ok();
}
