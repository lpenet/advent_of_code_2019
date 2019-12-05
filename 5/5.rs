use std::fs;
use std::env;

fn read_input_vector(input_filename: &str) -> Vec<i64> {
    let content = fs::read_to_string(input_filename)
        .expect(&format!("Something went wrong reading {}", input_filename));
    let input_vector: Vec<i64> = content.split(",").map(str::parse::<i64>).filter_map(Result::ok).collect();
    input_vector
}

fn decode_param(input_vector: &Vec<i64>, mode: u8, cur: u32) -> i64 {
    if mode == 0 {
        return input_vector[input_vector[cur as usize] as usize];
    } else {
        return input_vector[cur as usize] as i64;
    }
}

fn get_from_cursor(cursor: &mut u32) -> u32 {
    let ret = *cursor;
    *cursor = *cursor+1;
    ret
}

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Syntax : {} <input>",args[0]);
    }
    let input = args[1].parse::<i64>().expect("Invalid input");
    let mut input_vector: Vec<i64> = read_input_vector("input.txt");
    let mut cur: u32 = 0;
    let mut final_res = -1;
    loop {
        let full_op_code: i64 = input_vector[get_from_cursor(&mut cur) as usize];
        let full_op_code: Vec<u8> = format!("{:05}", full_op_code).to_string().as_bytes().iter().map(|b| (b - '0' as u8) as u8).collect();
        let mut op_code = full_op_code[full_op_code.len()-1];
        if full_op_code[full_op_code.len()-2] == 9 {
            op_code = op_code+90;
        }
        let op_code = op_code;
        let mode_1st = full_op_code[full_op_code.len()-3];
        let mode_2nd = full_op_code[full_op_code.len()-4];
        let mode_3rd = full_op_code[full_op_code.len()-5];
        match op_code {
            1 => {
                let param1: i64 = decode_param(&input_vector, mode_1st, get_from_cursor(&mut cur));
                let param2: i64 = decode_param(&input_vector, mode_2nd, get_from_cursor(&mut cur));
                let res_index = input_vector[get_from_cursor(&mut cur) as usize];
                input_vector[res_index as usize] = param1 + param2;
            },
            2 => {
                let param1: i64 = decode_param(&input_vector, mode_1st, get_from_cursor(&mut cur));
                let param2: i64 = decode_param(&input_vector, mode_2nd, get_from_cursor(&mut cur));
                let res_index = input_vector[get_from_cursor(&mut cur) as usize];
                input_vector[res_index as usize] = param1 * param2;
            },
            3 => {
                let param1: i64 = input_vector[get_from_cursor(&mut cur) as usize];
                input_vector[param1 as usize] = input;
            },
            4 => {
                let param1: i64 = input_vector[get_from_cursor(&mut cur) as usize];
                final_res = input_vector[param1 as usize];
            },
            5 => {
                let param1: i64 = decode_param(&input_vector, mode_1st, get_from_cursor(&mut cur));
                let param2: i64 = decode_param(&input_vector, mode_2nd, get_from_cursor(&mut cur));
                if param1 != 0 {
                    cur = param2 as u32;
                }
            },
            6 => {
                let param1: i64 = decode_param(&input_vector, mode_1st, get_from_cursor(&mut cur));
                let param2: i64 = decode_param(&input_vector, mode_2nd, get_from_cursor(&mut cur));
                if param1 == 0 {
                    cur = param2 as u32;
                }
            },
            7 => {
                let param1: i64 = decode_param(&input_vector, mode_1st, get_from_cursor(&mut cur));
                let param2: i64 = decode_param(&input_vector, mode_2nd, get_from_cursor(&mut cur));
                let param3: i64 = input_vector[get_from_cursor(&mut cur) as usize];
                if param1 < param2 {
                    input_vector[param3 as usize] = 1;
                } else {
                    input_vector[param3 as usize] = 0;
                }
            },
            8 => {
                let param1: i64 = decode_param(&input_vector, mode_1st, get_from_cursor(&mut cur));
                let param2: i64 = decode_param(&input_vector, mode_2nd, get_from_cursor(&mut cur));
                let param3: i64 = input_vector[get_from_cursor(&mut cur) as usize];
                if param1 == param2 {
                    input_vector[param3 as usize] = 1;
                } else {
                    input_vector[param3 as usize] = 0;
                }
            },
        99 => {
            break;
            },
            _ => panic!(format!("unknown op code: {}", op_code))
        }
    }
    println!("{}",final_res);
}

