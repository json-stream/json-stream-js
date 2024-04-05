fn main() {
    let incomplete_json = r#"{
        "key": "value",
        "age": 23,
        "is_student": true,
        "height": 5.
    "#;
    let parsed_json = json_stream_parser::parse_stream(incomplete_json);
    if let Ok(json) = parsed_json {
        println!("{:?}", json);
    }
}
