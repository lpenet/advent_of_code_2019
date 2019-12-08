use permutator::Permutation;

mod intcode_computer;
use intcode_computer::Processor;

fn run_amplifiers(program_filename: &str, phases: &Vec<i64>, initial_input: i64) -> i64{
    let mut input = initial_input;
    let mut first_pass: bool = true;
    let mut processors = vec![Processor::init(program_filename); 5];
    loop {
        for (index,phase) in phases.iter().enumerate() {
            let inputs;
            if first_pass {
                inputs = vec![phase, &input];
            } else {
                inputs = vec![&input];
            }
            let cur_processor = &mut processors[index];
            let output = cur_processor.process(&inputs);
            match output {
                Some(val) => { input = val; },
                None => { return input }
            }
        }
        first_pass = false;
    }
}

fn main() {
    let mut base_phases: Vec<i64> = vec![5, 6, 7, 8, 9];
    let mut max_result = -std::i64::MAX;
    let mut best_phases: Vec<i64> = Vec::<i64>::new();
    base_phases.permutation().for_each(|phases| {
        let cur_val: i64 = run_amplifiers("input.txt", &phases, 0);
        if cur_val > max_result {
            max_result = cur_val;
            best_phases = phases;
        }
    });
    println!("Best result: {} for phases {:?}", max_result, best_phases);
}

