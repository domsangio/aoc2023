use itertools::{Itertools, min};
use std::collections::HashMap;
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
    for c in conversions {
        println!("Conversion from {} to {}", c.source, c.dest);
        // is it faster to binary search?
        let mut prev_seed = starting_seeds.clone();

        for (seed_index, seed) in starting_seeds.iter().enumerate() {
            for (f_dest, f_source, f_range) in c.func.iter() {
                // println!("Current seed: {}, f_dest: {}, f_source: {}, f_range: {}", seed, f_dest, f_source, f_range);
                // println!("first part: {}, second part: {}", *f_source <= *seed, (*seed + *f_source) < *f_range );
                if *f_source <= *seed && *seed < *f_source + *f_range {
                    println!("Current seed '{}', falls in between '{}' and '{} + {}' so gets mapped to '{}'", *seed, *f_source, *f_source, *f_range, *f_dest + *seed - *f_source);
                    prev_seed[seed_index] = *f_dest + (*seed - *f_source);
                    break;
                }
            }
        }

        starting_seeds = prev_seed;
    }
    
    println!("Min found: {}", min(starting_seeds.iter()).unwrap());
}