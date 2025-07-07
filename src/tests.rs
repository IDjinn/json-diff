use std::fmt;
use crate::parse_json;

#[test]
pub(crate) fn hello_world_json_parse_test(){
    let str = "{ \"hello\": \"world\" }";
    let json = &serde_json::from_str(str).expect("failed to parse json using serde library");
    let json_data = parse_json(json).expect("failed to parse json");

    print!("{:#?}", json_data);
    assert!(json_data.fragments.len() > 0);
}