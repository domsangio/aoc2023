use std::collections::HashMap;
use regex::Regex;

pub struct Path {
    left: String,
    right: String
}

pub fn day8a(input: &Vec<&str>) {
    let order: Vec<char> = input[0].chars().collect();
    let mut steps = 0;
    let mut pos: &str = "AAA";

    let mut choices: HashMap<&str, Path> = HashMap::new();
    let choice_pattern = Regex::new("^([A-Z]+) = [(]([A-Z]+), ([A-Z]+)[)]$").unwrap();
    for line in input.iter().skip(2) {
        if let Some(captures) = choice_pattern.captures(line) {
            choices.insert(captures.get(1).unwrap().as_str(), Path {
                left: captures.get(2).unwrap().as_str().to_owned(),
                right: captures.get(3).unwrap().as_str().to_owned()
            });
        }
    }

    // for (x, p) in choices.iter() {
    //     println!("String: {}, path.left: {}, path.right: {}", *x, p.left, p.right);
    // }

    let mut order_iter = order.iter();
    loop {
        // println!("Pos: {}", pos);
        match order_iter.next() {
            Some('L') => {
                pos = &choices.get(pos).unwrap().left;
                steps += 1;
            },
            Some('R') => {
                pos = &choices.get(pos).unwrap().right;
                steps += 1;
            },
            _ => order_iter = order.iter()
        }

        if pos == "ZZZ" {
            println!("Answer: {}", steps);
            return;
        }
    }
}