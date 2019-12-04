use std::env;

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Syntax : {} <interval_start> <interval_end>",args[0]);
    }
    let interval_start:i64 = args[1].parse::<i64>().expect("Invalid start of interval");
    let interval_end:i64 = args[2].parse::<i64>().expect("Invalid end of interval");
    let mut nb_compatible = 0;
    for cur in interval_start..interval_end {
        let cur_str = cur.to_string();
        let vec_digits: Vec<u8> = cur_str.as_bytes().iter().map(|b| (b - '0' as u8) as u8).collect();
        let mut sorted_vec = vec_digits.clone();
        sorted_vec.sort();
        if sorted_vec != vec_digits {
            continue;
        }
        let mut dedup_vec = sorted_vec;
        dedup_vec.dedup();
        if dedup_vec.len() == vec_digits.len() {
            continue;
        }
        let mut counts: Vec<u8> = vec![0; 10];
        for v in vec_digits.iter() {
            counts[*v as usize] = counts[*v as usize]+1;
        }
        for count in counts.iter().rev() {
            if *count == 2 {
                nb_compatible= nb_compatible+1;
                break;
            }
        }
    }
    println!("nb valid: {}", nb_compatible);
}
