use std::collections::HashSet;

pub fn get_card_num(line: &str) -> i64 { 
    line.split_whitespace()
        .skip(1)
        .next()
        .unwrap()
        .parse::<i64>()
        .unwrap()
}

pub fn parse_winning_numbers(line: &str) -> HashSet<i64> {
    HashSet::from_iter(
        line.split_whitespace()
            .map(|x| x.parse::<i64>()
            .unwrap())      
    )
}

pub fn day4a(input: &Vec<&str>) {
    println!("Day 4a: {}", 
    input.iter().fold(0, |acc, line| {
        let (front, back) = line.split_once(":").unwrap();
        let _ = get_card_num(front); // card num doesnt do anything yet

        let (winning_section, nums) = back.split_once("|").unwrap();
        let winning_numbers = parse_winning_numbers(winning_section);

        acc + match nums.split_whitespace().map(|x| {
            
            match winning_numbers.contains(&x.parse::<i64>().unwrap()) {
                true => 1,
                _ => 0
            }
        }).sum() {
            0 => 0,
            x => (2 as i64).pow(x - 1),
            _ => panic!("Errror")
        }
    }));
}