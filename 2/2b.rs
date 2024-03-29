use std::fs;

fn compute_result(input_vector: &Vec<u64>, init1: u64, init2: u64) -> u64 {
    let mut input_vector = input_vector.clone();
    input_vector[1] = init1;
    input_vector[2] = init2;

    for cur_inst in 0..input_vector.len()/4 {
        let cur_base_index = cur_inst*4;
        let op_code = input_vector[cur_base_index];
        let op1 = input_vector[input_vector[cur_base_index + 1] as usize];
        let op2 = input_vector[input_vector[cur_base_index + 2] as usize];
        let res;
        match op_code {
            1 => {
                res = op1 + op2;
            },
            2 => {
                res = op1 * op2;
            },
            99 => {
                break;
            },
            _ => panic!(format!("unknown op code: {}", op_code))
        }
        let res_index = input_vector[cur_base_index + 3] as usize;
        input_vector[res_index] = res;
    }
    input_vector[0]
}

fn main() {
    let input_filename = "input.txt";
    let content = fs::read_to_string(input_filename)
        .expect(&format!("Something went wrong reading {}", input_filename));
    let input_vector: Vec<u64> = content.split(",").map(str::parse::<u64>).filter_map(Result::ok).collect();
    for x in 0..99 {
        for y in 0..99 {
            let res = compute_result(&input_vector, x, y);
            if 19690720 == res {
                println!("result: {}", x*100+y);
                break;
            }
        }
    }
}

