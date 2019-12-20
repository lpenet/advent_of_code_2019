use std::fs;

fn compute_mask_value(param_len: usize, step: usize, index: usize) -> i32 {
    let mask: [i32; 4] = [0, 1, 0, -1];
    mask[((index+1)%(4*(step+1)))/(step+1)]

}

fn one_fft_pass(cur_input: &Vec<u8>) -> Vec<u8> {
    let param_len: usize = cur_input.len();
    let mut output_vec = Vec::<u8>::with_capacity(param_len);

    for step in 0..param_len {
        let mut accum: i32 = 0;
        for i in 0..param_len {
            let mask_value = compute_mask_value(param_len,step, i);
            accum = accum + (cur_input[i] as i32 * mask_value) as i32;
        }
        let digit = (accum.abs() % 10) as u8;
        output_vec.push(digit);
    }

    output_vec
}

// offset is so big that there are only ones
// so,we can jsut sum
fn one_fft_pass_2(cur_input: &Vec<u8>) -> Vec<u8> {
    let param_len: usize = cur_input.len();
    let mut output_vec = Vec::<u8>::with_capacity(param_len);

    let mut accum: i32 = 0;
    for i in (0..param_len).rev() {
        accum = accum + cur_input[i] as i32;
        let digit = (accum.abs() % 10) as u8;
        output_vec.push(digit);
    }
    output_vec.reverse();
    output_vec
}

fn string_to_vec_u8(input: &str) -> Vec<u8> {
    input.to_string().as_bytes().iter().filter(|b| **b <= '9' as u8 && **b >= '0' as u8).map(|b| (b - '0' as u8) as u8).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec_start_with(vec1: &Vec<u8>, begin: &Vec<u8>) -> bool {
        vec1.iter().zip(begin.iter()).filter(|&(a,b)| a == b).count() == begin.len()
    }

    #[test]
    fn test_1_step() {
        let input = "12345678";
        let res = one_fft_pass(&string_to_vec_u8(input));
        assert!(vec_start_with(&res,&string_to_vec_u8("48226158")));
    }

    #[test]
    fn test_2_steps() {
        let input = "12345678";
        let res = one_fft_pass(&string_to_vec_u8(input));
        let res = one_fft_pass(&res, &masks);
        assert!(vec_start_with(&res,&string_to_vec_u8("34040438")));
    }

    #[test]
    fn test_4_steps() {
        let input = "12345678";
        let res = one_fft_pass(&string_to_vec_u8(input));
        let res = one_fft_pass(&res, &masks);
        let res = one_fft_pass(&res, &masks);
        let res = one_fft_pass(&res, &masks);
        assert!(vec_start_with(&res,&string_to_vec_u8("01029498")));
    }

    fn test_100_steps(input: &Vec<u8>, result_begin: &Vec<u8>) {
        let mut cur = input.clone();
        for i in 0..100 {
            cur = one_fft_pass(&cur);
        }
        assert!(vec_start_with(&cur,result_begin));
    }

    #[test]
    fn test_100_steps_1() {
        test_100_steps(&string_to_vec_u8("80871224585914546619083218645595"), &string_to_vec_u8("24176176"));
    }

    #[test]
    fn test_100_steps_2() {
        test_100_steps(&string_to_vec_u8("19617804207202209144916044189917"), &string_to_vec_u8("73745418"));
    }

    #[test]
    fn test_100_steps_3() {
        test_100_steps(&string_to_vec_u8("69317163492948606335995924319873"), &string_to_vec_u8("52432133"));
    }
}

fn part1() {
    let mut cur: Vec<u8> = string_to_vec_u8(&fs::read_to_string("input.txt").unwrap());
    for _ in 0..100 {
        cur = one_fft_pass(&cur);
    }
    let vec_first8: Vec<u8> = cur[0..8].iter().map(|x| *x + '0' as u8).collect();
    let first8: &str = std::str::from_utf8(&vec_first8).unwrap();
    println!("First 8: {}", first8);
}

fn part2() {
    let mut input: Vec<u8> = string_to_vec_u8(&fs::read_to_string("input.txt").unwrap());
    let mut cur = Vec::<u8>::new();
    let mut tmp = Vec::<u8>::new();
    for _ in 0..100 {
        tmp.append(&mut input.clone());
    }
    for _ in 0..100 {
        cur.append(&mut tmp.clone());
    }
    
    let vec_first7: Vec<u8> = cur[0..7].iter().map(|x| *x + '0' as u8).collect();
    let first7: &str = std::str::from_utf8(&vec_first7).unwrap();
    let first7: usize = first7.parse::<usize>().unwrap();
    let mut cur: Vec<u8> = cur[first7..].to_vec();
    for i in 0..100 {
        cur = one_fft_pass_2(&cur);
    }

    let vec_first8: Vec<u8> = cur[0..8].iter().map(|x| *x + '0' as u8).collect();
    let first8: &str = std::str::from_utf8(&vec_first8).unwrap();
    println!("First 8: {}", first8);
}

fn main() {
    part1();
    part2();
}

