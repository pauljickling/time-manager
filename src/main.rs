#![allow(dead_code)]
use std::env;

// TODO use tm::{get_date, Record, Action};

fn main() {
    if env::args().len() != 3 {
        println!("Incorrect number of arguments.\nFormat is as follows:\ntm action activity");
    } else {
        let action = env::args().nth(1);
        let activity = env::args().nth(2);
        println!("{:?}\n{:?}", action.unwrap(), activity.unwrap());
    }
} 
