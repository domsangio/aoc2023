use std::collections::HashSet;
use std::fmt::{self, Display};
use std::{thread, time};

use itertools::Itertools;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

impl Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::DOWN => write!(f, "DOWN"),
            Direction::LEFT => write!(f, "LEFT"),
            Direction::RIGHT => write!(f, "RIGHT"),
            Direction::UP => write!(f, "UP"),
        }
    }
}

fn iterate_direction(x: i32, y: i32, direction: Direction) -> (i32, i32) {
    match direction {
        Direction::LEFT => (x, y - 1),
        Direction::RIGHT => (x, y + 1),
        Direction::DOWN => (x + 1, y),
        Direction::UP => (x - 1, y),
    }
}

fn check_in_bounds(x: i32, y: i32, max_rows: i32, max_cols: i32) -> bool {
    x >= 0 && x < max_rows && y >= 0 && y < max_cols
}

// We could technically like memoi this instead of just brute forcing
// we need to change this functino to rreturn a value, and then memoi it so change returrn
// and change seen. meantime lets just trry and brute force it?

fn beam(
    grid: &Vec<Vec<char>>,
    charged_grid: &mut Vec<Vec<u8>>,
    starting_pos: (i32, i32),
    direction: Direction,
    seen: &mut HashSet<(i32, i32, Direction)>, // TODO hash map to dirrection
) {
    let (x, y) = starting_pos;

    // let ten_millis = time::Duration::from_millis(10);
    // thread::sleep(ten_millis);

    // kill the recursion
    if !check_in_bounds(
        x,
        y,
        charged_grid.len() as i32,
        charged_grid[0].len() as i32,
    ) || seen.contains(&(x, y, direction))
    {
        return;
    }

    // println!(
    //     "Here: ({}, {}) going dir: {}, char: {}",
    //     x, y, direction, grid[x as usize][y as usize]
    // );

    charged_grid[x as usize][y as usize] += 1;
    seen.insert((x, y, direction));

    match (grid[x as usize][y as usize], direction) {
        // if we keep on going
        ('-', Direction::RIGHT)
        | ('-', Direction::LEFT)
        | ('|', Direction::DOWN)
        | ('|', Direction::UP)
        | ('.', _) => beam(
            grid,
            charged_grid,
            iterate_direction(x, y, direction),
            direction,
            seen,
        ),

        // if were recursing left and right
        ('-', Direction::UP) | ('-', Direction::DOWN) => {
            beam(
                grid,
                charged_grid,
                iterate_direction(x, y, Direction::RIGHT),
                Direction::RIGHT,
                seen,
            );
            beam(
                grid,
                charged_grid,
                iterate_direction(x, y, Direction::LEFT),
                Direction::LEFT,
                seen,
            );
        }

        // if were recursing up and down
        ('|', Direction::LEFT) | ('|', Direction::RIGHT) => {
            beam(
                grid,
                charged_grid,
                iterate_direction(x, y, Direction::DOWN),
                Direction::DOWN,
                seen,
            );
            beam(
                grid,
                charged_grid,
                iterate_direction(x, y, Direction::UP),
                Direction::UP,
                seen,
            );
        }

        // if were changing at a 90 degree
        ('\\', Direction::UP) | ('/', Direction::DOWN) => {
            beam(
                grid,
                charged_grid,
                iterate_direction(x, y, Direction::LEFT),
                Direction::LEFT,
                seen,
            );
        }

        ('\\', Direction::RIGHT) | ('/', Direction::LEFT) => {
            beam(
                grid,
                charged_grid,
                iterate_direction(x, y, Direction::DOWN),
                Direction::DOWN,
                seen,
            );
        }
        ('\\', Direction::DOWN) | ('/', Direction::UP) => {
            beam(
                grid,
                charged_grid,
                iterate_direction(x, y, Direction::RIGHT),
                Direction::RIGHT,
                seen,
            );
        }
        ('\\', Direction::LEFT) | ('/', Direction::RIGHT) => {
            beam(
                grid,
                charged_grid,
                iterate_direction(x, y, Direction::UP),
                Direction::UP,
                seen,
            );
        }
        _ => panic!("why"),
    }
}

fn calculate_charged(charged_grid: &Vec<Vec<u8>>) -> i64 {
    charged_grid
        .iter()
        .map(|line| line.iter().fold(0, |acc, x| acc + (
            match *x {
                0 => 0,
                _ => 1
            } as i64)))
        .sum()
}

pub fn day16a(input: &Vec<&str>) {
    let grid = input
        .iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let mut charged: Vec<Vec<u8>> = Vec::new();

    for _ in 0..input.len() {
        charged.push(
            std::iter::repeat(0 as u8)
                .take(input[0].len())
                .collect_vec(),
        );
    }

    let mut seen: HashSet<(i32, i32, Direction)> = HashSet::new();

    beam(&grid, &mut charged, (0, 0), Direction::RIGHT, &mut seen);

    println!("Answer: {}", calculate_charged(&charged));
}

pub fn day16b(input: &Vec<&str>) {
    let grid = input
    .iter()
    .map(|line| line.chars().collect_vec())
    .collect_vec();

    let mut max: i64 = 0;
    
    for row in 0..grid.len() {   
        println!("Row {} going right", row);
        let mut charged: Vec<Vec<u8>> = Vec::new();

        for _ in 0..input.len() {
            charged.push(
                std::iter::repeat(0 as u8)
                    .take(input[0].len())
                    .collect_vec(),
            );
        }

        let mut seen: HashSet<(i32, i32, Direction)> = HashSet::new();

        beam(&grid, &mut charged, (row as i32, 0), Direction::RIGHT, &mut seen);

        if calculate_charged(&charged) > max {
            max = calculate_charged(&charged);
        }
    }
    for row in 0..grid.len() {   
        println!("Row {} going left", row);

        let mut charged: Vec<Vec<u8>> = Vec::new();

        for _ in 0..input.len() {
            charged.push(
                std::iter::repeat(0 as u8)
                    .take(input[0].len())
                    .collect_vec(),
            );
        }

        let mut seen: HashSet<(i32, i32, Direction)> = HashSet::new();

        beam(&grid, &mut charged, (row as i32, grid[0].len() as i32 - 1), Direction::LEFT, &mut seen);

        if calculate_charged(&charged) > max {
            max = calculate_charged(&charged);
        }
    }



    for col in 0..grid[0].len() {   
        println!("Col {} going down", col);
        let mut charged: Vec<Vec<u8>> = Vec::new();

        for _ in 0..input.len() {
            charged.push(
                std::iter::repeat(0 as u8)
                    .take(input[0].len())
                    .collect_vec(),
            );
        }

        let mut seen: HashSet<(i32, i32, Direction)> = HashSet::new();

        beam(&grid, &mut charged, (0, col as i32), Direction::DOWN, &mut seen);

        if calculate_charged(&charged) > max {
            max = calculate_charged(&charged);
        }
    }
    for col in 0..grid.len() {   
        println!("Col {} going down", col);

        let mut charged: Vec<Vec<u8>> = Vec::new();

        for _ in 0..input.len() {
            charged.push(
                std::iter::repeat(0 as u8)
                    .take(input[0].len())
                    .collect_vec(),
            );
        }

        let mut seen: HashSet<(i32, i32, Direction)> = HashSet::new();

        beam(&grid, &mut charged, (grid.len() as i32 - 1, col as i32), Direction::UP, &mut seen);

        if calculate_charged(&charged) > max {
            max = calculate_charged(&charged);
        }
    }


    println!("Answer: {}", max);
}
