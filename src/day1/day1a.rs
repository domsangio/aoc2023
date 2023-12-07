fn get_first_decimal<T>(iter: T) -> u32 
where
    T: Iterator<Item = char>,
{
    for x in iter {
        if x.is_numeric() {
            return x.to_digit(10).unwrap();
        }
    }
    0
}

pub fn day1a(input: &Vec<&str>) {
    println!("{}", input.iter().map(|line| get_first_decimal(line.chars()) * 10 + get_first_decimal(line.chars().rev())).sum::<u32>());
}