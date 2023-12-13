use std::collections::HashMap;

use itertools::Itertools;

fn calculate_line_num_ways(
    idx: &usize,
    springs: &Vec<char>,
    lengths: &[i64],
    memoi: &mut HashMap<(usize, usize), i64>
) -> i64 {
    // print_info(idx, springs, lengths);

    if memoi.contains_key(&(*idx, lengths.len())) {
        return memoi.get(&(*idx, lengths.len())).unwrap().clone();
    }
    if *idx >= springs.len() {
        if lengths.len() != 0 {
            memoi.insert((idx.clone(), lengths.len()), 0);
            return 0;
        }
    }

    if lengths.len() == 0 {
        // make suire no more '#' in the list
        for i in (*idx)..springs.len() {
            if springs[i] == '#' {
                memoi.insert((idx.clone(), lengths.len()), 0);
               return 0;
            }
        }

        memoi.insert((idx.clone(), lengths.len()), 1);
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
                memoi.insert((idx.clone(), lengths.len()), 0);
                return 0;
            } else if (i + match_length as usize) < springs.len() && springs[i + match_length as usize] == '#'{
                memoi.insert((idx.clone(), lengths.len()), 0);
                return 0;
            }

            // make sure this skipping business works (pull first value off lengths too)
            let ret = calculate_line_num_ways(
                &(i + match_length as usize + 1),
                springs,
                &lengths[1..],
                memoi
            );
            memoi.insert((idx.clone(), lengths.len()), ret);
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
                    let ret = calculate_line_num_ways(
                        &(i + lengths[0] as usize + 1),
                        springs,
                        &lengths[1..],
                        memoi
                    ) + calculate_line_num_ways(&(i + 1), springs, lengths, memoi);

                    memoi.insert((idx.clone(), lengths.len()), ret);
                    return ret;
                }
            }

            let ret = calculate_line_num_ways(&(i + 1), springs, lengths, memoi);
            memoi.insert((idx.clone(), lengths.len()), ret);
            return ret;
        }
        _ => {
            let ret = calculate_line_num_ways(&(idx + 1), springs, lengths, memoi);
            memoi.insert((idx.clone(), lengths.len()), ret);
            return ret;
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
            &mut memoi
        ) as i64;

        sum += t;

        
        if i % 10 == 0 {
            println!("Processed another 10: {}", i);
        }
    }

    println!("Answer: {}", sum);
}
