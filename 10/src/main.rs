use std::env;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate cgmath;

use core::ops::Neg;
use cgmath::*;
use cgmath::Vector2;

#[derive(Clone,Copy)]
struct Asteroid {
    x: u32,
    y: u32
}

impl PartialEq for Asteroid {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Clone)]
struct AsteroidField {
    asteroids: Vec<Asteroid>,
    canon_index: usize,
    canon_direction: Vector2::<f64>,
    former_angle: Option<f64>
}

impl Asteroid {
    fn displacement(&self, other: &Asteroid) -> Vector2<f64> {
        Vector2::<f64>::new(other.x as f64- self.x as f64,other.y as f64 - self.y as f64)
    }
}

impl AsteroidField {
    pub fn load_file(filename: &str) -> AsteroidField {
        let mut asteroids = Vec::<Asteroid>::new();        
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut canon_index = 0;
        for (y, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            if !line.trim().is_empty() {
                for (x, ch) in line.as_bytes().iter().enumerate() {
                    if *ch == '#' as u8 {
                        asteroids.push(Asteroid {
                            x: x as u32,
                            y: y as u32
                        });
                    } else if *ch == 'X' as u8 {
                        asteroids.push(Asteroid {
                            x: x as u32,
                            y: y as u32
                        });
                        canon_index = asteroids.len() - 1;
                    }
                }
            }
        }

        AsteroidField {
            asteroids,
            canon_index,
            canon_direction: Vector2::<f64> { x: 0., y: -1. },
            former_angle: None
        }
    }

    fn compute_others_in_sight(&self) -> Vec<(Asteroid,u32)> {
        let mut ret = Vec::<(Asteroid,u32)>::new();
        let ref_vector = Vector2::<f64> {
            x: 5.,
            y: 0.
        };

        for a in &self.asteroids {
            let mut displacements = Vec::<Vector2::<f64>>::new();
            for b in &self.asteroids {
                if b == a {
                    continue;
                }
                displacements.push(a.displacement(&b));
            }
            let mut angles: Vec<Rad<f64>> = displacements.iter().map(|d| d.angle(ref_vector)).collect();
            angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
            angles.dedup();
            ret.push((*a,angles.len() as u32));
        }
        ret
    }
    fn find_best_asteroid(&self) -> (Asteroid, u32) {
        let sights = self.compute_others_in_sight();
        *sights.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap()
    }

    fn shoot_asteroid(&mut self) -> Asteroid {
        let mut displacements = Vec::<(Asteroid,Vector2::<f64>)>::new();
        let canon_position = self.asteroids[self.canon_index];
        for a in &self.asteroids {
            if *a == canon_position {
                continue;
            }
            displacements.push((*a,canon_position.displacement(&a)));
        }

        println!("canon {:?}", self.canon_direction);
        match self.former_angle {
            Some(x) => println!("former angle: {}", x),
            None => println!("no former angle")
        }
        let mut angles_distances: Vec<(Asteroid,f64,f64)> = 
            displacements.iter().map(|d| (d.0, AsteroidField::modulo(&d.1.angle(self.canon_direction).neg().s), d.1.length()))
            .collect();
        angles_distances.sort_by(|a, b| { 
            if a.1.eq(&b.1) {
                return a.2.partial_cmp(&b.2).unwrap();
            } else {
                return a.1.partial_cmp(&b.1).unwrap();
            }
        });
        angles_distances.reverse();
        println!("{:?}", angles_distances);
        match self.former_angle {
            Some(x) => if angles_distances.len() > 1 { angles_distances.retain(|(aster, angle, distance)| angle.abs() > 1e-2) },
            None => { }
        }
        let shooted = angles_distances.pop().unwrap();
        self.former_angle = Some(shooted.1);
        let shooted = shooted.0;
        self.canon_direction = canon_position.displacement(&shooted);
        self.asteroids.retain(|i| *i != shooted);
        self.canon_index = self.asteroids.iter().position(|&aster| aster == canon_position).unwrap();
        shooted
    }

