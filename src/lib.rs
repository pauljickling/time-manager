use std::fs;
use std::process::Command;

// csv writer function for new and existing files
pub fn write_file(path: String, text: String) -> std::io::Result<()> {
    fs::write(path, text)?;
    Ok(())
}

// read file contents and returns a Result
pub fn read(path: &String) -> String {
    let contents = fs::read_to_string(path);
    let file = match contents {
        Ok(x) => x,
        _ => String::from("action, date/time stamp, unix time, hours\n"),
    };
    file
}

/* 
 * takes string of csv file and returns a vec for each item
 * The control flow for this is tedious and complicated.
 * It implements the following rules:
 * 1. assumes that entries contain no commas (i.e. they are delimiters)
 * 2. ignores spaces after commas
 * 3. also treats newline escape characters as delimiters
 */
pub fn parse_csv(src: String) -> Vec<String> {
    let mut item = String::new();
    let mut output = Vec::new();
    let mut skip = false;
    for c in src.chars() {
        if c != ',' {
            if skip == true {
                skip = false;
            } else {
                if c != '\n' {
                    item.push(c);
                } else {
                    output.push(item);
                    item = String::from("");
                }
            }
        } else {
            output.push(item);
            skip = true;
            item = String::from("");
        }
    }
    output
}

// gets date string from terminal. Used for sake of human readability in file.
pub fn get_date() -> String {
    let output = Command::new("date")
                          .output()
                          .expect("Failed to execute command");  
    let date_seq = output.stdout.as_slice();
    let mut date_str = String::new();
    for c in date_seq {
        if c.is_ascii() == true {
            date_str.push(*c as char);
        }
    }
    let date_slice = &date_str[0..28];
    let date = String::from(date_slice);
    date
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
    let time_slice = &time_str[0..10];
    let time = String::from(time_slice);
    time
}

// TODO calculate correct unit of time
pub fn calc_time(base: String, stop: String) -> f32 {
    let base_time = base.parse::<u32>().unwrap();
    let stop_time = stop.parse::<u32>().unwrap();
    let secs = stop_time - base_time;
    let time = secs as f32;
    time
}

// create record
pub fn create_record(action: &String) -> String {
    let delimiter = ", ";
    let mut record = String::new();
    record.push_str(action);
    record.push_str(delimiter);
    record.push_str(&get_date());
    record.push_str(delimiter);
    record.push_str(&get_unix_time());
    record.push_str(delimiter);
    // TODO handle calculation
    record.push_str("0\n");
    record
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
}
