use std::vec;
use std::collections::HashSet;
use itertools::Itertools;
 
// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

pub fn day10a(input: &Vec<&str>) {
    // create grid as a 2d array of chars padded as well
    let mut grid: Vec<Vec<char>> = Vec::new();
    grid.push(Vec::from_iter(std::iter::repeat('.').take(grid[0].len())));
    grid.append(&mut input.iter().map(|line| {
        let mut v = vec![','];
        v.append(&mut line.chars().collect_vec());
        v.push('.');
        v
    }).collect_vec());
    grid.push(Vec::from_iter(std::iter::repeat('.').take(grid[0].len())));

    let mut seen = HashSet::<(usize, usize)>::new();
    for i in 1..(grid.len() - 1) {
        for j in 1..(grid[i].len() - 1) {
            // let mut viewed_set: HashSet

            if grid[i][j] == 'S' {
                // find the cycle
            }
        }
    }
}

fn get_next_spot(c: char, position: (usize, usize), came_from: (usize, usize)) -> (usize, usize) {
    let mut pipe_start = (0,0);
    let mut pipe_end = (0, 0); 

    match c {
        '|' => {
            pipe_start = (position.0, position.1 - 1);
            pipe_end = (position.0, position.1 + 1);
        },
        '-' => {
            pipe_start = (position.0 - 1, position.1);
            pipe_end = (position.0 + 1, position.1);
        },
        'L' => {
            pipe_start = (position.0 - 1, position.1);
            pipe_end = (position.0, position.1 + 1);
        },
        'J' => {
            pipe_start = (position.0 - 1, position.1);
            pipe_end = (position.0, position.1 - 1);
        },
        '7' => {
            pipe_start = (position.0, position.1 - 1);
            pipe_end = (position.0 + 1, position.1);
        },
        'F' => {
            pipe_start = (position.0, position.1 + 1);
            pipe_end = (position.0 + 1, position.1);
        },
        _ => ()
    }
    
    if pipe_end == came_from {
        return pipe_start;
    }
    pipe_end
}

// // idek if we need this ffunction
// fn get_adjacents(c: char, i, j) -> [(usize, usize); 2] {
//     match c {
//         ""
//     }
// }