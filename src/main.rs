/* tm accepts the following:
 * 1. action parameters: "start", "stop", and "resume" are valid terms.
 * 2. activity string: used to record to csv
 */

#![allow(dead_code)]
use std::env;
use std::collections::HashSet;
// TODO use tm::{get_date, Record, Action};

fn main() {
    /* Probably not the best way to implement this HashSet, but everything else I did created
    string comparison errors. */
    let mut actions = HashSet::new();
    actions.insert("start".to_string());
    actions.insert("stop".to_string());
    actions.insert("resume".to_string());

    // env parameters
    let action_arg = env::args().nth(1);
    let _activity_arg = env::args().nth(2);
    
    // TODO tm should complain if None
    let action = match action_arg {
        Some(x) => x.to_string(),
        None => "none".to_string(),
    };

    if actions.contains(&action) {
        println!("{}", action);
    } else {
        println!("Invalid argument for action.\nstart, stop, and resume are valid arguments");
    }
    
} 
