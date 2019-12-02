use std::fs;

fn main() {
    let input_filename = "input.txt";
    let content = fs::read_to_string(input_filename)
        .expect(&format!("Something went wrong reading {}", input_filename));
    let mut input_vector: Vec<u64> = content.split(",").map(str::parse::<u64>).filter_map(Result::ok).collect();
    input_vector[1] = 12;
    input_vector[2] = 2;

    for cur_inst in 0..input_vector.len()/4 {
        let cur_base_index = cur_inst*4;
        let op_code = input_vector[cur_base_index];
        let op1 = input_vector[input_vector[cur_base_index + 1] as usize];
        let op2 = input_vector[input_vector[cur_base_index + 2] as usize];
        let res;
        if op_code == 1 {
            res = op1 + op2;
        } else if op_code == 2 {
            res = op1 * op2;
        } else if op_code == 99 {
            break;
        } else {
            panic!(format!("unknown op code: {}", op_code));
        }
        let res_index = input_vector[cur_base_index + 3] as usize;
        input_vector[res_index] = res;
    }
    println!("result: {}", input_vector[0]);
}

