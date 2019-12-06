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

fn path_to_origin(planet: &str, planet_sun: &HashMap<String,String>) -> Vec<String> {
    let mut planet = planet;
    let mut ret: Vec<String> = Vec::<String>::new();
    ret.push(planet.to_string());
    loop {
        let sun = planet_sun.get(planet);
        match sun {
            Some(sun) => {
                planet = sun;
                ret.push(sun.clone());
            },
            None => {
                break;
            }
        }
    }
    ret
}

fn count_orbits(planet_sun: &HashMap<String,String>) -> u32 {
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
    orbits
}

fn find_minimal_orbital_transfers(planet_sun: &HashMap<String,String>) -> Option<u32> {
    let path_you = path_to_origin("YOU", planet_sun);
    let path_santa = path_to_origin("SAN", planet_sun);

    let mut idx_you = path_you.len() - 1;
    let mut idx_santa = path_santa.len() - 1;
    while (path_you[idx_you] == path_santa[idx_santa]) && (idx_you != 0) && (idx_santa != 0) {
        idx_you = idx_you - 1;
        idx_santa = idx_santa - 1;
    }
    if idx_you == path_you.len() - 1 {
        return None;
    }
    Some((idx_you+idx_santa) as u32)
}

fn main() {
    let routes = read_routes("input.txt");
    let planet_sun: HashMap<String,String> = routes.iter().map( |v| (v.1.clone(),v.0.clone())).collect();
    let orbits = count_orbits(&planet_sun);
    println!("Orbits: {}", orbits);
    let minimal_transfers = find_minimal_orbital_transfers(&planet_sun);
    match minimal_transfers {
        None => {
            println!("No path found!");
        },
        Some(count) => {
            println!("Minimum number of orbital transfers: {}", count);
        }
    }
}

