use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref LIMITS: HashMap<&'static str, u32> = {
        let map = HashMap::from([
            ("red", 12),
            ("green", 13),
            ("blue", 14),
        ]);
        map
    };
}

fn strip_game_id_and_shorten(line: &str, game_id: &mut u32) -> String {
    let game_id_pattern = Regex::new(r"^Game (\d+):(.*)$").unwrap();
    if let Some(captures) = game_id_pattern.captures(line) {
        *game_id = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
        return captures.get(2).unwrap().as_str().to_owned();
    }

    String::new()
}

fn parse_subsets(subsets: &String) -> bool {
    let cube_pattern = Regex::new(r"^ (\d+) ([a-z]+)").unwrap();
    for subset in subsets.split(";") {
        // let mut cube_amount: HashMap<&str, u32> = HashMap::new();
        
        for cubes in subset.split(",") {
            // println!("Cube: {}", cubes);
            if let Some(captures) = cube_pattern.captures(cubes) {
                println!("Limit for this: {}", *LIMITS.get(captures.get(2).unwrap().as_str()).unwrap());
                println!("this line sees: {}", captures.get(1).unwrap().as_str().parse::<u32>().unwrap());
                if *LIMITS.get(captures.get(2).unwrap().as_str()).unwrap() < captures.get(1).unwrap().as_str().parse::<u32>().unwrap() {
                    return true;
                }
            }
        }
    }

    false
}

fn mapper(line: &str) -> u32 {
    // regex capture the game id
    // split remaining on ';'
    let mut game_id: u32 = 0;
    let subsets: String = strip_game_id_and_shorten(line, &mut game_id);
    println!("SUBSETS: {}, game id: {}", subsets, game_id);
    if parse_subsets(&subsets) {
        println!("yes this broke it");
        return 0;
    }
    println!("no it didnt");
    game_id
}

pub fn day2a(input: &Vec<&str>) {
    println!("{}", input.iter().map(|line| mapper(line)).sum::<u32>());
}