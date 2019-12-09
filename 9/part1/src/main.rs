use std::env;

mod intcode_computer;
use intcode_computer::Processor;

fn main() {
    let mut computer = Processor::init("input.txt");
    let args:Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Syntax : {} <input>",args[0]);
    }
    let input = args[1].parse::<i64>().expect("Invalid input");

    let inputs = vec!(&input);
    computer.process(&inputs);
}
