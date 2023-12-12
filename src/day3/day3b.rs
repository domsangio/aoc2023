// TODO only use three rows at once, and pad left right up down with '.' so we can avoid bounds checks

use std::collections::HashMap;

use itertools::Itertools;

fn add_to_gear_map(gear_map: &mut HashMap<(usize, usize), Vec<u32>>, gear: (usize, usize), num: u32) {
    if gear_map.contains_key(&gear) {
        gear_map.get_mut(&gear).unwrap().push(num);
    } else {
        gear_map.insert(gear, vec![num]);
    }
}

fn convert_input(input: &Vec<&str>) -> Vec<Vec<char>> {
    [
        vec![std::iter::repeat('.').take(input[0].len() + 2).collect_vec()],
        input.iter().map(|line| [
            vec!['.'],
            line.chars().collect_vec(),
            vec!['.']
        ].concat()).collect_vec(),
        vec![std::iter::repeat('.').take(input[0].len() + 2).collect_vec()],
    ].concat()
}

fn calculate_gear_ratio(gear_map: &HashMap<(usize, usize), Vec<u32>>) -> i64 {
    gear_map.values().fold(0, |acc, x| {
        if x.len() == 2 {
            return acc + (x[0] as i64) * (x[1] as i64);
        }
        acc
    })
}

pub fn day3b(input: &Vec<&str>) {
    let clean_grid = convert_input(input);

    for line in clean_grid.iter() {
        println!("Line: {}", line.clone().into_iter().collect::<String>());
    }

    // find a number
    // iterate boundnary find gear
    // add to gear map (gear location -> vec of nums number 1, number 2)
        // if two gears remove it
    // loop thru gear map and sum if exxactly 2

    let mut gear_map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    
    let mut x = 1;
    while x < clean_grid.len() - 1 {
        let mut y = 1;
        while y < clean_grid[x].len() {
            if let Some(first_digit) = clean_grid[x][y].to_digit(10) {
                let mut j: usize = 1;  // end of current_number
                let mut current_number = first_digit;
                // println!("Checking {},{}", x, y);

                while let Some(next_num) = clean_grid[x][y + j].to_digit(10) {
                    j += 1;
                    current_number = current_number * 10 + next_num;
                }

                // iterate thru bounds for a gear
                // top line
                for top_bound in (y-1)..(y+j+1) {
                    if clean_grid[x - 1][top_bound] == '*' {
                        // println!("Top gear found here: {}-{}, curr num: {}", x - 1, top_bound, current_number);
                        add_to_gear_map(&mut gear_map, (x - 1, top_bound), current_number);
                    }
                }

                // left
                if clean_grid[x][y - 1] == '*' {
                    // println!("Left gear found here: {}-{}, curr num: {}", x, y - 1, current_number);
                    add_to_gear_map(&mut gear_map, (x, y - 1), current_number);
                }

                // right
                if clean_grid[x][y + j] == '*' {
                    // println!("Right gear found here: {}-{}, curr num: {}", x, y + j, current_number);
                    add_to_gear_map(&mut gear_map, (x, y + j), current_number);
                }

                // bottom line
                for bottom_bound in (y-1)..(y+j+1) {
                    if clean_grid[x + 1][bottom_bound] == '*' {
                        // println!("Bottom gear found here: {}-{}, curr num: {}", x + 1, bottom_bound, current_number);
                        add_to_gear_map(&mut gear_map, (x + 1, bottom_bound), current_number);
                    }
                }
                // println!("Increment y: {} by {}", y, j + 1);
                y += j + 1;     // can go one further since we know non digit followss
                continue;
            }
            y += 1;
        }

        x += 1;
    }

    // for ((x,y), v) in gear_map.iter() {
    //     println!("Gear found at: row: {}, col: {} with len: {}", x, y, v.len());
    // }

    println!("{}", calculate_gear_ratio(&gear_map));
}