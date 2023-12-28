use std::collections::HashMap;

use itertools::Itertools;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}
// TODO i already think were going to need dijkstras here
fn create_heat_loss_grid(input: &Vec<&str>) -> Vec<Vec<char>> {
    input.iter().map(|line| line.chars().collect_vec()).collect_vec()
}

fn recurse(pos: (usize, usize), incoming_direction: Direction, ) {

}


pub fn day17a(input: &Vec<&str>) {
    let heat_loss_grid = create_heat_loss_grid(input);
    let mut memoi: HashMap<(usize, usize, Direction), i64> = HashMap::new(); // position & outgoing direction

    
}


pub fn day17b(input: &Vec<&str>) {

}