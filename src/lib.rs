use std::fs;
use std::process::Command;

extern crate dirs;

/// Used for writing a CSV file using the provided path and text
pub fn write_file(path: String, text: String) -> std::io::Result<()> {
    fs::write(path, text)?;
    Ok(())
}

/// Reads file contents from path and returns String or new header
pub fn read_file(path: &String) -> String {
    let contents = fs::read_to_string(path);
    let file = match contents {
        Ok(x) => x,
        _ => String::from("action, date/time stamp, unix time, hours\n"),
    };
    file
}

/// Removes file specified in parameter
pub fn remove(path: &str) -> std::io::Result<()> {
    warning();
    fs::remove_file(path)?;
    Ok(())
}

/// TODO: Provide warning for any function that removes things
pub fn warning() {}

/// Displays the contents of a csv file, or error message if no file found
pub fn view(path: &String) {
    let contents = fs::read_to_string(path);
    match contents {
        Ok(x) => println!("{}", x),
        _ => eprintln!("No file found with that name"),
    }
}

/// Truncate contents of a csv file to last 5 lines
pub fn truncate(contents: &String) -> String {
    let mut count = 0;

    for i in contents.chars() {
        if i == '\n' {
            count += 1;
        }
    }

    let truncate_start = count - 5;
    let mut count2 = 0;
    let mut truncated_contents = String::from("action, date/time stamp, unix time, hours\n. . .\n");
    
    if count > 5 {
        for i in contents.chars() {
            if count2 >= truncate_start {
                truncated_contents.push(i);
            }
            if i == '\n' {
                count2 += 1;
            }
       } 
    } else {
        truncated_contents = String::from(contents);
    }
    truncated_contents
}

/// Return activity_logs path
pub fn get_path() -> String {
    let path = dirs::document_dir().unwrap();
    let mut path_str = match path.to_str() {
        Some(x) => String::from(x),
        _ => panic!("Could not retrieve directory path"),
    };
    path_str.push_str("/tm_activity_logs/");
    path_str
}

/// Creates an activity_logs directory if it doesn't exist
pub fn create_activity_dir() -> std::io::Result<()> {
    let path = get_path();
    fs::create_dir(path)?;
    Ok(())
}

/// TODO: Archives specified activities.
pub fn archive(name: &String) -> std::io::Result<()> {
    let path = dirs::document_dir().unwrap();
    let mut path_str = match path.to_str() {
        Some(x) => String::from(x),
        _ => panic!("Could not retrieve directory path"),
    };
    path_str.push_str("/tm_activity_logs/");
    path_str.push_str(name);
    fs::create_dir(path_str)?;
    Ok(())
}

/// Takes string of CSV file and returns a vec of entries.
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

/// Get last item of a Vec to calculate totals
pub fn get_hrs(csv: Vec<&str>) -> f64 {
    let final_item = csv.len();
    let total = csv[final_item - 1].parse::<f64>().unwrap();
    total
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
    let time: f64 = mins / 60.0;
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
    if action == "start" {
        record.push_str("0.0\n");
    }
    if action == "stop" {
        let base_int = base_hours.parse::<f64>().unwrap();
        let hours = calc_time(base_time.to_string(), unix_time);
        let total_hours = base_int + hours;
        let total_hours_str = format!("{:.1}\n", total_hours);
        record.push_str(&total_hours_str);
    }
    if action == "resume" {
        let base_hours_str = format!("{}\n", base_hours);
        record.push_str(&base_hours_str);
    }
    record
}

/// Provides help text when user provides help parameter
pub fn help_text() {
    let help = "
Usage: tm <action> <activity>

where <action> is one of:
    start, stop, resume

and <activity> is any valid string that does not contain commas.
    
Additionally: 
    tm list             lists activity csv files
    tm view <activity>  csv file sent to stdout";

    println!("{}", help);
    std::process::exit(0);
}

/// Lists existing activities when user provides list parameter
pub fn list_activity() -> std::io::Result<()> {
    let path = get_path();
    println!("List of csv files for activities:");
    for entry in fs::read_dir(path)? {
        let file = entry?;
        let file_slice = file.path().display().to_string();
        if file_slice.contains("csv") == true {
            println!("{}", &file_slice);
        }
    }
    Ok(())
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_time() {
        let base = String::from("360");
        let stop = String::from("720");
        let time = calc_time(base, stop);
        assert_eq!(0.1, time);
    }
    #[test]
    fn test_get_date() {
        let date = get_date();
        assert_eq!(date, get_date());
    }
    #[test]
    fn test_parse_csv() {
        let csv_string = String::from(
            "start, Tue May  7 14:42:28 PDT 2019, 1557265348, 0.0
stop, Tue May  7 14:43:00 PDT 2019, 1557265380, 0.0
",
        );
        let vec_eval1 = vec![
            "start",
            "Tue May  7 14:42:28 PDT 2019",
            "1557265348",
            "0.0",
            "stop",
            "Tue May  7 14:43:00 PDT 2019",
            "1557265380",
            "0.0",
        ];

        let vec_eval2 = parse_csv(&csv_string);
        assert_eq!(vec_eval1, vec_eval2);
    }
    #[test]
    fn test_create_record() {
        let base_time = get_date();
        let unix_time = get_unix_time();
        let record1 = format!("start, {}, {}, 0.0\n", base_time, unix_time);
        let action = String::from("start");
        println!("{}", base_time);
        let base_hours = String::from("0.0");

        let record2 = create_record(&action, &base_time, &base_hours);

        assert_eq!(record1, record2);
    }
    #[test]
    fn test_path() {
        let docs = dirs::document_dir().unwrap();
        let mut doc_str = match docs.to_str() {
            Some(x) => String::from(x),
            _ => panic!("problem with getting doc path"),
        };
        doc_str.push_str("/tm_activity_logs/");
        let path = get_path();
        assert!(
            path == doc_str,
            "Function was {} and test was {}",
            path,
            doc_str
        );
    }
    #[test]
    fn test_archive() {
        let foo = String::from("foo");
        archive(&foo);
    }
    #[test]
    fn test_get_hrs() {
        let csv_vec_sample = vec![
            "start",
            "Tue May  7 14:42:28 PDT 2019",
            "1557265348",
            "0.0",
            "stop",
            "Tue May  7 16:43:00 PDT 2019",
            "1557247380",
            "2.0",
        ];
        let result = get_hrs(csv_vec_sample);
        assert_eq!(result, 2.0);
    }
}
