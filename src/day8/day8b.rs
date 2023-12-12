
use std::collections::{HashMap, HashSet};
use regex::Regex;

struct Path {
    left: String,
    right: String
}

// for a given node, find its cycle
// the cycle will be the pos str and the index into the choice loop
// note, the cycle does not necessarily mean we start there? we could do like A -> B -> C -> B -> C
fn find_cycle<'a>(choice_loop: &'a Vec<char>, start: &'a str, choices: &'a HashMap<&'a str, Path>) -> (&'a str, i32) {
    let mut visited: HashMap<&str, HashSet<i32>> = HashMap::new();
    let mut pos = start;
    let mut pos_choice = 0;
    loop {
        // if visited.contains_key(pos) && visited.get(pos).unwrap().contains(pos_choice) {
        //     // found cycle at this point
        //     return (pos, pos_choice);
        // }
        
        // add to visited
        if !visited.contains_key(pos) {
            visited.insert(pos, HashSet::new());
        }
        // visited.get(pos).unwrap().insert(pos_choice);

        //increment pos and pos_choice
        pos_choice = (pos_choice + 1) % (choice_loop.len() as i32);
        match choice_loop[pos_choice as usize] {
            'L' => {
                pos = &choices.get(pos).unwrap().left;

            },
            'R' => {
                pos = &choices.get(pos).unwrap().right;
            },
            _ => panic!("uhoh")
        }
    }
}

// where do the z indices occur, how longs the loop, and where does the start of the loop begin
fn find_z_indices_and_loop_len_and_offset<'a>(start: &'a str, choice_loop: &'a Vec<char>, choices: &'a HashMap<&'a str, Path>) {
    let mut z_indices = Vec::<i32>::new();
    let mut loop_len = 0;
    let mut offset = 0;

    // seen of map from str -> index of where we see it
    let mut seen = HashMap::<&str, i32>::new();
    let mut choice_iter = choice_loop.iter();
    let mut pos: &str = start;
    let mut steps = 0;

    loop {
        if seen.contains_key(pos) {
            loop_len = steps;
        }
        match choice_iter.next() {
            Some('L') => {
                steps += 1;
            },
            Some('R') => {
                steps += 1;
            },
            // _ => order_iter = order.iter()
            _ => panic!("uhohhhh")
        }
    }
}

// TODO not fast enough, track cycle once its reached we dont have to use the map?
// maybe im missing something - iterate 1 by 1 and see when we hit the Zs?
pub fn day8b(input: &Vec<&str>) {
    let order: Vec<char> = input[0].chars().collect();
    let mut steps = 0;
    let mut positions: Vec<&str> = Vec::new();

    let mut choices: HashMap<&str, Path> = HashMap::new();
    let choice_pattern = Regex::new("^([A-Z0-9]+) = [(]([A-Z0-9]+), ([A-Z0-9]+)[)]$").unwrap();
    for line in input.iter().skip(2) {
        println!("{}", line);
        if let Some(captures) = choice_pattern.captures(line) {
            choices.insert(captures.get(1).unwrap().as_str(), Path {
                left: captures.get(2).unwrap().as_str().to_owned(),
                right: captures.get(3).unwrap().as_str().to_owned()
            });

            // println!("captures 1: {}", captures.get(1).unwrap().as_str());
            if captures.get(1).unwrap().as_str().chars().nth(2).unwrap() == 'A' {
                positions.push(captures.get(1).unwrap().as_str());
            }
        }
    }

    for x in positions.iter() {
        println!("Starings: {}", x);
    }

    // for (x, p) in choices.iter() {
    //     println!("String: {}, path.left: {}, path.right: {}", *x, p.left, p.right);
    // }

    let mut order_iter = order.iter();
    loop {
        // print!("Current position: ");
        // for x in positions.iter() {
        //     print!("{}, ", x);
        // }
        // print!("\n");

        // println!("Pos: {}", pos);
        match order_iter.next() {
            Some('L') => {
                // iterate all options to the left/right
                // pos = &choices.get(pos).unwrap().left;
                for i in 0..positions.len() {
                    // println!("Shifting: {} to the left: {}", positions[i], &choices.get(positions[i]).unwrap().left);
                    positions[i] = &choices.get(positions[i]).unwrap().left;
                }
                steps += 1;
            },
            Some('R') => {
                for i in 0..positions.len() {
                    // println!("Shifting: {} to the right: {}", positions[i], &choices.get(positions[i]).unwrap().right);
                    positions[i] = &choices.get(positions[i]).unwrap().right;
                }
                steps += 1;
            },
            _ => order_iter = order.iter()
        }

        // print!("after we shiftede position: ");
        // for x in positions.iter() {
        //     print!("{}, ", x);
        // }
        // print!("\n");

        if steps % 10000 == 0 {
            println!("Executed {} steps", steps);
        }


        let mut all_end_with_z = true;
        for x in positions.iter() {
            if x.chars().nth(2).unwrap() != 'Z' {
                all_end_with_z = false;
                break;
            }
        }
        if all_end_with_z {
            println!("Answer: {}", steps);
            return;
        }
    }
}