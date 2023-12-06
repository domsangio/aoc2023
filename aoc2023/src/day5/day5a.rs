use itertools::Itertools;
use std::collections::HashMap;
use regex::Regex;

struct Conversion {
    source: String,
    dest: String,
    func: Vec<(i32, i32, i32)>
}

fn get_starting_seeds(line: &str) -> Vec<i32> {
    line.split(" ").skip(1).map(|num| num.parse::<i32>().unwrap()).collect()
}

fn construct_conversions(lines: &Vec<&str>) {
    let mut input_iter = lines.iter();
    input_iter.skip(2);

    let mut conversions: Vec<Conversion> = Vec::new();
    let mut conversion_index_map: HashMap<&str, usize> = HashMap::new();
    let conversion_pattern = Regex::new(r"^([a-z]+)-to-([a-z]+) map:$").unwrap();

    while let Some(conversion_line) = input_iter.next() {
        let captures = conversion_pattern.captures(&conversion_line).unwrap();

        // create the conversion object
        let mut conversion: Conversion = Conversion {
            source: captures.get(1).unwrap().as_str().to_string(),
            dest: captures.get(2).unwrap().as_str().to_string(),
            func: Vec::new()
        };

        // add it to the map and index map
        conversion_index_map.insert(&conversion.source, conversions.len());
        conversions.push(conversion);

        // read rest of blob to actually get the conversion func
        let func_pattern = Regex::new(r"^(\d+) (\d+) (\d+)$").unwrap();
        while let Some(func_line) = input_iter.next() {
            // break when we get to the new line
            if *func_line == "\n" {
                break;
            }

            let captures = func_pattern.captures(&func_line).unwrap();
            let convert_tup = (1..=3)
                .map(|index: usize| captures.get(index).unwrap().as_str().parse::<i32>().unwrap())
                .collect_tuple::<(_, _, _)>().unwrap();
            let insert_index = conversion.func.binary_search_by_key(&convert_tup.0, |(a, _, _)| *a).unwrap();
            conversion.func.insert(insert_index, convert_tup);
        }
    }
}

pub fn day5a(input: &Vec<&str>) {
    // need to grab our starting seeds
    let starting_seeds: Vec<i32> = get_starting_seeds(input[0]);
    
    // list of conversions, map of string to conversion-index in vec
    construct_conversions(input);

    println!("{}", input.iter().map(|line| line).sum::<u32>());
}