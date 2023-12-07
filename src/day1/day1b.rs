const DEBUG: bool = false;

fn match_three_char_word(word: &str) -> u32 {
    match word {
        "one" => 1,
        "two" => 2,
        "six" => 6,
        _ => 0
    }
}

fn match_four_char_word(word: &str) -> u32 {
    match word {
        "four" => 4,
        "five" => 5,
        "nine" => 9,
        "zero" => 0,
        _ => 0
    }
}

fn match_five_char_word(word: &str) -> u32 {
    match word {
        "three" => 3,
        "eight" => 8,
        "seven" => 7,
        _ => 0
    }
}

fn find_first_last_word(line: &str) -> u32 {
    if DEBUG {println!("Line: {}", line);}

    let mut first: u32 = 0;
    let mut last: u32 = 0;
    let mut chars = line.chars();

    for i in 0..line.len() {
        let c = chars.next().unwrap();
        if c.is_digit(10) {
            first = c.to_digit(10).unwrap();
            if DEBUG {println!("First: {}, {}", i, first);}
            break;
        }
        // match 3 letter words if we can
        if i + 3 <= line.len() {
            first = match_three_char_word(&line[i..(i + 3)]);
            if first != 0 {
                if DEBUG {println!("First: {}, {}", i, &line[i..(i + 3)]);}
                break;
            }
        } 
        if i + 4 <= line.len() {
            first = match_four_char_word(&line[i..(i + 4)]);
            if first != 0 {
                if DEBUG {println!("First: {}, {}", i, &line[i..(i + 4)]);}
                break;
            }
        }
        if i + 5 <= line.len() {
            first = match_five_char_word(&line[i..(i + 5)]);
            if first != 0 {
                if DEBUG {println!("First: {}, {}", i, &line[i..(i + 5)]);}
                break;
            }
        }
    }

    // TODO zip the iters or iter with value (for i, x in or whatever)
    let mut chars_rev = line.chars().rev();
    for i in (0..(line.len())).rev() {
        let c = chars_rev.next().unwrap();
        if c.is_digit(10) {
            if DEBUG {
                println!("Last: {}, {}", i, c.to_digit(10).unwrap());
                println!("Return: {}", first * 10 + c.to_digit(10).unwrap());
            }
            return first * 10 + c.to_digit(10).unwrap();
        }
        if i + 3 <= line.len() {
            last = match_three_char_word(&line[i..(i + 3)]);
            if last != 0 {
                if DEBUG {
                    println!("Last: {}, {}", i, &line[i..(i + 3)]);
                    println!("Return: {}", first * 10 + last);
                }
                return first * 10 + last;
            }
        } 
        if i + 4 <= line.len() {
            last = match_four_char_word(&line[i..(i + 4)]);
            if last != 0 {
                if DEBUG {
                    println!("Last: {}, {}", i, &line[i..(i + 4)]);
                    println!("Return: {}", first * 10 + last);
                }
                return first * 10 + last;
            }
        }
        if i + 5 <= line.len() {
            last = match_five_char_word(&line[i..(i + 5)]);
            if last != 0 {
                if DEBUG {
                    println!("Last: {}, {}", i, &line[i..(i + 5)]);
                }
                return first * 10 + last;
            }
        }
    }
    0
}

pub fn day1b(input: &Vec<&str>) {
    println!("{}", input.iter().map(|line| find_first_last_word(line)).sum::<u32>());
}