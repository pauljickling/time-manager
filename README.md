# Time Manager

Time Manager (tm) is a CLI written in Rust for managing and tracking time spent on specified activities. It outputs a csv file with this data.

## Installation

At the moment I don't have binaries available, so you should have Rust [installed](https://www.rust-lang.org/tools/install). I expect to make those available in the future after I have tested the tool on different operating systems.

I will be publishing this on [crates.io](https://crates.io) shortly, and will have instructions for a crates installation once I understand that process.

### Github Installation

Clone this repository into a specified directory.

`git clone https://github.com/pauljickling/time-manager.git`

After you have installed, change into the directory and run:

`cargo build --release`

This will install an optimized build of the platform

### Create an Alias in your .bashrc File

For ease of use, I recommend creating an alias in your .bashrc file.

`alias tm='$HOME/path/to/target/release/tm'`

Where `path/to` is the location you installed the time-manager files.

If you have never modified a .bashrc file before you can read about it in *Linux Command Line Fourth Internet Edition* by William Shotts. You can acquire a free copy of the book [here](http://www.linuxcommand.org/tlcl.php/). You can read about the relevant information in Chapter 11.

## Using Time Manager

`tm start {activity}` creates a new csv file with the specified activity

`tm stop {activity})` creates a stop record entry for the csv file and adds up the total time spent on the activity since the last entry was recorded

`tm resume {activity}` creates a resume entry for the csv file

Of course if you have not specified an alias you will need to type in `cargo run` in the correct directory instead.

## The CSV files

The csv files contain four fields:

1. The specified action
2. A time/date stamp
3. A Unix timestamp
4. Total hours spent on that activity
