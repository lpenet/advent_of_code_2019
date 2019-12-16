use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct ChemicalInput {
    name: String,
    count: u64
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Chemical {
    name: String,
    build_increment: u64,
    available: u64,
    built: u64,
    inputs: Vec<ChemicalInput>
}

impl Chemical {
    fn new(name: String, build_increment: u64, inputs: Vec<ChemicalInput>) -> Chemical {
        Chemical {
            name,
            build_increment,
            available:0,
            built: 0,
            inputs
        }
    }

    fn reset(&mut self) {
        self.available = 0;
        self.built = 0;
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
            let output_build_increment = output_parts[0].parse::<u64>().unwrap();
            let output_name = output_parts[1];

            let inputs = inputs_output[0].split(",").map(
                |input| {
                    let parts:Vec<&str> = input.trim().split(" ").collect();
                    let count = parts[0].parse::<u64>().unwrap();
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

    fn prepare(&mut self, element_name: &str, quantity: u64) -> Result<(), &'static str> {
        // we do this inefficient copy to satisfy rust borrowing policy...What a pity
        let chemical = self.chemicals.get(element_name).clone();
        if chemical.is_some() {
            let mut chemical = chemical.unwrap().clone();
            if chemical.available < quantity {
                let packets_to_build = (((quantity - chemical.available) as f64)/(chemical.build_increment as f64)).ceil() as u64;
                let built_count = packets_to_build * chemical.build_increment;
                chemical.available = chemical.available + built_count;
                chemical.built = chemical.built + built_count;
                for ingredient in &chemical.inputs {
                    self.prepare(&ingredient.name, ingredient.count * packets_to_build)?;
                }
            }
            chemical.available = chemical.available - quantity;
            self.chemicals.insert(chemical.name.clone(), chemical);
            Ok(())
        } else {
            Err("Element not found")
        }
    }

    fn get_used_material_count(&self, element_name: &str) -> Result<u64, &'static str> {
        let chemical = self.chemicals.get(element_name);
        match chemical {
            Some(x) => Ok(x.built),
            None => Err("Unknown element")
        }
    }

    fn reset(&mut self) {
        for (_, chemical) in self.chemicals.iter_mut() {
            chemical.reset();
        }
    }

    fn dichoto_use_ore(&mut self, quantity: u64) -> u64 {
        self.prepare("FUEL", 1);
        let used_ore_for_one = self.get_used_material_count("ORE").unwrap();
        let mut lower_bound = quantity/used_ore_for_one;
        let mut upper_bound = lower_bound * 4;
        while lower_bound != (upper_bound-1) {
            let cur = (lower_bound + upper_bound) / 2;
            self.reset();
            self.prepare("FUEL", cur);
            let used_ore = self.get_used_material_count("ORE").unwrap();
            if used_ore > quantity {
                upper_bound = cur;
            } else {
                lower_bound = cur;
            }
        }
        lower_bound
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

    #[test]
    fn test_build_fuel_1() {
        let mut registry: ChemicalRegistry = ChemicalRegistry::load("test1.in");
        registry.prepare("FUEL", 1);
        assert_eq!(registry.get_used_material_count("ORE").unwrap(), 31);
    }

    #[test]
    fn test_build_fuel_2() {
        let mut registry: ChemicalRegistry = ChemicalRegistry::load("test2.in");
        registry.prepare("FUEL", 1);
        assert_eq!(registry.get_used_material_count("ORE").unwrap(), 165);
    }

    #[test]
    fn test_build_fuel_3() {
        let mut registry: ChemicalRegistry = ChemicalRegistry::load("test3.in");
        registry.prepare("FUEL", 1);
        assert_eq!(registry.get_used_material_count("ORE").unwrap(), 13312);
    }

    #[test]
    fn test_build_fuel_5() {
        let mut registry: ChemicalRegistry = ChemicalRegistry::load("test5.in");
        registry.prepare("FUEL", 1);
        assert_eq!(registry.get_used_material_count("ORE").unwrap(), 2210736);
    }

    #[test]
    fn test_one_trillion_3() {
        let mut registry: ChemicalRegistry = ChemicalRegistry::load("test3.in");
        let fuel = registry.dichoto_use_ore(1000000000000);
        assert_eq!(fuel, 82892753);
    }

    #[test]
    fn test_one_trillion_4() {
        let mut registry: ChemicalRegistry = ChemicalRegistry::load("test4.in");
        let fuel = registry.dichoto_use_ore(1000000000000);
        assert_eq!(fuel, 5586022);
    }

    #[test]
    fn test_one_trillion_5() {
        let mut registry: ChemicalRegistry = ChemicalRegistry::load("test5.in");
        let fuel = registry.dichoto_use_ore(1000000000000);
        assert_eq!(fuel, 460664);
    }
}

fn part1() {
    let mut registry: ChemicalRegistry = ChemicalRegistry::load("input.txt");
    registry.prepare("FUEL", 1);
    println!("Required ORE: {}", registry.get_used_material_count("ORE").unwrap());
}

fn part2() {
    let mut registry: ChemicalRegistry = ChemicalRegistry::load("input.txt");
    let fuel = registry.dichoto_use_ore(1000000000000);
    println!("Max fuel: {}", fuel);
}

fn main() {
    part1();
    part2();
}

