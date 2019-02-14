use std::fs;
use std::error::Error;
use std::process::Command;

pub struct Record {
    pub activity: String,
    pub action_match: String,
    pub date: String,
}

pub enum Action {
    Start,
    Stop,
    Resume,
}

// date parser adds up hours
pub fn _date_parser(date: String) -> String { 
    let _month = &date[4..8];
    let _day = &date[8..10];
    let _hour = &date[10..13];
    let _minute = &date[14..16];
    let _year = &date[24..28];
    let _hour_float: f32 = _hour.parse().unwrap();
    let mut hours = 0.0;
    hours = hours + _hour_float;
    hours.to_string()
}
// csv writer function for new and existing files
pub fn _write_file(path: String, text: String) -> std::io::Result<()> {
    fs::write(path, text)?;
    Ok(())
}

// read file contents and return Some or None
pub fn read(path: &String) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
    }

// gets date string from terminal. Used for sake of human readability in file.
pub fn get_date() -> String {
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

// gets Unix timestamp. Used for ease of calculations.
pub fn get_unix_timestamp() -> u32 {
    let output = Command::new("date")
                          .arg("+%s")
                          .output()
                          .expect("Failed to execute command");
    let time_seq = output.stdout.as_slice();
    let mut time_str = String::new();
    for c in time_seq {
        if c.is_ascii() == true {
            time_str.push(*c as char);
        }
    }
    let time_splice = &time_str[0..10];
    let time = time_splice.parse::<u32>().unwrap();
    time
}

// TODO write meaningful tests for test_read() test_write_file() and test_get_date()
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        println!("Read test");
        let path = String::from("activity_logs/sample.csv");
        let content = read(&path);
        println!("{:?}", content);
    }

    #[test]
    fn test_write_file() {
        assert_eq!(true, false);
    }

    #[test]
    fn test_get_date() {
        let date = get_date();
        assert_eq!(date, get_date());
    }
    #[test]
    fn test_struct() {
        let activity = String::from("sample task");
        let action = Action::Resume;
        let date = get_date();
        let date2 = date.clone();
        let action_match = match action {
            Action::Start => String::from("start"),
            Action::Stop => String::from("stop"),
            Action::Resume => String::from("resume"),
        };
        let record = Record {activity, action_match, date};
        assert_eq!(record.activity, String::from("sample task"));
        assert_eq!(record.action_match, "resume");
        assert_eq!(record.date, date2);
    }
    #[test]
    fn test_date_parser() {
        assert_eq!(true, false);
    }
}
