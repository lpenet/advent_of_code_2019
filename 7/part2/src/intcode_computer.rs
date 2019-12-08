use std::fs;

#[derive(Clone)]
pub struct Processor {
    memory: Vec<i64>,
    cur: u32,
}

impl Processor {
    pub fn init(program_filename: &str) -> Processor {
        Processor {
            memory: Processor::read_input_vector(program_filename),
            cur: 0
        }
    }
    pub fn process(&mut self, inputs: &Vec<&i64>) -> Option<i64> {
        let mut input = inputs.iter();
        loop {
            let full_op_code: i64 = self.memory[Processor::get_from_cursor(&mut self.cur) as usize];
            let full_op_code: Vec<u8> = format!("{:05}", full_op_code).to_string().as_bytes().iter().map(|b| (b - '0' as u8) as u8).collect();
            let mut op_code = full_op_code[full_op_code.len()-1];
            if full_op_code[full_op_code.len()-2] == 9 {
                op_code = op_code+90;
            }
            let op_code = op_code;
            let mode_1st = full_op_code[full_op_code.len()-3];
            let mode_2nd = full_op_code[full_op_code.len()-4];
            match op_code {
                1 => {
                    let param1: i64 = Processor::decode_param(&self.memory, mode_1st, Processor::get_from_cursor(&mut self.cur));
                    let param2: i64 = Processor::decode_param(&self.memory, mode_2nd, Processor::get_from_cursor(&mut self.cur));
                    let res_index = self.memory[Processor::get_from_cursor(&mut self.cur) as usize];
                    self.memory[res_index as usize] = param1 + param2;
                },
                2 => {
                    let param1: i64 = Processor::decode_param(&self.memory, mode_1st, Processor::get_from_cursor(&mut self.cur));
                    let param2: i64 = Processor::decode_param(&self.memory, mode_2nd, Processor::get_from_cursor(&mut self.cur));
                    let res_index = self.memory[Processor::get_from_cursor(&mut self.cur) as usize];
                    self.memory[res_index as usize] = param1 * param2;
                },
                3 => {
                    let param1: i64 = self.memory[Processor::get_from_cursor(&mut self.cur) as usize];
                    self.memory[param1 as usize] = **input.next().unwrap();
                },
                4 => {
                    let param1: i64 = self.memory[Processor::get_from_cursor(&mut self.cur) as usize];
                    let final_res = self.memory[param1 as usize];
                    return Some(final_res);
                },
                5 => {
                    let param1: i64 = Processor::decode_param(&self.memory, mode_1st, Processor::get_from_cursor(&mut self.cur));
                    let param2: i64 = Processor::decode_param(&self.memory, mode_2nd, Processor::get_from_cursor(&mut self.cur));
                    if param1 != 0 {
                        self.cur = param2 as u32;
                    }
                },
                6 => {
                    let param1: i64 = Processor::decode_param(&self.memory, mode_1st, Processor::get_from_cursor(&mut self.cur));
                    let param2: i64 = Processor::decode_param(&self.memory, mode_2nd, Processor::get_from_cursor(&mut self.cur));
                    if param1 == 0 {
                        self.cur = param2 as u32;
                    }
                },
                7 => {
                    let param1: i64 = Processor::decode_param(&self.memory, mode_1st, Processor::get_from_cursor(&mut self.cur));
                    let param2: i64 = Processor::decode_param(&self.memory, mode_2nd, Processor::get_from_cursor(&mut self.cur));
                    let param3: i64 = self.memory[Processor::get_from_cursor(&mut self.cur) as usize];
                    if param1 < param2 {
                        self.memory[param3 as usize] = 1;
                    } else {
                        self.memory[param3 as usize] = 0;
                    }
                },
                8 => {
                    let param1: i64 = Processor::decode_param(&self.memory, mode_1st, Processor::get_from_cursor(&mut self.cur));
                    let param2: i64 = Processor::decode_param(&self.memory, mode_2nd, Processor::get_from_cursor(&mut self.cur));
                    let param3: i64 = self.memory[Processor::get_from_cursor(&mut self.cur) as usize];
                    if param1 == param2 {
                        self.memory[param3 as usize] = 1;
                    } else {
                        self.memory[param3 as usize] = 0;
                    }
                },
            99 => {
                return None;
                },
                _ => panic!(format!("unknown op code: {}", op_code))
            }
        }
    }


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
}


