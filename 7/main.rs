mod intcode_computer;

fn run_amplifiers(program_filename: &str, phases: &Vec<i64>, initial_input: i64) -> i64{
    let mut input = initial_input;
    let mut output: i64 = -std::i64::MAX;
    for phase in phases {
        let inputs = vec![phase, &input];
        output = intcode_computer::process(program_filename, &inputs);
        input = output;
    }
    output
}

fn main() {
    let vec1: Vec<i64> = vec![0, 1, 2, 3, 4];
    let mut max_result = -std::i64::MAX;
    let mut best_phases: Vec<i64> = Vec::<i64>::new();
    // unrolled version is quicker to write
    for p1 in vec1.iter() {
        let mut vec2 = vec1.clone();
        vec2.retain(|p| p != p1);
        for p2 in vec2.iter() {
            let mut vec3 = vec2.clone();
            vec3.retain(|p| p != p2);
            for p3 in vec3.iter() {
                let mut vec4 = vec3.clone();
                vec4.retain(|p| p != p3);
                for p4 in vec4.iter() {
                    let mut vec5 = vec4.clone();
                    vec5.retain(|p| p != p4);
                    for p5 in vec5.iter() {
                        let phases = vec![*p1, *p2, *p3, *p4, *p5];
                        let cur_val: i64 = run_amplifiers("input.txt", &phases, 0);
                        if cur_val > max_result {
                            max_result = cur_val;
                            best_phases = phases;
                        }
                    }
                }
            }
        }
    }
    println!("Best result: {} for phases {:?}", max_result, best_phases);
}

