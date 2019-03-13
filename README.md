# Time Manager

Time Manager (tm) is a CLI written in Rust for managing and tracking time spent on specified activities. It outputs a csv file with this data. Time is measured in 10ths of an hour (i.e. six minutes).

## Installation

Clone this repository into a specified directory.

`git clone https://github.com/pauljickling/time-manager.git`

After you have installed, change into the directory and run:

`cargo build --release`

This will install an optimized build of the platform. 

### Create an Alias in your .bashrc File

For ease of use, I recommend creating an alias in your .bashrc file.

`alias tm='$HOME/path/to/target/release/tm'`

Where `path/to` is the location you installed the time-manager files.

If you have never modified a .bashrc file before you can read about it in *Linux Command Line Fourth Internet Edition* by William Shotts. You can acquire a free copy of the book [here](http://www.linuxcommand.org/tlcl.php/), and the relevant information can be found in Chapter 11.

## Using Time Manager

Time manager accepts two parameters in this order: `tm {action} {activity}`.

`tm start {activity}` creates a new csv file with the specified activity.

`tm stop {activity})` creates a stop record entry for the csv file and adds up the total time spent on the activity since the last entry was recorded.

`tm resume {activity}` creates a resume entry for the csv file.

`tm help` lists valid syntax for the Time Manager.

Of course if you have not specified an alias you will need to type in `cargo run` in the correct directory instead.

The `{activity}` specified will be the name of the csv file.

### Errors Using Time Manager

The following uses of Time Manager are errors that will cause the program to complain:

- Using tm with no specified parameters
- Using an invalid action parameter
- Using start as an action parameter for an activity that already exists
- Using an action parameter that was the last used action for an activity

Note that using a `resume` action after `start` is considered a benign bug, and Time Manager will not complain about it.

## The CSV files

The csv files contain four fields:

1. The specified action
2. A time/date stamp
3. A Unix timestamp
4. Total hours spent on that activity

## OS Support

This program has been tested on Linux Ubuntu 18.04 LTS. It may not work on certain versions of Linux depending on pathing implementation details. Testing on OSX and Windows will be forthcoming.
