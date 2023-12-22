use regex::Regex;

// i think can do the 17^length of word % n at the end
fn compute_hash(word: &str) -> u8 {
    let ret = word
        .chars()
        .fold(0, |acc: u16, c: char| ((acc + c as u16) * 17) % 256);
    // println!("Word: {} calculated as {}", word, ret);
    ret as u8
}

pub fn day15a(input: &Vec<&str>) {
    // the input is one long line
    let answer: u16 = input
        .iter()
        .next()
        .unwrap()
        .split(",")
        .map(|word| compute_hash(word) as u16)
        .sum();
    println!("Answer: {}", answer);
}

// let game_id_pattern = Regex::new(r"^Game (\d+):(.*)$").unwrap();
// if let Some(captures) = game_id_pattern.captures(line) {
//     *game_id = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
//     return captures.get(2).unwrap().as_str().to_owned();

struct Lens {
    label: String,
    focal_length: u8,
}

struct Box {
    lens: Vec<Lens>,
}

// remove the label from this box
fn remove(curr_box: &mut Box, label: &str) {
    for (i, curr_lens) in curr_box.lens.iter().enumerate() {
        if curr_lens.label == label {
            curr_box.lens.remove(i);
            break;
        }
    }
}

// insert the lens
fn insert(curr_box: &mut Box, lens: Lens) {
    // check if its in the box
    for curr_lens in curr_box.lens.iter_mut() {
        if curr_lens.label == lens.label {
            curr_lens.focal_length = lens.focal_length;
            return;
        }
    }

    curr_box.lens.push(lens);
}

fn calculate_box(b: &Box) -> i64 {
    b.lens.iter().enumerate().fold(0, |acc, (i, l)| {
        acc + ((i + 1) as i64) * (l.focal_length as i64)
    })
}

fn calculate_total_focal_power(lens: &Vec<Box>) -> i64 {
    lens.iter()
        .enumerate()
        .fold(0, |acc, (i, b)| acc + (i as i64 + 1) * calculate_box(b))
}

pub fn day15b(input: &Vec<&str>) {
    let mut lens: Vec<Box> = Vec::with_capacity(256);
    for _ in 0..256 {
        lens.push(Box { lens: Vec::new() });
    }

    let line_match = Regex::new(r"^([a-z]+)([-=])([0-9])?$").unwrap();

    for word in input.iter().next().unwrap().split(",") {
        if let Some(captures) = line_match.captures(word) {
            // get the hash from capture group 1
            let label = captures.get(1).unwrap().as_str().to_owned();
            let hash = compute_hash(&label);

            match captures.get(2).unwrap().as_str() {
                "=" => {
                    insert(
                        &mut lens[hash as usize],
                        Lens {
                            label,
                            focal_length: captures.get(3).unwrap().as_str().parse::<u8>().unwrap(),
                        },
                    );
                }
                _ => {
                    // '-'
                    remove(&mut lens[hash as usize], label.as_str());
                }
            }
        }
    }

    // calculate focal power now
    println!("total focal power: {}", calculate_total_focal_power(&lens));
}
