use std::path::Path;
use chrono::{DateTime, Local};

pub fn start_time() {
    println!("Flow started a timer to measure your work time.");
    println!("You can stop the execution (Ctrl + C or Ctrl + D) if you want and then Flow will stop timing you.");
    let starting_point = Local::now();
}