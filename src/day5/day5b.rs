use itertools::{min, Itertools};
use regex::Regex;
use std::collections::{HashMap, VecDeque};

struct Conversion {
    dest: String,
    source: String,
    func: Vec<(i64, i64, i64)>,
}

fn get_starting_seeds(line: &str) -> Vec<(i64, i64)> {
    let mut ret: Vec<(i64, i64)> = Vec::new();
    let mut it = line.split(" ").skip(1);
    while let Some((seed, range)) = it.next_tuple() {
        ret.push((seed.parse::<i64>().unwrap(), range.parse::<i64>().unwrap()));
    }

    ret
}

fn construct_conversions<'a>(lines: &Vec<&str>, conversions: &'a mut Vec<Conversion>) {
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
                .map(|index: usize| {
                    captures
                        .get(index)
                        .unwrap()
                        .as_str()
                        .parse::<i64>()
                        .unwrap()
                })
                .collect_tuple::<(_, _, _)>()
                .unwrap();
            let insert_index = match func.binary_search_by_key(&convert_tup.0, |(a, _, _)| *a) {
                Ok(i) => i,
                Err(i) => i,
            };
            func.insert(insert_index, convert_tup);
        }

        conversions.push(Conversion {
            dest: captures.get(1).unwrap().as_str().to_string().clone(),
            source: captures.get(2).unwrap().as_str().to_string().clone(),
            func: func,
        });
    }
}

fn is_overlap(
    current_seed_start: i64,
    range_start: i64,
    range_length: i64,
) -> bool {
    current_seed_start >= range_start && current_seed_start < range_start + range_length
}

// return ((converted_range, amount), (front_range, length), (back_range, length))
fn get_overlap(
    current_seed_start: i64,
    current_seed_length: i64,
    range_start: i64,
    range_length: i64,
) -> ((i64, i64), (i64, i64), (i64, i64)) {
    // find start of current range and amount
    let start = std::cmp::max(current_seed_start, range_start);
    let end = std::cmp::min(range_start + range_length, start + current_seed_start);

    let front_start = std::cmp::min(start, current_seed_start);
    let front_length = start - front_start;

    let end_start = std::cmp::max(end, current_seed_start + current_seed_length);
    let end_length = std::cmp::max(0, range_start + range_length - end_start); // TODO unnecessary max?

    return (
        (start, end - start),
        (front_start, front_length),
        (end_start, end_length),
    );
}

// how do we convert a range? 
// we have source dest range
// dest + (seed start - source convert start)

/// need to switch to a quewue like object
pub fn day5b(input: &Vec<&str>) {
    // need to grab our starting seeds (sorted - mutable so we change them)
    let mut current_seeds: Vec<(i64, i64)> = get_starting_seeds(input[0]);

    // list of conversions we iterate over this
    let mut conversions: Vec<Conversion> = Vec::new();
    construct_conversions(input, &mut conversions);

    // for every conversion, try to convert this seed range
    for c in conversions.iter() {
        let mut i: usize = 0;
        while i < current_seeds.len() {
            // convert this range in place, throw whats left at the end
            // unefficiently, lets just find an overlap, convert it and add rest to end -- technically, we know that
            // since we converted the front part of the range, it cant be this one and we should really search better lets see
            let (curr_seed_start, curr_seed_range) = current_seeds[i];
 
            // TODO binary search here
            for (source, dest, amount) in c.func.iter() {
                if is_overlap(curr_seed_start, *source, *amount) {
                    let (converted, front, back) = get_overlap(curr_seed_start, curr_seed_range, *source, *amount);
                    
                    // modify current seed
                    current_seeds[i] = (dest + converted.0 - source, converted.1);

                    if front.1 != 0 {
                        current_seeds.push(front);
                    }
                    if back.1 != 0 {
                        current_seeds.push(back);
                    }
                    break;
                }
            }

            i += 1;
        }
    }

    println!("Min found: {}", min(current_seeds.iter().map(|(a, b)| a)).unwrap());
}
