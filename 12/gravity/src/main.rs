use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Planet {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64
}

impl Planet {
    fn apply_velocity(&mut self) {
        self.x = self.x + self.vx;
        self.y = self.y + self.vy;
        self.z = self.z + self.vz;
    }
    
    fn potential_energy(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn kinetic_energy(&self) -> i64 {
        self.vx.abs() + self.vy.abs() + self.vz.abs()
    }

    fn total_energy(&self) -> i64 {
        self.potential_energy() * self.kinetic_energy()
    }
}

struct PlanetarySystem {
    planets: Vec<Planet>
}

impl PlanetarySystem {
    fn load_file(filename: &str) -> PlanetarySystem {
        let mut planet_system = PlanetarySystem {
            planets: Vec::<Planet>::new()
        };
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let mut line = line.unwrap();
            line.retain(|c| !r#"<>"#.contains(c));
            if !line.trim().is_empty() {
                let coords: Vec<i64> = line.split(",").map(|coord| {
                    let parts: Vec<i64> = coord.split("=").map(str::parse::<i64>).filter_map(Result::ok).collect();
                    parts[0]
                }).collect();
                planet_system.planets.push( Planet {
                    x: coords[0],
                    y: coords[1],
                    z: coords[2],
                    vx: 0,
                    vy: 0,
                    vz: 0
                });
            }
        }
        planet_system
    }

    fn update_velocity(&mut self) {
        let mut new_planets = Vec::<Planet>::new();
        for p1 in &self.planets {
            let mut new_p1 = p1.clone();
            for p2 in &self.planets {
                if p1.x > p2.x {
                    new_p1.vx = new_p1.vx-1;
                } else if p1.x < p2.x {
                    new_p1.vx = new_p1.vx+1;
                }
                if p1.y > p2.y {
                    new_p1.vy = new_p1.vy-1;
                } else if p1.y < p2.y {
                    new_p1.vy = new_p1.vy+1;
                }
                if p1.z > p2.z {
                    new_p1.vz = new_p1.vz-1;
                } else if p1.z < p2.z {
                    new_p1.vz = new_p1.vz+1;
                }
            }
            new_planets.push(new_p1);
        }

        self.planets = new_planets;
    }

    fn apply_velocity(&mut self) {
        for p in self.planets.iter_mut() {
            p.apply_velocity();
        }
    }

    fn time_step(&mut self) {
        self.update_velocity();
        self.apply_velocity();
    }

    fn total_energy(&self) -> i64 {
        self.planets.iter().map(|p| p.total_energy()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let system: PlanetarySystem = PlanetarySystem::load_file("input.txt");
        assert_eq!(system.planets, [Planet{x:3, y:-6, z:6, vx:0, vy:0, vz:0 },
                   Planet{x:10, y:7, z:-9, vx:0, vy:0, vz:0},
                   Planet{x:-3, y:-7, z:9, vx:0, vy:0, vz:0},
                   Planet{x:-8, y:-0, z:4, vx:0, vy:0, vz:0}]);
    }

    #[test]
    fn test_one_step() {
        let mut system: PlanetarySystem = PlanetarySystem::load_file("test2.in");
        system.time_step();
        assert_eq!(system.planets, [Planet{x:2, y:-1, z:1, vx:3, vy:-1, vz:-1 },
                   Planet{x:3, y:-7, z:-4, vx:1, vy:3, vz:3},
                   Planet{x:1, y:-7, z:5, vx:-3, vy:1, vz:-3 },
                   Planet{x:2, y:2, z:0, vx:-1, vy:-3, vz:1}]);
    }

    #[test]
    fn test_ten_steps() {
        let mut system: PlanetarySystem = PlanetarySystem::load_file("test2.in");
        for _ in 0..10 {
            system.time_step();
        }
        assert_eq!(system.planets, [Planet{x:2, y:1, z:-3, vx:-3, vy:-2, vz:1 },
                   Planet{x:1, y:-8, z:-0, vx:-1, vy:1, vz:3},
                   Planet{x:3, y:-6, z:1, vx:3, vy:2, vz:-3 },
                   Planet{x:2, y:0, z:4, vx:1, vy:-1, vz:-1}]);

        assert_eq!(system.total_energy(), 179);
    }

    #[test]
    fn test_hundred_steps() {
        let mut system: PlanetarySystem = PlanetarySystem::load_file("test3.in");
        for _ in 0..100 {
            system.time_step();
        }
        assert_eq!(system.planets, [Planet{x:8, y:-12, z:-9, vx:-7, vy:3, vz:0 },
                   Planet{x:13, y:16, z:-3, vx:3, vy:-11, vz:-5},
                   Planet{x:-29, y:-11, z:-1, vx:-3, vy:7, vz:4 },
                   Planet{x:16, y:-13, z:23, vx:7, vy:1, vz:1}]);

        assert_eq!(system.total_energy(), 1940);
    }

}

fn part1() {
    let mut system: PlanetarySystem = PlanetarySystem::load_file("input.txt");
    for _ in 0..1000 {
        system.time_step();
    }
    println!("Total energy: {}", system.total_energy());
}

fn main() {
   part1(); 
}

