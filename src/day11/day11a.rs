use core::num;
use std::collections::HashSet;
use std::collections::HashMap;

use itertools::Itertools;

fn find_expanded_rows(grid: &Vec<Vec<char>>, expanded_rows: &mut HashSet<usize>) {
    for row in 0..grid.len() {
        let mut expand = true;
        for col in 0..grid[row].len() {
            if grid[row][col] != '.' {
                expand = false;
                break;
            } 
        }
        if expand {
            expanded_rows.insert(row);
        }
    }
}

fn find_expanded_cols(grid: &Vec<Vec<char>>, expanded_cols: &mut HashSet<usize>) {
    for col in 0..grid[0].len() {
        let mut expand = true;
        for row in 0..grid.len() {
            if grid[row][col] != '.' {
                expand = false;
                break;
            } 
        }
        if expand {
            expanded_cols.insert(col);
        }
    }
}

fn calculate_shortest_path(galaxy_a: usize, galaxy_b: usize, galaxy_map: &Vec<(usize, usize)>, expanded_cols: &HashSet<usize>, expanded_rows: &HashSet<usize>) -> i64 {
    let galaxy_a_pos = galaxy_map[galaxy_a];
    let galaxy_b_pos = galaxy_map[galaxy_b];


    let left_bound = std::cmp::min(galaxy_a_pos.0, galaxy_b_pos.0);
    let right_bound = std::cmp::max(galaxy_a_pos.0, galaxy_b_pos.0);
    let mut row_distance = right_bound - left_bound;
    for r in left_bound..right_bound {
        if expanded_rows.contains(&r) {
            row_distance += 1000000 - 1;
        }
    }



    let left_bound = std::cmp::min(galaxy_a_pos.1, galaxy_b_pos.1);
    let right_bound = std::cmp::max(galaxy_a_pos.1, galaxy_b_pos.1);
    let mut col_distance = right_bound - left_bound;

    for r in left_bound..right_bound {
        if expanded_cols.contains(&r) {
            col_distance += 1000000 - 1;
        }
    }

    (row_distance + col_distance) as i64
}


pub fn day11a(input: &Vec<&str>) {
    let mut expanded_rows: HashSet<usize> = HashSet::new();
    let mut expanded_columns: HashSet<usize> = HashSet::new();
    let mut galaxy_map: Vec<(usize, usize)> = vec![];

    // convert grid
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.iter() {
        grid.push(line.chars().collect_vec());
    }

    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] == '#' {
                galaxy_map.push((x, y));
            }
        }
    }

    // find expanded rows and columns
    find_expanded_cols(&grid, &mut expanded_columns);
    find_expanded_rows(&grid, &mut expanded_rows);

    // for every galaxy pair calculate the cost
    let num_galaxies = galaxy_map.len();

    let mut sum: i64 = 0;
    for i in 0..(num_galaxies - 1) {
        for j in (i+1)..num_galaxies {
            sum += calculate_shortest_path(i, j, &galaxy_map, &expanded_columns, &expanded_rows);
        }
    }

    println!("Answer: {}", sum);
}