    fn modulo(param: &f64) -> f64 {
        let TWO_PI = 2.*std::f64::consts::PI;
        if *param > TWO_PI {
            return *param - TWO_PI;
        } else if *param < 0. {
            return *param + TWO_PI;
        } else {
            return *param;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_load_file() {
        let vec: Vec<(u32,u32)> = AsteroidField::load_file("test1.in").asteroids.iter().map(|a| (a.x, a.y)).collect();
        assert_eq!(vec, [(1, 0), (4, 0), (0, 2), (1, 2), (2, 2), (3, 2), (4, 2), (4, 3), (3, 4), (4, 4)]);
    }
    
    #[test]
    fn compute_other_in_sight1() {
        let asteroid_field: AsteroidField = AsteroidField::load_file("test1.in");
        let others_in_sight: Vec<((u32,u32),u32)> = asteroid_field.compute_others_in_sight().iter().map(|(a, s)| ((a.x,a.y),*s)).collect();
        assert_eq!(others_in_sight, [((1, 0), 7), ((4, 0), 7), ((0, 2), 6), ((1, 2), 7), ((2, 2), 7), ((3, 2), 7), ((4, 2), 5), ((4, 3), 7), ((3, 4), 8), ((4, 4), 7)]);
    }

    #[test]
    fn best_in_sight1() {
        let asteroid_field: AsteroidField = AsteroidField::load_file("test1.in");
        let best = asteroid_field.find_best_asteroid();
        assert_eq!(best, (Asteroid { x: 3, y: 4}, 8));
    }

    #[test]
    fn best_in_sight2() {
        let asteroid_field: AsteroidField = AsteroidField::load_file("test2.in");
        let best = asteroid_field.find_best_asteroid();
        assert_eq!(best, (Asteroid { x: 5, y: 8}, 33));
    }

    #[test]
    fn best_in_sight3() {
        let asteroid_field: AsteroidField = AsteroidField::load_file("test3.in");
        let best = asteroid_field.find_best_asteroid();
        assert_eq!(best, (Asteroid { x: 1, y: 2}, 35));
    }

    #[test]
    fn best_in_sight4() {
        let asteroid_field: AsteroidField = AsteroidField::load_file("test4.in");
        let best = asteroid_field.find_best_asteroid();
        assert_eq!(best, (Asteroid { x: 6, y: 3}, 41));
    }

    #[test]
    fn best_in_sight5() {
        let asteroid_field: AsteroidField = AsteroidField::load_file("test5.in");
        let best = asteroid_field.find_best_asteroid();
        assert_eq!(best, (Asteroid { x: 11, y: 13}, 210));
    }

    #[test]
    fn vaporizei6() {
        let mut asteroid_field: AsteroidField = AsteroidField::load_file("test6.in");
        assert_eq!(asteroid_field.asteroids[asteroid_field.canon_index], Asteroid { x: 8, y:3 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 8, y:1 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 9, y:0 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 9, y:1 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 10, y:0 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 9, y:2 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 11, y:1 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 12, y:1 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 11, y:2 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 15, y:1 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 12, y:2 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 13, y:2 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 14, y:2 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 15, y:2 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 12, y:3 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 16, y:4 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 15, y:4 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 10, y:4 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 4, y:4 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 2, y:4 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 2, y:3 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 0, y:2 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 1, y:2 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 0, y:1 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 1, y:1 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 5, y:2 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 1, y:0 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 5, y:1 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 6, y:1 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 6, y:0 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 7, y:0 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 8, y:0 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 10, y:1 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 14, y:0 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 16, y:1 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 13, y:3 });
        assert_eq!(asteroid_field.shoot_asteroid(), Asteroid { x: 14, y:3 });
    }
}

impl fmt::Debug for Asteroid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Display for Asteroid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Syntax : {} <inputi file>",args[0]);
    }
    let input_file = &args[1];
    let asteroid_field = AsteroidField::load_file(&input_file);
    let result = asteroid_field.find_best_asteroid();
    println!("{:?}", result);
}
