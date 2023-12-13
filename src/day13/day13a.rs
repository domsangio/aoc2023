use itertools::Itertools;

pub fn day13a(input: &Vec<&str>) {
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


        let mut active = true;
        let mut active_vert_split = 0;

        // check if this is a vertical split (splt occurs at x+1)
        for vert_split in 1..(cleaned_input[row_start].len()) {
            active = true;
            for row in row_start..row_end {
                println!("row?: {}", row);
                let mut j = 0;
                while vert_split >= j + 1 && (vert_split + j) < input[row_start].len() {
                    println!("Vert split: {}, comparing: {},{} to {},{}, j: {}", vert_split, row, vert_split - 1 - j, row, vert_split + j, j);
                    if cleaned_input[row][vert_split - 1 - j] != cleaned_input[row][vert_split + j] {
                        println!("DIFF");
                        active = false;
                        break;
                    }
                    j += 1;
                }
                // println!("vert split: {}, active: {}");
                if active == false {
                    break;
                }
            }
            if active == true {
                active_vert_split = vert_split;
                break;
            }
        }

        if active == true {
            // add vert split and keep going otherwise do horiz split now
            sum += active_vert_split as i64;
            row_start = row_end + 1;
            continue;
        }

        // check if this is a vertical split (splt occurs at x+1)
        for horiz_split in 1..(row_end - row_start) {
            active = true;

            let mut j = 0;
            println!("Horiz split: {}, j: {}", horiz_split, j);
            while horiz_split > j && (row_start + horiz_split + j) < row_end {
                println!("Horiz split: {}, comparing: {} to {}", horiz_split, horiz_split - 1 - j, horiz_split + j);
                if input[row_start + horiz_split - 1 - j] != input[row_start + horiz_split + j] {
                    println!("DIFF");
                    active = false;
                    break;
                }
                j += 1;
            }
            
            if active == true {
                active_vert_split = horiz_split;
                break;
            }
        }
            
        
        

        if active == false {
            println!("Error");
            break;
        }

        // should be here now must be
        sum += 100 * active_vert_split as i64;

        // advance current row
        row_start = row_end + 1;
    }

    println!("Abswerrw: {}", sum);
}