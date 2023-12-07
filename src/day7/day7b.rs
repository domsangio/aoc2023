use std::{collections::HashMap, cmp::Ordering};

enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

fn convert_hand_type(hand_type: &HandType) -> i8 {
    match *hand_type {
        HandType::FiveOfAKind => 6,
        HandType::FourOfAKind => 5,
        HandType::FullHouse => 4,
        HandType::ThreeOfAKind => 3,
        HandType::TwoPair => 2,
        HandType::OnePair => 1,
        HandType::HighCard => 0
    }
}

fn convert_card(c: &char) -> i8 {
    match *c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        // 'J' => 11,//
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 1 // Joker
    }
}


// TODO compare hand types

struct Hand<'a> {
    bid: i64,
    hand: &'a str,
    hand_type: HandType
}

fn get_hand_type(unsorted_hand: &str) -> HandType {
    let mut occurances: HashMap<char, i8> = HashMap::new();

    let mut joker = 0;
    for c in unsorted_hand.chars() {
        if c == 'J' {
            joker += 1;
        } else  {
            println!("Should be incrementing char: {} by 1", c);
            *occurances.entry(c).or_default() += 1;
        }
    }
    if joker > 0 {
        if joker == 5 {
            return HandType::FiveOfAKind;
        }

        // sub the highest occurred, then sub the highest card
        let max_occurred = *occurances.values().max().unwrap();
        let mut max_card = None;
        for (c, o) in occurances.iter() {
            println!("c: {}, o: {}", c, o);
            if max_occurred == *o && max_card == None {
                println!("Setting max_card here, it was none");
                max_card = Some((*c).clone());
            } else if max_occurred == *o && convert_card(&c) >= convert_card(&max_card.unwrap()) {
                println!("Cahnging value of max card here");
                max_card = Some((*c).clone());
            }
        }
        
        println!("Hand: {}, found max occurred this amt of times: {}, so subbing in {}", unsorted_hand, max_occurred, max_card.unwrap());
        *occurances.entry(max_card.unwrap()).or_default() += joker;
    }

    match occurances.keys().len() {
        1 => HandType::FiveOfAKind,
        2 => match *occurances.values().next().unwrap() {
                1 | 4 => HandType::FourOfAKind,
                _ => HandType::FullHouse
            },
        3 => {
            let mut num_singles: i8 = 0;
            for v in occurances.values() {
                if *v == 3 {
                    return HandType::ThreeOfAKind;
                } else if *v == 1 {
                    num_singles += 1;
                }
            }

            match num_singles {
                1 => HandType::TwoPair,
                _ => HandType::OnePair
            }
        },
        4 => HandType::OnePair,
        _ => HandType::HighCard
    }
}

fn compare_hands(a: &Hand, b: &Hand) -> Ordering {
    match convert_hand_type(&a.hand_type) - convert_hand_type(&b.hand_type) {
        0 => {
            // println!("Same hand type");
            for (c1, c2) in a.hand.chars().zip(b.hand.chars()) {
                if convert_card(&c1) > convert_card(&c2) {
                    // println!("c1: {}-{} bigger than c2: {}-{}", c1, c2, convert_card(&c1), convert_card(&c2));

                    return Ordering::Greater;
                } else if convert_card(&c1) < convert_card(&c2) {
                    return Ordering::Less;
                }
            }
            return Ordering::Equal;
        },
        d if d > 0 => Ordering::Greater,
        _ => Ordering::Less 
    }
}


pub fn day7b(input: &Vec<&str>) {
    let mut sorted_hands: Vec<Hand> = Vec::new();
    for line in input.into_iter() {
        let (hand, bid) = line.split_once(" ").unwrap();
        sorted_hands.push(Hand { bid:bid.parse::<i64>().unwrap(), hand: hand, hand_type: get_hand_type(hand) })
    }

    sorted_hands.sort_by(|x,y| compare_hands(x, y));

    let mut sum: i64 = 0;
    for (i, v) in sorted_hands.iter().enumerate() {
        println!("Hand: {}, rank: {}, hand_type: {}", v.hand, i + 1, convert_hand_type(&v.hand_type));
        sum += v.bid * (i as i64 + 1);
    }

    println!("Annswer: {}", sum);
}