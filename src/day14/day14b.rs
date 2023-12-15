use std::collections::{HashSet, HashMap};

use itertools::Itertools;


// we might need memoi

// calculates the northern support
fn calculate_northern_support(grid: &Vec<Vec<char>>) -> i64 {
    let mut sum: i64 = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'O' {
                sum += (grid.len() - row) as i64;     // perrhaps +- 1
            }
        }
    }

    sum
}

fn slide_vec(mut v: Vec<char>) -> Vec<char> {
    let mut curr_position = 0;
    let mut last_open_position = 0;

    while curr_position < v.len() {
        match v[curr_position] {
            'O' => {
                if last_open_position != curr_position {
                    v[last_open_position] = 'O';
                    v[curr_position] = '.';
                }
                last_open_position += 1;
                curr_position += 1;
            },
            '#' => {
                // skip ahead
                curr_position += 1;
                last_open_position = curr_position;
            },
            _ => {
                curr_position += 1;
            }
        } 
    }

    v
}

fn slide_column_up(grid: &mut Vec<Vec<char>>, column: usize) {
    let v = slide_vec(grid.iter().map(|row| row[column]).collect_vec());
    for row in 0..grid.len() {
        grid[row][column] = v[row];
    }
}

fn slide_column_down(grid: &mut Vec<Vec<char>>, column: usize) {
    let v = slide_vec(grid.iter().rev().map(|row| row[column]).collect_vec());
    for row in 0..grid.len() {
        grid[row][column] = v[v.len() - 1 - row];
    }
}

fn slide_row_left(grid: &mut Vec<Vec<char>>, row: usize) {
    let v = slide_vec(grid[row].clone());
    grid[row] = v
}

fn slide_row_right(grid: &mut Vec<Vec<char>>, row: usize) {
    let mut v = slide_vec(grid[row].clone().into_iter().rev().collect_vec());
    v.reverse();
    grid[row] = v;
}

fn perform_cycle(grid: &mut Vec<Vec<char>>) {
    for col in 0..grid[0].len() {
        slide_column_up(grid, col);
    }

    for row in 0..grid.len() {
        slide_row_left(grid, row);
    }

    for column in 0..grid[0].len() {
        slide_column_down(grid, column);
    }

    for row in 0..grid.len() {
        slide_row_right(grid, row);
    }
}

fn get_config(grid: &Vec<Vec<char>>) -> i64 {
    let mut config: i64 = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            match grid[row][col] {
                'O' => config += 2 * (row * grid[row].len()) as i64 + col as i64,
                '#' => config += 1 * (row * grid[row].len()) as i64 + col as i64,
                _ => config += 0
            }
        }
    }

    config
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in 0..grid.len() {
        for c in grid[row].iter() {
            print!("{}", c);
        }
        println!("");
    }
}


pub fn day14b(input: &Vec<&str>) {
    let mut grid: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect_vec()).collect_vec();
    let mut support_to_config_to_iteration: HashMap<i64, HashMap<i64, i64>> = HashMap::new();
    
    // for i in 0..1000 {
    for i in 0..1000000000 {
        perform_cycle(&mut grid);
        if i == 2 {
            print_grid(&grid);
        }
        println!("iteration: {}, support: {}", i, calculate_northern_support(&grid));
        
        let support = calculate_northern_support(&grid);
        
        let mut seen_before = -1;

        // if weve seen it before, 
        match support_to_config_to_iteration.get_mut(&support) {
            Some(config_to_iteration) => {
                println!("Seen this support before");
                match config_to_iteration.get(&get_config(&grid)) {
                    Some(iteration) => {
                        println!("Seen it at iteration: {}, curr it: {}", iteration, i);
                        seen_before = *iteration;
                        print_grid(&grid);
                    },
                    None => {
                        let _ = config_to_iteration.insert(get_config(&grid), i);
                    }
                }
            },
            None => {
                support_to_config_to_iteration.insert(support, HashMap::from([
                (get_config(&grid), i as i64)
            ]));
                }
        }

        if seen_before != -1 {
            // cycle length = (i - iteration)
            let to_go = (1000000000 - i - 1) % (i - seen_before);
                    
            for _ in 0..to_go {
                perform_cycle(&mut grid);
            }

            break;
        }
    }

    // for x in 0..grid.len() {
    //     for y in grid[x].iter() {
    //         print!("{}", y);
    //     }
    //     println!("");
    // }

    println!("Answer: {}", calculate_northern_support(&grid));
}