use roots::Roots;
use roots::find_roots_quadratic;

fn root_solver(a: f64, b: f64, c: f64) -> (f64, f64) {
    // println!("a: {}, b: {}, c: {}", a, b, c);
    // let discriminant = ((b * b) - 4.0 * a * c).sqrt();
    // println!("Discrim: {}", discriminant);
    // ((-1.0 * b - discriminant) / (2.0 * a), (-1.0 * b + discriminant) / (2.0 * a));

    match find_roots_quadratic(a, b, c) {
        Roots::Two(a) => (a[0], a[1]),
        _ => (-1.0, -1.0)
    }
}

pub fn day6b(input: &Vec<&str>) {
    let mut prod = 1.0;

    let mut total_time = String::new();
    let mut total_distance = String::new();

    for (time, distance) in input[0].split_whitespace().skip(1).zip(input[1].split_whitespace().skip(1)) {
        total_time.push_str(time);
        total_distance.push_str(distance);
    }

    println!("total time: {}, total distance: {}", total_time, total_distance);

    let (mut lb, mut ub) = root_solver(-1.0, total_time.parse::<f64>().unwrap(), -1.0 * total_distance.parse::<f64>().unwrap());
    lb = lb.ceil();
    ub = ub.floor();
    lb = lb.max(0.0);
    ub = ub.min(total_time.parse::<f64>().unwrap());
    prod = ub - lb + 1.0;

    println!("Answer: {}", prod);
}

// let mut owned_string: String = "hello ".to_owned();
// let borrowed_string: &str = "world";

// owned_string.push_str(borrowed_string);