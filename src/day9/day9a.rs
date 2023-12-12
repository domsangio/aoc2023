use itertools::Itertools;

fn convert_input(input: &Vec<&str>) -> Vec<Vec<i64>> {
    input
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn is_all_zeroes(sequence: &Vec<i64>) -> bool {
    sequence.iter().fold(true, |acc, x| acc && *x == 0)
}

fn predict_next_value(sequence: &Vec<i64>) -> i64 {
    let mut sequences = vec![sequence.clone()];

    while let Some(last_line) = sequences.iter().next_back() {
        if is_all_zeroes(last_line) {
            break;
        }

        let mut insert_last_line = Vec::<i64>::new();
        for i in 0..(last_line.len() - 1) {
            insert_last_line.push(last_line[i + 1] - last_line[i]);
        }
        sequences.push(insert_last_line);
    }

    let mut unknowns = std::iter::repeat(0).take(sequences.len()).collect_vec();
    for i in (0..(unknowns.len() - 1)).rev() {
        unknowns[i] = sequences[i].iter().next_back().unwrap() + unknowns[i + 1];
    }
    unknowns[0]
}

pub fn day9a(input: &Vec<&str>) {
    println!("Answer: {}", convert_input(input).iter().map(|line| predict_next_value(&line)).sum::<i64>());
}
