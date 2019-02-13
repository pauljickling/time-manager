/* tm accepts the following:
 * 1. action parameters: "start", "stop", and "resume" are valid terms.
 * 2. activity string: used to record to csv
 */

use std::env;
use std::collections::HashSet;
use tm::{get_date};

fn main() {
    /* Probably not the best way to implement this HashSet, but everything else I did created
    string comparison errors. */
    let mut actions = HashSet::new();
    actions.insert("start".to_string());
    actions.insert("stop".to_string());
    actions.insert("resume".to_string());

    // env parameters
    let action_arg = env::args().nth(1);
    let activity_arg = env::args().nth(2);
    
    // TODO tm should complain if None
    let action = match action_arg {
        Some(x) => x.to_string(),
        None => "none".to_string(),
    };

    let activity = match activity_arg {
        Some(x) => x.to_string(),
        None => "".to_string(),
    };

    if actions.contains(&action) {
        if activity == "" {
            println!("Activity must be specified");
        } else {
            let separator = ", ";
            let mut record = String::new();
            record.push_str(&activity);
            record.push_str(separator);
            record.push_str(&action);
            record.push_str(separator);
            record.push_str(&get_date());
            println!("{}", record);
        }
    } else {
        println!("Invalid argument for action.\nstart, stop, and resume are valid arguments");
    }
    
} 
