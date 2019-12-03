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

    pub fn length(self) -> i64 {
        (self.0.x - self.1.x).abs() + (self.0.y - self.1.y).abs()
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
    let mut min_wire_length = i64::max_value();
    let mut closest_collision = Point { x: 0, y: 0 };
    let mut length1: i64 = a0.manhattan_distance();
    for a1 in first_it {
        let mut second_it = second_coords.iter();
        let l1 = Line(a0, a1);
        length1 += l1.length();
        let mut b0 = second_it.next().unwrap();
        let mut length2: i64 = b0.manhattan_distance();
        for b1 in second_it {
            let l2 = Line(b0,b1);
            length2 += l2.length();
            let intersec = l1.intersect(l2);
            match intersec {
                Some(p) => {
                    let to_remove1 = Line(&p, l1.1);
                    let to_remove2 = Line(&p, l2.1);
                    let total_wire_length = length1 + length2 - to_remove1.length() - to_remove2.length();
                    if total_wire_length < min_wire_length {
                        min_wire_length=total_wire_length;
                        closest_collision = p;
                    }
                },
                None => ()
            }
            b0 = b1;
        }
        a0 = a1;
    }
    if i64::max_value() != min_wire_length {
        println!("closest collision by wire length {:?}", closest_collision);
        println!("wire length {}", min_wire_length);
    } else {
        println!("No collision found");
    }
}

