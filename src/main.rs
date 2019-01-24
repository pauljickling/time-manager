use chrono::prelude::*;

fn main() {
    let local: DateTime<Local> = Local::now();
    let mut name = String::from("this is a name");
    let mut summary = String::from("this is a summary");
    let activity = Activity {start, summary, local};

    println!("{} with the time {:?}", activity.name, activity.local);
    
}

struct Activity {
    name: String,
    summary: String,
    local: DateTime<Local>,
}

// csv writer function for new and existing files
fn write_file(path: String, text: String) -> std::io::Result<()> {
 41     fs::write(path, text)?;
 42     Ok(())
 43 }

