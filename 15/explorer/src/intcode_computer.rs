use std::fs;


pub trait InputCallback {
    fn callback(&mut self) -> i64;
}

#[derive(Clone)]
pub struct Processor {
    pub memory: Vec<i64>,
    cur: u32,
    relative_offset: u32
}

impl Processor {
    pub fn init(program_filename: &str) -> Processor {
        Processor {
            memory: Processor::read_input_vector(program_filename),
            cur: 0,
            relative_offset: 0
        }
    }

    pub fn reset_pointer(&mut self) {
        self.cur = 0;
    }

    pub fn process<C>(&mut self, f: &mut C) -> Option<i64> 
        where C: InputCallback{
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
            let mode_3rd = full_op_code[full_op_code.len()-5];
            match op_code {
                1 => {
                    let param1: i64 = self.decode_param(mode_1st);
                    let param2: i64 = self.decode_param(mode_2nd);
                    let res_index = self.memory[Processor::get_from_cursor(&mut self.cur) as usize];
                    self.assign_offset(mode_3rd, res_index, param1 + param2);
                },
                2 => {
                    let param1: i64 = self.decode_param(mode_1st);
                    let param2: i64 = self.decode_param(mode_2nd);
                    let res_index = self.memory[Processor::get_from_cursor(&mut self.cur) as usize];
                    self.assign_offset(mode_3rd, res_index, param1 * param2);
                },
                3 => {
                    let param1: i64 = self.memory[Processor::get_from_cursor(&mut self.cur) as usize];
                    self.assign_offset(mode_1st, param1, f.callback());
                },
                4 => {
                    let param1: i64 = self.memory[Processor::get_from_cursor(&mut self.cur) as usize];
                    let output;
                    match mode_1st {
                        0 => {
                            output = self.memory[param1 as usize];
                        },
                        1 => {
                            output = param1;
                        },
                        2 => {
                            output = self.memory[(param1+self.relative_offset as i64) as usize];
                        },
                        _ => {
                            panic!("Unsupported mode");
                        }
                    }
                    return Some(output);
                },
                5 => {
                    let param1: i64 = self.decode_param(mode_1st);
                    let param2: i64 = self.decode_param(mode_2nd);
                    if param1 != 0 {
                        self.cur = param2 as u32;
                    }
                },
                6 => {
                    let param1: i64 = self.decode_param(mode_1st);
                    let param2: i64 = self.decode_param(mode_2nd);
                    if param1 == 0 {
                        self.cur = param2 as u32;
                    }
                },
                7 => {
                    let param1: i64 = self.decode_param(mode_1st);
                    let param2: i64 = self.decode_param(mode_2nd);
                    let param3: i64 = self.memory[Processor::get_from_cursor(&mut self.cur) as usize];
                    if param1 < param2 {
                        self.assign_offset(mode_3rd, param3, 1);
                    } else {
                        self.assign_offset(mode_3rd, param3, 0);
                    }
                },
                8 => {
                    let param1: i64 = self.decode_param(mode_1st);
                    let param2: i64 = self.decode_param(mode_2nd);
                    let param3: i64 = self.memory[Processor::get_from_cursor(&mut self.cur) as usize];
                    if param1 == param2 {
                        self.assign_offset(mode_3rd, param3, 1);
                    } else {
                        self.assign_offset(mode_3rd, param3, 0);
                    }
                },
                9 => {
                    let param1: i64 = self.decode_param(mode_1st);
                    self.relative_offset = (self.relative_offset as i64 + param1) as u32;
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

    fn translate_offset(&self, mode: u8, offset: i64) -> i64 {
        match mode {
            0 => {
                return self.memory[offset as usize];
            },
            1 => {
                return offset;
            },
            2 => {
                return self.relative_offset as i64 + self.memory[offset as usize];
            },
            _ => {
                panic!("Unhandled mode");
            }
        }
    }

    fn assign_offset(&mut self, mode: u8, offset: i64, value: i64) {
        let real_offset: usize;
        match mode {
            0 => {
                real_offset = offset as usize;
            },
            2 => {
                real_offset = (self.relative_offset as i64 + offset) as usize;
            },
            _ => {
                panic!("Unhandled mode");
            }
        }
        if real_offset >= self.memory.len() {
            self.memory.resize(real_offset+1, 0);
        }
        self.memory[real_offset] = value;
    }

    fn decode_param(&mut self, mode: u8) -> i64 {
        let cur = Processor::get_from_cursor(&mut self.cur);
        let offset = self.translate_offset(mode, cur as i64) as usize;
        if offset < self.memory.len() {
            self.memory[offset]
        } else {
            0
        }
    }

    fn get_from_cursor(cursor: &mut u32) -> u32 {
        let ret = *cursor;
        *cursor = *cursor+1;
        ret
    }
}


