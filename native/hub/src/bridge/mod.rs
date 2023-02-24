pub mod api;
mod bridge_generated;

pub fn update_viewmodel_with_json(item_address: &str, json_value: serde_json::Value) {
    let viewmodel_update_sender = api::VIEWMODEL_UPDATE_SENDER.get().unwrap().lock().unwrap();
    viewmodel_update_sender
        .send((
            String::from(item_address),
            json_value.to_string().as_bytes().to_vec(),
        ))
        .ok();
}

pub fn update_viewmodel_with_bytes(item_address: &str, bytes: Vec<u8>) {
    let viewmodel_update_sender = api::VIEWMODEL_UPDATE_SENDER.get().unwrap().lock().unwrap();
    viewmodel_update_sender
        .send((String::from(item_address), bytes))
        .ok();
}
