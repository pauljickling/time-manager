use chrono::prelude::*;

fn main() {
    let local: DateTime<Local> = Local::now();
    let mut start = String::from("this is a start");
    let mut summary = String::from("this is a summary");
    let activity = Activity {start, summary, local};

    println!("{} with the time {:?}", activity.start, activity.local);
    
}

struct Activity {
    start: String,
    summary: String,
    local: DateTime<Local>,
}
