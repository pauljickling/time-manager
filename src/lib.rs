use std::fs;
use std::error::Error;
use std::process::Command;

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

// gets date string from terminal
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fail_test() {
        assert_eq!(true, false);
    }

    #[test]
    fn test_write_file() {
        assert!(false)
    }

    #[test]
    fn test_read() {
        assert!(false)
    }

    #[test]
    fn test_get_date() {
        assert!(false)
    }
}
