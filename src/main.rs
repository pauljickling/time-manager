// TODO use std::io;
#![allow(dead_code)]
use tm::{read, get_date};

enum Action {
    Start,
    Stop,
    Resume,
}

fn main() {
    use Action::*;

    println!("Read test");
    let path = String::from("activity_logs/sample.csv");
    let content = read(&path);
    println!("{:?}", content);
    
    println!("Struct test");
    let activity = String::from("sample task");
    let action = Start;
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

struct Record {
    activity: String,
    action_match: String,
    date: String,
}
