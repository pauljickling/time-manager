// TODO use std::io;
#![allow(dead_code)]
use tm::{get_date, Record, Action};

fn main() {
    use Action::*;
    
    println!("Struct test");
    let activity = String::from("sample task");
    let action = Start;
    let date = get_date();
    let date2 = date.clone();
    let action_match = match action {
        Action::Start => String::from("start"),
        Action::Stop => String::from("stop"),
        Action::Resume => String::from("resume"),
    
    };
    let record = Record {activity, action_match, date};
    assert_eq!(record.activity, "sample task");
    assert_eq!(record.action_match, "start");
    assert_eq!(record.date, date2);
}
