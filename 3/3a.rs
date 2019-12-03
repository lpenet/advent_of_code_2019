use std::fs;
use std::cmp;

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i64,
    y: i64
}

impl Point {
    pub fn manhattan_distance(self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Copy, Clone, Debug)]
struct Line<'a>(&'a Point, &'a Point);

impl<'a> Line<'a> {
    pub fn intersect(self, other: Self) -> Option<Point> {
        // yeah, we could fold that with functions. But that is so short...
        let x0 = cmp::min(self.0.x, self.1.x);
        let x1 = cmp::max(self.0.x, self.1.x);
        let y0 = cmp::min(self.0.y, self.1.y);
        let y1 = cmp::max(self.0.y, self.1.y);
        
        let u0 = cmp::min(other.0.x, other.1.x);
        let u1 = cmp::max(other.0.x, other.1.x);
        let v0 = cmp::min(other.0.y, other.1.y);
        let v1 = cmp::max(other.0.y, other.1.y);

        if u0 >= x0 && u0 <= x1 && y0 >= v0 && y0 <= v1 {
            return Some(Point {
                x: u0,
                y: y0
            });
        } else if x0 >= u0 && x0 <= u1 && v0 >= y0 && v0 <= y1 {
            return Some(Point {
                x: x0,
                y: v0
            });
        }
        None
    }
}

fn parse_coords(line: &str) -> Vec<Point> {
    let points: Vec<&str> = line.split(",").collect();
    let mut cur_x: i64 = 0;
    let mut cur_y: i64 = 0;
    let coords: Vec<Point> = points.iter().map(
        |c| {
            let cur_op = c.chars().nth(0).unwrap();
            let cur_arg = c[1..].parse::<i64>().unwrap();
            match cur_op {
                'R' => cur_x += cur_arg,
                'L' => cur_x -= cur_arg,
                'U' => cur_y += cur_arg,
                'D' => cur_y -= cur_arg,
                _ => panic!("Unexpected char")
            };
            Point { 
                x: cur_x,
                y: cur_y
            }
        }
    ).collect();
    coords
}

fn main() {
    let input_filename = "input.txt";
    let content: String = fs::read_to_string(input_filename)
        .expect(&format!("Something went wrong reading {}", input_filename));
    let mut it = content.lines();
    let first_coords = parse_coords(it.next().unwrap());
    let second_coords = parse_coords(it.next().unwrap());

    let mut first_it = first_coords.iter();
    let mut a0 = first_it.next().unwrap();
    let mut min_distance = i64::max_value();
    let mut closest_collision = Point { x: 0, y: 0 };
    for a1 in first_it {
        let mut second_it = second_coords.iter();
        let l1 = Line(a0, a1);
//        println!("l1 {:?}", l1);
        let mut b0 = second_it.next().unwrap();
        for b1 in second_it {
            let l2 = Line(b0,b1);
//            println!("l2 {:?}", l2);
            let intersec = l1.intersect(l2);
            match intersec {
                Some(p) => {
                    let cur_manhattan = p.manhattan_distance();
                    if cur_manhattan < min_distance {
                        min_distance = cur_manhattan;
                        closest_collision = p;
                    }
                },
                None => ()
            }
            b0 = b1;
        }
        a0 = a1;
    }
    if i64::max_value() != min_distance {
        println!("closest collision {:?}", closest_collision);
        println!("manhattan distance {}", min_distance);
    } else {
        println!("No collision found");
    }
}

