use std::fs::File;
use std::io::{BufRead, BufReader};

fn compute_fuel(weight: f64) -> f64 {
    let res = (weight/3.).trunc() -2.;
    if res <= 0. {
        return 0.
    }
    else {
        return res + compute_fuel(res);
    }
}

fn read_and_compute_initial_weight() -> f64 {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    
    let mut accumulator: f64 = 0.;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if !line.trim().is_empty() {
            let as_f64: f64 = line.parse().unwrap();
            let required = compute_fuel(as_f64);
            accumulator += required;
        }
    }
    accumulator
}

fn main() {
    let total_fuel = read_and_compute_initial_weight();

    println!("Fuel with fuel of fuel: {}", total_fuel);
}

