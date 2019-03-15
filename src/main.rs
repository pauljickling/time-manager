use std::collections::HashSet;
use std::env;

use tm::*;

fn main() {
    // creates activity directory if none exists
    let _check_dir = create_activity_dir();

    // env parameters matched or else panic
    let action_arg = env::args().nth(1);
    let activity_arg = env::args().nth(2);

    let action = match action_arg {
        Some(x) => x.to_string(),
        None => "help".to_string(),
    };

    // if first parameter is "help" run help_text() and terminate
    if action == "help" {
        help_text();
    }

    // list parameter
    if action == "list" {
        let _list_result = list_activity();
        std::process::exit(0);
    }

    // otherwise activity must be specified
    let activity = match activity_arg {
        Some(x) => x.to_string(),
        None => {
            eprintln!("activity not specified");
            std::process::exit(0)
        }
    };

    // read csv file (or create a new header to be written if no file exists)
    let path = get_path();
    let csv_path = format!("{}/{}.csv", path, activity);

    // handle view arguments
    if action == "view" {
        view(&csv_path);
        std::process::exit(0);
    }

    let mut csv_content = read_file(&csv_path);
    let csv_vec = parse_csv(&csv_content);

    // check valid start statements
    if csv_vec.len() == 4 {
        if action != "start" {
            eprintln!("For a new activity action must be start");
            std::process::exit(0);
        }
    }

    if csv_vec.len() > 4 {
        if action == "start" {
            eprintln!(
                "Cannot use start for an existing activity.\nTry tm resume <activyt> instead"
            );
            std::process::exit(0);
        }
    }

    // last action is used to see if TM needs to complain about action parameter
    let last_action = &csv_vec[csv_vec.len() - 4];
    if last_action == &action {
        eprintln!("Last action must be different from current action");
        std::process::exit(0);
    }

    // HashSet is used to guarantee correct action parameters
    let mut action_set = HashSet::new();
    action_set.insert("start".to_string());
    action_set.insert("stop".to_string());
    action_set.insert("resume".to_string());

    // if valid action happens, then tm add an entry
    if action_set.contains(&action) {
        let base_time = &csv_vec[csv_vec.len() - 2];
        let base_hours = &csv_vec[csv_vec.len() - 1];
        let record = create_record(&action, &base_time, &base_hours);
        csv_content.push_str(&record);
        println!("{}", csv_content);
        let _result_check = write_file(csv_path, csv_content);
    } else {
        eprintln!("Invalid argument for action.\nstart, stop, and resume are valid arguments");
    }
}
