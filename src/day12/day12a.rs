use core::num;
use std::str::Chars;

use itertools::Itertools;


fn print_info(idx: &usize, springs: &Vec<char>, lengths: &[i32]) {
    print!("Idx: {}, ", idx);
    for c in springs.iter().skip(*idx) {
        print!("{}", c);
    }
    print!(", lengths: ");
    for x in lengths.iter() {
        print!("{}, ", x);
    }
    print!("\n");
}

fn calculate_line_num_ways(idx: &usize, springs: &Vec<char>, lengths: &[i32]) -> i32 {
    print_info(idx, springs, lengths);

    if lengths.len() == 0 {
        // make suire no more '#' in the list
        for i in (*idx)..springs.len() {
            if springs[i] == '#' {
                println!("Ret 0");
                return 0;
            }
        }

        println!("Ret 1");
        return 1;
    }
    // if lengths.len() == 1 {
    //     // validate this last entry, since we are recursing and base case and stuff
    //     let mut in_a_row = 0;
    //     let mut start = idx.clone();

    //     while start < springs.len() {
    //         if springs[start] == '.' {
    //             start += 1;
    //         }
    //         else {
    //             break;
    //         }
    //     }

    //     while start < springs.len() {
    //         if springs[start] == '.' {
    //             break;
    //         }
    //         start += 1;
    //         in_a_row += 1;
    //     }
        

    //     return match in_a_row == lengths[0] {
    //         true => 1,
    //         false => 0
    //     };
    // }
    
    // find next non period
    let mut i = idx.clone();
    let num_ways = 0;

    while i < springs.len() {
        match springs[i] {
            '#' => {
                // we need to match this so lets try and test it as well
                let match_length = lengths[0];

                let mut in_a_row = 0;
                while i + in_a_row < springs.len() && springs[i + in_a_row] != '.' {
                    in_a_row += 1;
                }
                if (in_a_row as i32) < match_length {
                    return 0;
                }

                // make sure this skipping business works (pull first value off lengths too)
                return calculate_line_num_ways(&(i + match_length as usize + 1), springs, &lengths[1..])
            },
            '?' => {
                // if its a question mark, return num ways setting here + num ways not setting here

                // can we set it here - how do we ensure that its possible? right now we are just checking that char -
                // we need to iterate for match length, check how many #, and makle sure after it is not a # then recurse
                let mut in_a_row = 0;
                while i + in_a_row < springs.len() && springs[i + in_a_row] != '.' {
                    in_a_row += 1;
                }
                if (in_a_row as i32) >= lengths[0] {
                    // println!("Index: {} can fit the match of length: {} here", i, lengths[0]);
                    // yes we can set it here
                    // println!("Returning num_ways for two paths: idx: {}");
                    println!("Yes we can set it here, returning x + y");
                    print_info(&(i + lengths[0] as usize + 1), springs, &lengths[1..]);
                    print_info(&(i + 1), springs, lengths);
                    return calculate_line_num_ways(&(i + lengths[0] as usize + 1), springs, &lengths[1..]) + calculate_line_num_ways(&(i + 1), springs, lengths);
                } else {
                    return calculate_line_num_ways(&(i + 1), springs, lengths);
                }
            },
            _ => i += 1
        }
    }

    num_ways
}

pub fn day12a(input: &Vec<&str>) {
    let mut sum: i64 = 0;
    for line in input.iter() {
        let (springs, lengths) = line.split_once(' ').unwrap();

        sum += calculate_line_num_ways(&0, &springs.chars().collect_vec(), &lengths.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()[..]) as i64;
    }

    println!("Answer: {}", sum);
}