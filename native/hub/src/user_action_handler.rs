use crate::sample_functions;

pub async fn handle_user_action(task_address: String, json_string: String) {
    // `task_address` would be something like "some.task.address"
    // `json_string` would be something like "{'some':'json','string':true}"
    let json_value = serde_json::from_str(&json_string).unwrap();
    let layered: Vec<&str> = task_address.split(".").collect();

    if layered.len() == 0 {
    } else if layered[0] == "someTaskCategory" {
        if layered.len() == 1 {
        } else if layered[1] == "calculateSomething" {
            sample_functions::calculate_something(json_value);
        } else {
        }
    } else {
    }
}
