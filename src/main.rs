/* tm accepts the following:
 * 1. action parameters: "start", "stop", and "resume" are valid terms.
 * 2. activity string: used to record to csv
 */

use std::env;
use std::collections::{HashSet, HashMap};

use tm::*;

fn main() {
    
    // env parameters
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

    let csv_path = format!("activity_logs/{}.csv", activity);
    let mut csv_content = read(&csv_path);
    // println!("{}", csv_content);

    /* Not ideal for this HashSet to be mutable, but everything else I did created
    string comparison errors. */
    let mut action_set = HashSet::new();
    action_set.insert("start".to_string());
    action_set.insert("stop".to_string());
    action_set.insert("resume".to_string());
    
    // checks to make sure a valid action happens, then does a bunch of stuff
    if action_set.contains(&action) {
        let record = create_record(&action);
        csv_content.push_str(&record);
        println!("{}", csv_content);
        let csv_vec = parse_csv(csv_content);
        // counter initially set to 6 as a silly hack to skip the header line
        let mut counter = 6;
        let mut activity_map = HashMap::new();
        let mut current_key = String::new();
        for i in csv_vec {
            if counter == 10 {
                counter = 1;
            }
            if counter == 1 {
               current_key = i.clone();
            }
            if counter == 2 {
                activity_map.insert(current_key, i.clone());
                current_key = String::from("");
            }
            if counter == 4 {
                counter = 1;
            } else {
                counter += 1;
            }
        }
        if Some(&action) == activity_map.get(&activity) {
            println!("{} is already the current action for {}", action, activity);
        } 
        // TODO use _check_file to handle errors
        // let _check_file = write_file(csv_path, csv_content);
    } else {
        println!("Invalid argument for action.\nstart, stop, and resume are valid arguments");
    }
} 
