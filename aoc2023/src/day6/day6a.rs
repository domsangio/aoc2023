use num_traits::{Float, float::FloatCore};

fn root_solver(a: f64, b: f64, c: f64) -> (f64, f64) {
    let discriminant = ((b * b) - 4.0 * a * c).sqrt();
    ((-1.0 * b - discriminant) / (2.0 * a), (-1.0 * b + discriminant) / (2.0 * a))
}

pub fn day6a(input: &Vec<&str>) {
    let mut prod = 1.0;

    for (time, distance) in input[0].split_whitespace().skip(1).zip(input[1].split_whitespace().skip(1)) {
        let (mut lb, mut ub) = root_solver(-1.0, time.parse::<f64>().unwrap(), -1.0 * distance.parse::<f64>().unwrap());
        lb = lb.ceil();
        ub = ub.floor();
        lb = lb.max(0.0);
        ub = ub.min(time.parse::<f64>().unwrap());
        prod *= ub - lb + 1.0;
    }

    println!("Answer: {}", prod);
}