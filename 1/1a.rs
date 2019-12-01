use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    
    let mut accumulator: f64 = 0.;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if !line.trim().is_empty() {
            let as_u64: f64 = line.parse().unwrap();
            let required = (as_u64/3.).trunc() - 2.;
            accumulator += required;
        }
    }

    println!("total: {}", accumulator);
}

