
//[left, right) --> check [left - 1, right]
fn check_row(left: usize, right: usize, row: &&str) -> bool {
    for i in (if left == 0 { 0 } else { left - 1 })..=right {
        if i < row.len() {
            let c = row.chars().nth(i).unwrap();
            if !c.is_digit(10) && c != '.' {
                return true;
            }
        }
    }
    false
    // return (left - 1 > 0 && !row.chars().nth(left - 1).unwrap().is_digit(10) && row.chars().nth(left - 1).unwrap() != '.') || 
    // (right + 1 < row.len() && !row.chars().nth(right + 1).unwrap().is_digit(10) && row.chars().nth(right + 1).unwrap() != '.');
}

pub fn day3a(input: &Vec<&str>) {
    let mut part_sum: u32 = 0;

    for (row_index, row) in input.iter().enumerate() {
        let mut curr_start = input.len();
        let mut curr_end = input.len();
        let mut i = 0;

        while i < row.len() {
            /* find next digit */
            while i < row.len() && !row.chars().nth(i).unwrap().is_digit(10) {
                i += 1;
            }
            
            /* find last digit */
            curr_start = i;
            curr_end = i;
            
            while i < row.len() && row.chars().nth(i).unwrap().is_digit(10) {
                i += 1;
                curr_end += 1;
            }

            // have numbers from [curr_start, curr_end)
            // check if adjacent on current row
            if check_row(curr_start, curr_end, row) ||
            (row_index > 0 && check_row(curr_start, curr_end, &input[row_index - 1])) || 
            (row_index + 1 < input.len() && check_row(curr_start, curr_end, &input[row_index + 1])) {
                // calculate this number and then add or whatever
                // println!("current row: {}, found curr_start {}, curr_end {}", row, curr_start, curr_end);
                part_sum += input[row_index][curr_start..curr_end].parse::<u32>().unwrap();
            }
        }
    }

    println!("{}", part_sum);
}