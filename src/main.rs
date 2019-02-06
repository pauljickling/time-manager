use std::io;
use std::fs;
use std::error::Error;
use std::process::Command;

fn main() {
    let name = String::from("programming");
    let mut entries: Vec<String> = Vec::new();
    entries.push(String::from("start"));
    entries.push(String::from(get_date()));
    entries.push(String::from("stop"));
    entries.push(String::from(get_date()));
    let activity = Activity {name, entries};

    println!("{} has the following entries: {:?}", activity.name, activity.entries);
    
}

struct Activity {
    name: String,
    entries: Vec<String>,
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
