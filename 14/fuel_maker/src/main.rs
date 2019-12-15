use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct ChemicalInput {
    name: String,
    count: u32
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Chemical {
    name: String,
    build_increment: u32,
    available: u32,
    inputs: Vec<ChemicalInput>
}

impl Chemical {
    fn new(name: String, build_increment: u32, inputs: Vec<ChemicalInput>) -> Chemical {
        Chemical {
            name,
            build_increment,
            available:0,
            inputs
        }
    }
}

const ORE: &str= "ORE";

struct ChemicalRegistry {
    chemicals: HashMap<String, Chemical>
}

impl ChemicalRegistry {
    fn load(input_filename: &str) -> ChemicalRegistry {
        let mut ret = ChemicalRegistry {
            chemicals: HashMap::<String,Chemical>::new()
        };

        let file = File::open(input_filename).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            let inputs_output:Vec<&str> = line.split("=>").collect();
            let output:&str = inputs_output[1].trim();
            let output_parts:Vec<&str> = output.split(" ").collect();
            let output_build_increment = output_parts[0].parse::<u32>().unwrap();
            let output_name = output_parts[1];

            let inputs = inputs_output[0].split(",").map(
                |input| {
                    let parts:Vec<&str> = input.trim().split(" ").collect();
                    let count = parts[0].parse::<u32>().unwrap();
                    let name = parts[1];
                    ChemicalInput {
                        name: name.to_string(),
                        count
                    }
                }).collect();
            ret.chemicals.insert(output_name.to_string(), 
                                 Chemical::new(
                                     output_name.to_string(),
                                     output_build_increment,
                                     inputs
                                     )
                                 );
        }
        ret.chemicals.insert(ORE.to_string(), Chemical::new(ORE.to_string(), 1, vec!()));
        ret
    }
}

#[cfg(test)]
mod chemical_registry_tests {
    use super::*;

    #[test]
    fn test_file_read() {
        let registry: ChemicalRegistry = ChemicalRegistry::load("test1.in");
        let mut keys:Vec<String> = registry.chemicals.keys().map(|s| s.clone()).collect();
        keys.sort();
        assert_eq!(&keys, &vec!("A", "B", "C", "D", "E", "FUEL", "ORE"));
        let chem_e = registry.chemicals.get("E").unwrap();
        assert_eq!(chem_e.name, "E");
        assert_eq!(chem_e.build_increment, 1);
        assert_eq!(chem_e.available, 0);
        assert_eq!(chem_e.inputs, vec!(ChemicalInput { name: "A".to_string(), count: 7 }, ChemicalInput { name: "D".to_string(), count: 1 }));
    }
}

fn main() {
}

