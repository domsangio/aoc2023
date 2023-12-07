use regex::Regex;
use std::collections::HashMap;

fn strip_game_id_and_shorten(line: &str, game_id: &mut u32) -> String {
    let game_id_pattern = Regex::new(r"^Game (\d+):(.*)$").unwrap();
    if let Some(captures) = game_id_pattern.captures(line) {
        *game_id = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
        return captures.get(2).unwrap().as_str().to_owned();
    }

    String::new()
}

fn parse_subsets(subsets: &String) -> u32 {
    // println!("line: {}", subsets);
    let cube_pattern = Regex::new(r"^ (\d+) ([a-z]+)").unwrap();
    let mut min_cube_amount: HashMap<&str, u32> = HashMap::new();

    for subset in subsets.split(";") {
        // println!("Subset: '{}'", subset);
        for cubes in subset.split(",") {
            if let Some(captures) = cube_pattern.captures(cubes) {
                // println!("Current cube: '{}'", cubes);

                let color: &str = captures.get(2).unwrap().as_str();
                let num: u32 = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
                
                if !min_cube_amount.contains_key(color) {
                    min_cube_amount.insert(color, num);
                } else {
                    println!("Current color value: {}, num: {}", *min_cube_amount.get(color).unwrap(), num);
                    if *min_cube_amount.get(color).unwrap() < num {
                        min_cube_amount.insert(color, num);
                    }
                }
            }
        }

        // for (&k,&v) in min_cube_amount.iter() {
        //     println!("Color: {}, min needed: {}", k, v);
        // }

    }
    return min_cube_amount.values().fold(1, |acc, x| acc * x);
}

fn mapper(line: &str) -> u32 {
    // regex capture the game id
    // split remaining on ';'
    let mut game_id: u32 = 0;
    let subsets: String = strip_game_id_and_shorten(line, &mut game_id);
    parse_subsets(&subsets)
}

pub fn day2b(input: &Vec<&str>) {
    println!("{}", input.iter().map(|line| mapper(line)).sum::<u32>());
}