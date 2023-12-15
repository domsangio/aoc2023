use std::collections::VecDeque;

use itertools::Itertools;


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

fn slide_column(grid: &mut Vec<Vec<char>>, column: usize) {
    let mut curr_position = 0;
    let mut last_open_position = 0;

    while curr_position < grid.len() {
        match grid[curr_position][column] {
            'O' => {
                if last_open_position != curr_position {
                    grid[last_open_position][column] = 'O';
                    grid[curr_position][column] = '.';
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
}

pub fn day14a(input: &Vec<&str>) {
    let mut grid: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect_vec()).collect_vec();
    
    for col in 0..grid[0].len() {
        slide_column(&mut grid, col);
    }

    println!("Answer: {}", calculate_northern_support(&grid));
}