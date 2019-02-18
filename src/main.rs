use std::env;
use std::collections::HashSet;

use tm::*;

fn main() {
    
    // env parameters matched or else panic
    let action_arg = env::args().nth(1);
    let activity_arg = env::args().nth(2);
     
    let action = match action_arg {
        Some(x) => x.to_string(),
        None => panic!("action not specified"),
    };

    let activity = match activity_arg {
        Some(x) => x.to_string(),
        None => panic!("activity not specified"),
    };

    // read csv file (or create a new header to be written if no file exists)
    let csv_path = format!("activity_logs/{}.csv", activity);
    let mut csv_content = read(&csv_path);
    let csv_vec = parse_csv(&csv_content);

    // check valid start
    if csv_vec.len() == 4 {
        if action != "start" {
            panic!("For new activity action must be start");
        }
    }

    if csv_vec.len() > 4 {
        if action == "start" {
            panic!("Cannot start existing activity");
        }
    }

    // last action helps figure out how the program should handle the user request
    let last_action = &csv_vec[csv_vec.len() - 4];
    if last_action == &action {
        panic!("Last action must be different from current action");
    }

    /* Not ideal for this HashSet to be mutable, but everything else I did created
    string comparison errors. */
    let mut action_set = HashSet::new();
    action_set.insert("start".to_string());
    action_set.insert("stop".to_string());
    action_set.insert("resume".to_string());
    
    // checks to make sure a valid action happens, then does a bunch of stuff
    if action_set.contains(&action) {
        let base_time = &csv_vec[csv_vec.len() - 2];
        let base_hours = &csv_vec[csv_vec.len() - 1];
        let record = create_record(&action, &base_time, &base_hours);
        csv_content.push_str(&record);
        println!("{}", csv_content);
        let _result_check = write_file(csv_path, csv_content);
    } else {
        println!("Invalid argument for action.\nstart, stop, and resume are valid arguments");
    }
} 
