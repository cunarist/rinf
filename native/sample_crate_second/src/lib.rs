pub fn multiply_two(json: String) {
    let json_structure: serde_json::Value = serde_json::from_str(&json).unwrap();
    let before = json_structure["theValue"].as_i64().unwrap() as i32;
    let after = before * 2;
    println!("{:?}", after);
}
