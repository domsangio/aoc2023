use itertools::{Itertools, min};
use std::collections::{HashMap, VecDeque};
use regex::Regex;

struct Conversion {
    dest: String,
    source: String,
    func: Vec<(i64, i64, i64)>
}

fn get_starting_seeds(line: &str) -> Vec<(i64, i64)> {
    let mut ret: Vec<(i64, i64)> = Vec::new();
    let mut it = line.split(" ").skip(1);
    while let Some((seed, range)) = it.next_tuple() {
        ret.push((seed.parse::<i64>().unwrap(), range.parse::<i64>().unwrap()));
    }

    ret
}

fn construct_conversions<'a>(lines: &Vec<&str>, conversions: &'a mut Vec<Conversion>, conversion_index_map: &'a mut HashMap<&'a str, usize>) {
    let mut input_iter = lines.iter().skip(2);
    let conversion_pattern = Regex::new(r"^([a-z]+)-to-([a-z]+) map:$").unwrap();

    while let Some(conversion_line) = input_iter.next() {
        let captures = conversion_pattern.captures(&conversion_line).unwrap();

        // read rest of blob to actually get the conversion func
        let func_pattern = Regex::new(r"^(\d+) (\d+) (\d+)$").unwrap();
        let mut func: Vec<(i64, i64, i64)> = Vec::new();
        while let Some(func_line) = input_iter.next() {
            // break when we get to the new line
            if *func_line == "" {
                break;
            }

            let captures = func_pattern.captures(&func_line).unwrap();
            let convert_tup = (1..=3)
                .map(|index: usize| captures.get(index).unwrap().as_str().parse::<i64>().unwrap())
                .collect_tuple::<(_, _, _)>().unwrap();
            let insert_index =  match func.binary_search_by_key(&convert_tup.0, |(a, _, _)| *a) {
                Ok(i) => i,
                Err(i) => i
            };
            func.insert(insert_index, convert_tup);
        }

        conversions.push(
            Conversion {
                dest: captures.get(1).unwrap().as_str().to_string().clone(),
                source: captures.get(2).unwrap().as_str().to_string().clone(),
                func: func
            }
        );

        // conversion_index_map.insert(&conversions.last().unwrap().source, conversions.len() - 1);
    }

    for (i, c) in conversions.iter().enumerate() {
        conversion_index_map.insert(&c.source, i);
    }
}


// takes in a range [LB, amount) and seed range [Start, offset] and returns start, amount
// ex:: [10, 15), [12, 5] --> [12, 3]
fn find_overlap(conversion_range: (i64, i64), seed_range: (i64, i64)) -> (i64, i64) {
    let (conversion_lb, conversion_amount) = conversion_range;
    let (seed_range_lb, seed_range_amount) = seed_range;

    // perhaps off by one
    if conversion_lb <= seed_range_lb && seed_range_lb < conversion_lb + conversion_amount {
        return (seed_range_lb, std::cmp::min(seed_range_amount, (conversion_lb + conversion_amount) - (seed_range_lb + seed_range_amount)));
    }

    return (0, 0);
}

// left part of tuple is converted, right part is remaining
fn convert_range(seed_range: (i64, i64), c: &Conversion) -> ((i64, i64), (i64, i64)) {
    for conversion_func in c.func.iter() {
        let (converted, amount) = find_overlap((conversion_func.0, conversion_func.2), seed_range);
        if converted != 0 && amount != 0 {
            // convert this subsection
            let converted_tuple = (conversion_func.1 + (converted - seed_range.0), amount);

            // also is remaining tuple even correct idk
            let remaining_tuple = (conversion_func.0 + conversion_func.2, seed_range.1 - amount);
            return (converted_tuple, remaining_tuple);
        }
    }

    return (seed_range, (0,0));
}

/// need to switch to a quewue like object
pub fn day5b(input: &Vec<&str>) {
    // need to grab our starting seeds (sorted - mutable so we change them)
    let mut starting_seeds: Vec<(i64, i64)> = get_starting_seeds(input[0]);
    starting_seeds.sort();
    // TODO since its sorted can be clever

    // list of conversions, map of string to conversion-index in vec
    let mut conversions: Vec<Conversion> = Vec::new();
    let mut conversion_index_map: HashMap<&str, usize> = HashMap::new();

    construct_conversions(input, &mut conversions, &mut conversion_index_map);
    
    // for every conversion, try to convert this seed range
    for c in conversions {
        let mut processed: VecDeque<(i64, i64)> = VecDeque::new();
        let mut unprocessed = VecDeque::from_iter(starting_seeds.iter());

        while let Some(seed) = unprocessed.pop_front() {

            
        }
    }
    
    // println!("Min found: {}", min(starting_seeds.iter()).unwrap());
}