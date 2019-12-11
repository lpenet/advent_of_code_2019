use std::collections::HashSet;
use std::fs::File;
use std::io::BufWriter;

mod intcode_computer;
use intcode_computer::Processor;

#[derive(Copy,Clone)]
enum Direction {
    UP = 0,
    LEFT = 1,
    DOWN = 2,
    RIGHT = 3
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32
}

struct Robot {
    position: Point,
    direction: Direction,
    visited: HashSet<Point>,
    whites: HashSet<Point>
}

impl Robot {
    fn init() -> Robot {
        let mut ret = Robot {
            position: Point { x: 0,y :0},
            direction: Direction::UP,
            visited: HashSet::<Point>::new(),
            whites: HashSet::<Point>::new()
        };
        ret.visited.insert(ret.position);
        ret
    }

    fn get_position_color_code(&mut self) -> i64 {
        if self.whites.contains(&self.position) {
            return 1;
        } else {
            return 0;
        }
    }

    fn set_position_color_code(&mut self, color: i64) {
        if color == 1 {
            self.whites.insert(self.position);
        } else {
            self.whites.remove(&self.position);
        }
    }

    fn direction_to_i32(param: Direction) -> i32 {
        match param {
            Direction::UP => 0,
            Direction::LEFT => 1,
            Direction::DOWN => 2,
            Direction::RIGHT => 3
        }
    }

    fn i32_to_direction(param: i32) -> Direction {
        match param {
            0 => Direction::UP,
            1 => Direction::LEFT,
            2 => Direction::DOWN,
            3 => Direction::RIGHT,
            _ => panic!("Unsupported")
        }
    }

    fn turn_right(&mut self) {
        let mut new_direction = Robot::direction_to_i32(self.direction) - 1;
        if new_direction < 0 {
            new_direction = new_direction + 4;
        }
        self.direction = Robot::i32_to_direction(new_direction);
    }

    fn turn_left(&mut self) {
        let mut new_direction = Robot::direction_to_i32(self.direction) + 1;
        if new_direction > 3 {
            new_direction = new_direction - 4;
        }
        self.direction = Robot::i32_to_direction(new_direction);
    }

    fn advance(&mut self) {
        match self.direction {
            Direction::UP => self.position = Point { x: self.position.x, y: self.position.y +1 }, 
            Direction::LEFT => self.position = Point { x: self.position.x - 1, y: self.position.y }, 
            Direction::DOWN => self.position = Point { x: self.position.x, y: self.position.y -1 }, 
            Direction::RIGHT => self.position = Point { x: self.position.x + 1, y: self.position.y }
        }
        self.visited.insert(self.position);
    }

    fn rotate_and_move(&mut self, rotation: i64) {
        match rotation {
            0 => self.turn_left(),
            1 => self.turn_right(),
            _ => panic!("unhandled rotation type")
        }
        self.advance();
    }

    fn execute_program(&mut self, program_filename: &str) {
        let mut computer = Processor::init(program_filename);
        let mut cur_color = self.get_position_color_code();
        while let Some(new_color) = computer.process(&vec!(&self.get_position_color_code())) {
            self.set_position_color_code(new_color);
            let rotation_direction = computer.process(&vec!(&cur_color)).unwrap();
            self.rotate_and_move(rotation_direction);
            cur_color = self.get_position_color_code();
        }
    }

    fn output_image(&mut self, filename: &str) {
        let min_x = self.visited.iter().map(|v| v.x).min().unwrap();
        let max_x = self.visited.iter().map(|v| v.x).max().unwrap();
        let min_y = self.visited.iter().map(|v| v.y).min().unwrap();
        let max_y = self.visited.iter().map(|v| v.y).max().unwrap();
        
        let width: u32 = (max_x - min_x + 1) as u32;
        let height: u32 = (max_y - min_y + 1) as u32;

        let mut buffer: Vec<u8> = vec![0 ; (width*height) as usize];
        for p in &self.whites {
            buffer[(p.x-min_x+(height as i32 - p.y + min_y - 1)*width as i32) as usize] = 255;
        }

        let file = File::create(filename).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, width, height);
        encoder.set_color(png::ColorType::Grayscale);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&buffer).unwrap();
    }
}

fn part1() {
    let mut robot = Robot::init();
    robot.execute_program("input.txt");
    println!("Visited panes : {}", robot.visited.len());
}

fn part2() {
    let mut robot = Robot::init();
    robot.set_position_color_code(1);
    robot.execute_program("input.txt");
    let result_file_name = "result.png";
    robot.output_image(result_file_name);
    println!("Code is in {}", result_file_name);
}

fn main() {
    part1();
    part2();
}
