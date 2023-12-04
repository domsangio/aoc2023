
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

// returns start index
fn get_left(index: usize, row: &&str) -> usize {
    let mut i: usize = index;

    while i > 0 {
        if row.chars().nth(i - 1).unwrap().is_digit(10) {
            i -= 1;
        }
    }

    return i;
}
fn get_right(index: usize, row: &&str) -> usize {
    let mut i: usize = index;

    while i + 1 < row.len() {
        if row.chars().nth(i+1).unwrap().is_digit(10) {
            i += 1;
        }
    }

    return i;
}

// need to make suree that if its split it still works, hard code this?
fn check_other_row(index: usize, row: &&str) -> Vec<usize> {
    let mut ret: Vec<usize> = Vec::new();

    // if the one right above is not a digit can reuse method getright leeft
    if !row.chars().nth(index).unwrap().is_digit(10) {
        let left = get_left(index, row);
        let right = get_right(index, row);
        if left != index {
            ret.push(left);
        }
        if right != index {
            ret.push(right);
        }
        return ret;
    }

    // find a digit and then the rest of it (start frmo right)
    if index + 1 < row.len() {
        if row.chars().nth(index + 1).unwrap().is_digit(10) {
            ret.push(get_left(index + 1, row));
            return ret;
        }
    }
    if row.chars().nth(index).unwrap().is_digit(10) {
        ret.push(get_left(index, row));
        return ret;
    }
    if index > 0 {
        if row.chars().nth(index - 1).unwrap().is_digit(10) {
            ret.push(get_left(index - 1, row));
            return ret;
        }
    }

    ret
}

pub fn day3b(input: &Vec<&str>) {
    let mut part_sum: u32 = 0;

    for (row_index, row) in input.iter().enumerate() {
        for (i, c) in row.char_indices() {
            if c == '*' {
                // check adjacencies
                let mut indices: Vec<usize> = Vec::new();

                if get_left(i, row) != i {
                    indices.push(get_left(i, row));
                }
                if get_right(i, row) != i {
                    indices.push(get_right(i, row));
                }
                
                if row_index > 0 {
                    let above = check_other_row(i, &input[row_index - 1]);
                    for x in above {
                        indices.push(x);
                    }
                }

                if row_index + 1< input.len() {
                    for x in check_other_row(i, input[row_index + 1]) {
                        indices.push(x);
                    }
                }

                if indices.len() == 2 {
                    
                }

            }
        }

    }

    println!("{}", part_sum);
}