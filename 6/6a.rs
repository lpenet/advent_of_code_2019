use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn read_routes(filename: &str) -> Vec<(String,String)> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut ret: Vec<(String,String)> = Vec::new();

    for (_index,line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let route: Vec<&str> = line.split(")").collect();
        ret.push( (route[0].to_string(), route[1].to_string()) );
    }
    ret
}

fn main() {
    let routes = read_routes("input.txt");
    let planet_sun: HashMap<String,String> = routes.iter().map( |v| (v.1.clone(),v.0.clone())).collect();
    let mut orbits=0;
    for sun in planet_sun.values() {
        orbits = orbits + 1;
        let mut planet = sun;
        loop {
            let sun = planet_sun.get(planet);
            match sun {
                Some(sun) => {
                    orbits = orbits+1;
                    planet = sun;
                },
                None => {
                    break;
                }
            }
        }
    }

    println!("orbits: {}", orbits);
}
