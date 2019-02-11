// TODO use std::io;
#![allow(dead_code)]
use std::fs;
use std::error::Error;
use std::process::Command;

enum Action {
    Start,
    Stop,
    Resume,
}

fn main() {
    use Action::*;
    println!("Date test");
    let date = get_date();
    let date2 = date.clone();
    println!("{}", date);
    
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

// csv writer function for new and existing files
fn _write_file(path: String, text: String) -> std::io::Result<()> {
    fs::write(path, text)?;
    Ok(())
}

// read file contents and return Some or None
fn read(path: &String) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
    }

// gets date string from terminal
fn get_date() -> String {
    let output = Command::new("date")
                          .output()
                          .expect("Failed to execute command");  

    let date_seq = output.stdout.as_slice();
    let mut date = String::new();
    for c in date_seq {
        if c.is_ascii() == true {
            date.push(*c as char);
        }
    }
    date
}
