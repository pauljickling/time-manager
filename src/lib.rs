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
    let date_slice = &date[0..28];
    let return_str = String::from(date_slice);
    return_str
}

// gets Unix timestamp. Used for ease of calculations.
pub fn get_unix_time() -> String {
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
    time_str
}

// calculates hours for time
pub fn calc_time(base: String, stop: String) -> f32 {
    let base_time = base.parse::<u32>().unwrap();
    let stop_time = stop.parse::<u32>().unwrap();
    let secs = stop_time - base_time;
    let time = secs as f32;
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
}
