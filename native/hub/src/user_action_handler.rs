use crate::api::DotAddress;
use crate::sample_functions;

pub async fn handle_user_action(task_address: DotAddress, json_string: String) {
    // `task_address` would be something like "some.task.address"
    // `json_string` would be something like "{'some':'json','string':true}"
    let json_value = serde_json::from_str(&json_string).unwrap();

    if task_address.len() == 0 {
    } else if task_address[0] == "someTaskCategory" {
        if task_address.len() == 1 {
        } else if task_address[1] == "calculateSomething" {
            sample_functions::calculate_something(json_value);
        } else {
        }
    } else {
    }
}
