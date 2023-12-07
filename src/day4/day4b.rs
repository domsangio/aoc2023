use crate::day4::day4a::{get_card_num, parse_winning_numbers}; 
use std::collections::{HashMap, HashSet};


pub fn day4b(input: &Vec<&str>) {
    let mut accumulated_cards: HashMap<i64, i64> = HashMap::new();

    let answer = input.iter().fold(0, |acc, line| {
        let (front, back) = line.split_once(":").unwrap();
        let card_num = get_card_num(front);
        *accumulated_cards.entry(card_num).or_default() += 1; // add one to current card

        let (winning_section, nums) = back.split_once("|").unwrap();
        let winning_numbers: HashSet<i64> = parse_winning_numbers(winning_section);

        /// sets 1 to this value as well
        let num_matched = nums.split_whitespace().map(|x| {
            match winning_numbers.contains(&x.parse::<i64>().unwrap()) {
                true => 1,
                _ => 0
            }
        }).sum::<i64>();

        let num_this_card = accumulated_cards.get(&card_num).unwrap().clone();

        // add copies to future ones
        for copy in 1..=num_matched {
            *accumulated_cards.entry(card_num + copy).or_default() += num_this_card;
        }

        // println!("Card: {} matched {} numbers, accumlated value from other calls: {}", card_num, num_matched, accumulated_cards.get(&card_num).unwrap());
        // for (k, v) in accumulated_cards.iter() {
        //     println!("Card: {}, has: {}", k, v);
        // }


        // acc + match {
        //     0 => 0,
        //     x => (2 as i64).pow(x - 1),
        //     _ => panic!("Errror")
        // }

        acc + accumulated_cards.get(&card_num).unwrap()
    });

    println!("Day 4 b: {}", answer);
}
