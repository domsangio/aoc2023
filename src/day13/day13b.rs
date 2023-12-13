use core::num;

use itertools::Itertools;

pub fn day13b(input: &Vec<&str>) {
    let mut row_start = 0;
    let mut row_end = 0;

    let cleaned_input = input.iter().map(|line| line.chars().collect_vec()).collect_vec();
    let mut sum: i64 = 0;

    while row_start < input.len() {
        
        row_end = row_start;
        while row_end < input.len() && input[row_end] != "" {
            row_end += 1;
        }
        println!("Row start: {},  row end: {}", row_start, row_end);
        for line_index in row_start..row_end {
            println!("{}", input[line_index]);
        }


        let mut num_diffs = 0;
        let mut active_split = 0;

        // check if this is a vertical split (splt occurs at x+1)
        for vert_split in 1..(cleaned_input[row_start].len()) {
            num_diffs = 0;
            for row in row_start..row_end {
                let mut j = 0;
                while vert_split >= j + 1 && (vert_split + j) < input[row_start].len() {
                    println!("Vert split: {}, comparing: {},{} to {},{}, j: {}", vert_split, row, vert_split - 1 - j, row, vert_split + j, j);
                    if cleaned_input[row][vert_split - 1 - j] != cleaned_input[row][vert_split + j] {
                        num_diffs += 1;
                        if num_diffs > 1 {
                            break;
                        }
                    }
                    j += 1;
                }
                // println!("vert split: {}, active: {}");
                if num_diffs > 1 {
                    break;
                }
            }
            if num_diffs == 1 {
                active_split = vert_split;
                break;
            }
        }

        if num_diffs == 1 {
            // add vert split and keep going otherwise do horiz split now
            sum += active_split as i64;
            row_start = row_end + 1;
            continue;
        }

        // check if this is a vertical split (splt occurs at x+1)
        for horiz_split in 1..(row_end - row_start) {
            num_diffs = 0;

            for col in 0..cleaned_input[row_start].len() {
                let mut j = 0;
                println!("Horiz split: {}, j: {}", horiz_split, j);
                while horiz_split > j && (row_start + horiz_split + j) < row_end {
                    println!("Horiz split: {}, comparing: {},{} to {},{}", horiz_split, horiz_split - 1 - j, col, horiz_split + j, col);
                    if cleaned_input[row_start + horiz_split - 1 - j][col] != cleaned_input[row_start + horiz_split + j][col] {
                        num_diffs += 1;
                        println!("DIFFS: {}", num_diffs);
                        if num_diffs > 1 {
                            break;
                        }
                    }
                    j += 1;
                }
                
                if num_diffs > 1 {
                    break;
                }
            }

            println!("Num diffs found: {}", num_diffs);
            if num_diffs == 1 {
                active_split = horiz_split;
                break;
            }
        }
            
        
        

        if num_diffs != 1 {
            println!("Error");
            break;
        }

        // should be here now must be
        sum += 100 * active_split as i64;

        // advance current row
        row_start = row_end + 1;
    }

    println!("Abswerrw: {}", sum);
}