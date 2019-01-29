use std::fs;
use std::error::Error;

fn main() {
    let mut name = String::from("programming");
    let mut entries: Vec<String> = Vec::new();
    let activity = Activity {name, entries};

    println!("{} has the following entries: {:?}", activity.name, activity.entries);
    
}

struct Activity {
    name: String,
    entries: Vec<String>,
}

// csv writer function for new and existing files
fn write_file(path: String, text: String) -> std::io::Result<()> {
    fs::write(path, text)?;
    Ok(())
}

// read file contents and return Some or None
fn read(path: &String) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

