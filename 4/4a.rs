fn main() {
    let interval_start:i64 = 248345;
    let interval_end:i64 = 746315;
    let mut nb_compatible = 0;
    let mut cur = interval_start;
    while cur < interval_end {
        let cur_str = cur.to_string();
        cur = cur+1;
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
        nb_compatible= nb_compatible+1;
    }
    println!("nb valid: {}", nb_compatible);
}
