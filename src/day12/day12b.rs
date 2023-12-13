use std::collections::HashMap;

use itertools::Itertools;

fn print_info(idx: &usize, springs: &Vec<char>, lengths: &[i64]) {
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

fn calculate_line_num_ways(
    idx: &usize,
    springs: &Vec<char>,
    lengths: &[i64],
    memoi: &mut HashMap<(usize, usize), i64>,
    mut placed: Vec<(usize, usize)>,
    og_lengths: &[i64]
) -> i64 {
    // print_info(idx, springs, lengths);

    if memoi.contains_key(&(*idx, lengths.len())) {
        return memoi.get(&(*idx, lengths.len())).unwrap().clone();
    }
    if *idx >= springs.len() {
        if lengths.len() != 0 {
            return 0;
        }
    }

    if lengths.len() == 0 {
        // make suire no more '#' in the list
        for i in (*idx)..springs.len() {
            if springs[i] == '#' {
               return 0;
            }
        }

        let mut replaced_string: Vec<char> = Vec::new();
        for a in springs.iter() {
            replaced_string.push(a.clone());
        }
        for (idx, length) in placed.iter() {
            for i in *idx..(*idx + *length) {
                replaced_string[i] = '#';
            }
        }
        for i in 0..replaced_string.len() {
            if replaced_string[i] == '?' {
                replaced_string[i] = '.';
            }
        }

        let pre_truncated_str = replaced_string.clone();

        // print!("new string: ");
        // for x in replaced_string.iter() {
        //     print!("{}", x);
        // }
        // println!();

        let mut i = 0;
        while i < replaced_string.len() - 1 {
            if replaced_string[i] == '.' && replaced_string[i + 1] == '.' {
                replaced_string.remove(i);
            } else {
                i += 1;
            }
        }
        if replaced_string[0] == '.' {
            replaced_string.remove(0);
        }
        if replaced_string[replaced_string.len() - 1] == '.' {
            replaced_string.remove(replaced_string.len() - 1);
        }

        // print!("replaced str: ");
        // for x in replaced_string.iter() {
        //     print!("{}", x);
        // }
        // println!();
        
        for (length_index, line) in replaced_string.split(|x| *x == '.').enumerate() {
            if length_index >= og_lengths.len() || og_lengths[length_index] as usize != line.len() {
                println!("ERROR!!!!");
                print_info(&0, springs, og_lengths);
                print!("new found string: ");
                for x in pre_truncated_str.iter() {
                    print!("{}", x);
                }
                println!();
            }
        }

        return 1;
    }

    // find next non period
    let i: usize = idx.clone();

    match springs[i] {
        '#' => {
            // we need to match this so lets try and test it as well
            let match_length = lengths[0];

            let mut in_a_row = 0;       
            while i + in_a_row < springs.len() && springs[i + in_a_row] != '.' {
                in_a_row += 1;
            }
            if (in_a_row as i64) < match_length {
                return 0;
            } else if (i + match_length as usize) < springs.len() && springs[i + match_length as usize] == '#'{
                return 0;
            }

            placed.push((i, match_length as usize));
            // make sure this skipping business works (pull first value off lengths too)
            let ret = calculate_line_num_ways(
                &(i + match_length as usize + 1),
                springs,
                &lengths[1..],
                memoi,
                placed,
                og_lengths
            );
            // memoi.insert((idx.clone(), lengths.len()), ret);
            return ret;
        }
        '?' => {
            // if its a question mark, return num ways setting here + num ways not setting here

            // can we set it here - how do we ensure that its possible? right now we are just checking that char -
            // we need to iterate for match length, check how many #, and makle sure after it is not a # then recurse
            let mut in_a_row = 0;
            while i + in_a_row < springs.len()
                && springs[i + in_a_row] != '.'
                && (in_a_row as i64) < lengths[0]
            {
                in_a_row += 1;
            }
            if (in_a_row as i64) >= lengths[0] {
                if i + in_a_row >= springs.len() || springs[i + in_a_row] != '#' {
                    // can the match go here?
                    // it can only go here if yes length matches, and the next char is a '.' or a #

                    // println!("Index: {} can fit the match of length: {} here", i, lengths[0]);
                    // yes we can set it here
                    // println!("Returning num_ways for two paths: idx: {}");
                    // print_info(&(i + lengths[0] as usize + 1), springs, &lengths[1..]);
                    // print_info(&(i + 1), springs, lengths);
                    let c = placed.clone();
                    placed.push((i, lengths[0] as usize));

                    let ret = calculate_line_num_ways(
                        &(i + lengths[0] as usize + 1),
                        springs,
                        &lengths[1..],
                        memoi,
                        placed,
                        og_lengths
                    ) + calculate_line_num_ways(&(i + 1), springs, lengths, memoi, c, og_lengths);

                    // memoi.insert((idx.clone(), lengths.len()), ret);
                    return ret;
                }
            }

            let ret = calculate_line_num_ways(&(i + 1), springs, lengths, memoi, placed, og_lengths);
            // memoi.insert((idx.clone(), lengths.len()), ret);
            return ret;
        }
        _ => {
            return calculate_line_num_ways(&(idx + 1), springs, lengths, memoi, placed, og_lengths);
        }
    }
}

pub fn day12b(input: &Vec<&str>) {
    let mut sum: i64 = 0;
    for (i, line) in input.iter().enumerate() {
        let (springs, lengths) = line.split_once(' ').unwrap();
        let mut memoi: HashMap<(usize, usize), i64> = HashMap::new(); // memoi of idx seen, remaining lengths (unique)

        // convert the expanded springs
        let mut expanded_springs = springs.chars().collect_vec();
        expanded_springs.push('?');
        expanded_springs = expanded_springs.repeat(5); 
        expanded_springs.remove(expanded_springs.len() - 1);

        // convert the 
        let mut expanded_lengths = 
        lengths.split(',').map(|x| x.parse::<i64>().unwrap()).collect_vec();
        expanded_lengths = expanded_lengths.repeat(5);


        let t = calculate_line_num_ways(
            &0,
            &expanded_springs,
            &expanded_lengths[..],
            &mut memoi,
            Vec::<(usize, usize)>::new(),
            &expanded_lengths[..]
        ) as i64;
        // if t  == 0 {
        //     println!("0: {}", line);
        // }
        // println!("Line: {} calculated: {}", line, t);
        sum += t;
        // for ((idx, lengths), calc) in memoi.iter() {
        //     println!("idx: {} - length left: {}, calculated: {}", idx, lengths, calc);
        // }
        
        if i % 10 == 0 {
            println!("Processed another 10: {}", i);
        }
    }

    println!("Answer: {}", sum);
}
