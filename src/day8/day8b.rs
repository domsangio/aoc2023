use regex::Regex;
use std::collections::HashMap;

struct Path {
    left: String,
    right: String,
}

fn iterate<'a>(
    curr_word: &'a str,
    index: usize,
    choice_loop: &Vec<char>,
    choices: &'a HashMap<&'a str, Path>,
) -> &'a str {
    match choice_loop[index % choice_loop.len()] {
        'L' => &choices.get(curr_word).unwrap().left,
        _ => &choices.get(curr_word).unwrap().right,
    }
}

// return cycle start and cycle end usize
fn find_cycle<'a>(
    choice_loop: &'a Vec<char>,
    start: &'a str,
    choices: &'a HashMap<&'a str, Path>,
) -> (usize, usize) {
    let mut index: usize = 0;
    // index into the choice loop and the word were on and this points to index we found it on
    let mut visited: HashMap<(usize, &str), usize> = HashMap::new();
    let mut pos = start;

    loop {
        println!("Curr pos: {}, index: {},  direction: {}", pos, index, index % choice_loop.len());
        if let Some(x) = visited.get(&(index % choice_loop.len(), pos)) {
            println!("Seen this here: {}", *x);
            return (*x, index);
        }

        visited.insert((index % choice_loop.len(), pos), index);

        //increment pos and pos_choice
        pos = iterate(pos, index, choice_loop, choices);
        index += 1;
    }
}

// vec of indices that holds the starting Zs before its in the cycle
fn get_starting_zs<'a>(
    curr_word: &'a str,
    choice_loop: &Vec<char>,
    choices: &'a HashMap<&'a str, Path>,
    cycle_start: usize,
) -> Vec<usize> {
    let mut starting_zs: Vec<usize> = Vec::new();

    let mut pos = curr_word;
    let mut index = 0;

    loop {
        if index == cycle_start {
            return starting_zs;
        }

        if pos.chars().nth_back(0) == Some('Z') {
            starting_zs.push(index);
        }

        pos = iterate(pos, index, choice_loop, choices);
        index += 1;
    }
}

fn find_z_indices_in_loop<'a>(
    curr_word: &'a str,
    choice_loop: &'a Vec<char>,
    choices: &'a HashMap<&'a str, Path>,
    cycle_start: usize,
    cycle_end: usize,
) -> Vec<usize> {
    // iterate until the start
    let mut ret: Vec<usize> = Vec::new();

    let mut pos: &str = curr_word;
    let mut index: usize = 0;

    while index < cycle_start {
        pos = iterate(pos, index, choice_loop, choices);
        index += 1;
    }

    loop {
        if index == cycle_end {
            return ret;
        }

        if pos.chars().nth_back(0) == Some('Z') {
            ret.push(index);
        }

        pos = iterate(pos, index, choice_loop, choices);
        index += 1;
    }
}


pub fn day8b(input: &Vec<&str>) {
    let order: Vec<char> = input[0].chars().collect();
    let mut steps = 0;
    let mut positions: Vec<&str> = Vec::new();

    let mut choices: HashMap<&str, Path> = HashMap::new();
    let choice_pattern = Regex::new("^([A-Z0-9]+) = [(]([A-Z0-9]+), ([A-Z0-9]+)[)]$").unwrap();
    for line in input.iter().skip(2) {
        println!("{}", line);
        if let Some(captures) = choice_pattern.captures(line) {
            choices.insert(
                captures.get(1).unwrap().as_str(),
                Path {
                    left: captures.get(2).unwrap().as_str().to_owned(),
                    right: captures.get(3).unwrap().as_str().to_owned(),
                },
            );

            // println!("captures 1: {}", captures.get(1).unwrap().as_str());
            if captures.get(1).unwrap().as_str().chars().nth(2).unwrap() == 'A' {
                positions.push(captures.get(1).unwrap().as_str());
            }
        }
    }

    for x in positions.iter() {
        println!("starting: {}", x);
    }

    // for (x, p) in choices.iter() {
    //     println!("String: {}, path.left: {}, path.right: {}", *x, p.left, p.right);
    // }

    let mut all_starting_zs: Vec<Vec<usize>> = Vec::new();
    let mut all_cycle_zs: Vec<Vec<usize>> = Vec::new();
    let mut cycle_bounds: Vec<(usize, usize)> = Vec::new();

    for starting_position in positions {
        let (cycle_start, cycle_end) = find_cycle(&order, starting_position, &choices);
        let starting_zs = get_starting_zs(starting_position, &order, &choices, cycle_start);
        let cycle_zs = find_z_indices_in_loop(starting_position, &order, &choices, cycle_start, cycle_end);

        all_starting_zs.push(starting_zs);
        all_cycle_zs.push(cycle_zs);
        cycle_bounds.push((cycle_start, cycle_end));
    }

    // Do the math here
    for x in all_starting_zs.iter() {
        for y in x {
            print!("{} ", y);
        }
        println!("");
    }

    println!("Cycle Zs");
    for x in all_cycle_zs.iter() {
        for y in x {
            print!("{} ", y);
        }
        println!("");
    }

    for (start, end) in cycle_bounds.iter() {
        println!("Start: {}, End: {}", start, end);
    }


    // get into the cycles first
    let max_cycle_start = cycle_bounds
        .iter()
        .map(|(start, _)| *start)
        .max()
        .unwrap();

    // // TODO get this part to work
    // for starting_zs in all_starting_zs.iter() {
    //     for starting_z in starting_zs.iter() {
    //     }
    // }

    

}
