extern crate itertools;

use std::env;
use std::fs;

mod day1;
mod day2;
mod day3;
mod day5;
mod day6;

fn main() {
    /* Set up command line arguments for parsing and executing */
    /* given day/problem set up parser and interpreter */

    let args: Vec<String> = env::args().collect();

    // if the inputs are different
    // let file_path: String = format!("input/day{0}/{1}_input", args[1], args[2]);
    let file_path = format!("input/day{0}/a_input", args[1]);
 
    let content = fs::read_to_string(file_path).unwrap();

    // Split the string into lines and collect them into a vector
    let lines: Vec<&str> = content.lines().collect();


    let tup_args = (args[1].as_str(), args[2].as_str());
    match tup_args {
        ("1", "a") => day1::day1a::day1a(&lines),
        ("1", "b") => day1::day1b::day1b(&lines),
        ("2", "a") => day2::day2a::day2a(&lines),
        ("2", "b") => day2::day2b::day2b(&lines),
        ("3", "a") => day3::day3a::day3a(&lines),
        ("5", "a") => day5::day5a::day5a(&lines),
        ("5", "test") => day5::day5a::day5a(&lines),
        ("5", "b") => day5::day5b::day5b(&lines),
        ("6", "a") => day6::day6a::day6a(&lines),
        ("6", "b") => day6::day6b::day6b(&lines),
        _ => println!("WTF happened")
    }
}
