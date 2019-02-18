use std::fs;
use std::process::Command;

/// Used for writing a CSV file using the provided path and text
pub fn write_file(path: String, text: String) -> std::io::Result<()> {
    fs::write(path, text)?;
    Ok(())
}

/// Reads file contents from path and either:
/// 1. Returns contents as a String
/// 2. If file not found creates header for new file to be written
pub fn read_file(path: &String) -> String {
    let contents = fs::read_to_string(path);
    let file = match contents {
        Ok(x) => x,
        _ => String::from("action, date/time stamp, unix time, hours\n"),
    };
    file
}


/// Takes string of CSV file and returns a vec for each item.
/// The control flow for this is deeply nested and tedious. The purpose is to accomplish the
/// following:
/// 1. Treat commas as delimiters
/// 2. Ignores the space after a comma
/// 3. Treats newline escape characters as delimiters
pub fn parse_csv(src: &String) -> Vec<String> {
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

/// Gets date string from terminal. Provided for human readability in file.
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

/// Gets Unix timestamp. Used for ease of calculations.
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

/// Calculates time down to tenth of an hour (i.e. 6 minute intervals)
pub fn calc_time(base: String, stop: String) -> f64 {
    let base_time = base.parse::<f64>().unwrap();
    let stop_time = stop.parse::<f64>().unwrap();
    let secs: f64 = stop_time - base_time;
    let mins: f64 = secs / 60.0;
    let time: f64 =  mins / 60.0;
    time
}

/// Create record that automatically updates hours spent on activity
pub fn create_record(action: &String, base_time: &String, base_hours: &String) -> String {
    let delimiter = ", ";
    let mut record = String::new();
    record.push_str(action);
    record.push_str(delimiter);
    record.push_str(&get_date());
    record.push_str(delimiter);
    let unix_time = get_unix_time();
    record.push_str(&unix_time);
    record.push_str(delimiter);
    if action == "stop" {
        let base_int = base_hours.parse::<f64>().unwrap();
        let hours = calc_time(base_time.to_string(), unix_time);
        let total_hours = base_int + hours;
        let total_hours_str = format!("{:.1}\n", total_hours);
        record.push_str(&total_hours_str);
    } else {
        record.push_str(base_hours);
    }
    record
}

// TODO write meaningful tests for test_read() test_write_file() and test_get_date()
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_time() {
        let base= String::from("360");
        let stop = String::from("720");
        let time = calc_time(base, stop);
        assert_eq!(0.1, time);
    }
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
