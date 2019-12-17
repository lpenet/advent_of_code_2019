extern crate termimage;
extern crate image;

use std::fs::File;
use std::io::{BufWriter, stdout};
use std::path::PathBuf;

use std::collections::HashMap;

mod intcode_computer;
use intcode_computer::Processor;
use intcode_computer::InputCallback;

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn do_move(&mut self, m: usize) {
        match m {
            1 => self.y = self.y + 1,
            2 => self.y = self.y - 1,
            3 => self.x = self.x - 1,
            4 => self.x = self.x + 1,
            _ => panic!("Unhandled mode")
        }
    }

    fn decode_move(&self, other: &Point) -> i64 {
        if (self.y - other.y) == -1 {
            return 1;
        }
        if (self.y - other.y) == 1 {
            return 2;
        }
        if (self.x - other.x) == 1 {
            return 3;
        }
        return 4;
    }
}

struct AreaInfo {
    nature: HashMap<Point,u8>,
    distance: HashMap<Point,u32>,
    robot_track: Vec<Point>,
    leak_distance: Option<u32>,
    complete: bool,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

static IMAGE_FILENAME: &str = "screen.png";

impl AreaInfo {
    fn new() -> AreaInfo {
        let mut ret = AreaInfo {
            nature: HashMap::<Point,u8>::new(),
            distance: HashMap::<Point,u32>::new(),
            robot_track: vec!(Point { x: 0, y :0 }),
            leak_distance: None,
            complete: false,
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0
        };
        ret.distance.insert(*ret.robot_track.last().unwrap(),0);
        ret.nature.insert(*ret.robot_track.last().unwrap(),1);
        ret
    }
    fn adjust_min_max(&mut self, point: &Point) {
        if point.x > self.max_x {
            self.max_x = point.x;
        }
        if point.x < self.min_x {
            self.min_x = point.x;
        }
        if point.y > self.max_y {
            self.max_y = point.y;
        }
        if point.y < self.min_y {
            self.min_y = point.y;
        }
    }

    fn compute_min_known_distance(&self, point: &Point) -> u32{
        let mut min_distance = u32::max_value();
        for m in 1..5 {
            let mut evaluated_pos = point.clone();
            evaluated_pos.do_move(m);
            if self.distance.contains_key(&evaluated_pos) {
                let nature = self.nature.get(&evaluated_pos);
                if nature.is_none() || *nature.unwrap() == 0 {
                    continue;
                }
                let evaluated_pos_distance = self.distance.get(&evaluated_pos);
                if *evaluated_pos_distance.unwrap() < min_distance {
                    min_distance = *evaluated_pos_distance.unwrap();
                }
            }
        }
        min_distance + 1
    }

    fn fix_area(&mut self, point: &Point) {
        let distance = self.distance.get(point).unwrap().clone();
        for m in 1..5 {
            let mut cur_point = point.clone();
            cur_point.do_move(m);
            if self.distance.contains_key(&cur_point) {
                let nature = self.nature.get(&cur_point);
                if nature.is_none() || *nature.unwrap() == 0 {
                    continue;
                }
                let cur_distance = self.distance.get(&cur_point).unwrap();
                if *cur_distance > distance+1 {
                    self.distance.insert(cur_point,distance+1);
                    self.fix_area(&cur_point);
                }
            }
        }
    }

    fn map_area(&mut self, program_filename: &str) {
        let mut processor = Processor::init(program_filename);
        while !self.complete {
            let output = processor.process(self).clone();
            match output {
                Some(0) => {
                    self.nature.insert(*self.robot_track.last().unwrap(),0);
                    self.robot_track.pop();
                },
                Some(1) => {
                    self.nature.insert(*self.robot_track.last().unwrap(),1);
                },
                Some(2) => {
                    self.nature.insert(*self.robot_track.last().unwrap(),2);
                    self.leak_distance = Some(*self.distance.get(self.robot_track.last().unwrap()).unwrap());
                },
                Some(_) => panic!("Unhandled output"),
                None => return ()
            }
            self.fix_area(&self.robot_track.last().unwrap().clone());
            self.output_image();
            std::thread::sleep(std::time::Duration::from_millis(100));
            print!("{}[2J", 27 as char);
            if self.leak_distance.is_some() {
                println!("Leak distance: {}", self.leak_distance.unwrap());
            } else {
                println!("Leak not found yet");
            }
            println!("Robot distance: {}", self.distance.get(self.robot_track.last().unwrap()).unwrap());
            AreaInfo::display_image();
        }
    }

    fn output_image(&mut self) {
        let file = File::create(IMAGE_FILENAME).unwrap();
        let ref mut w = BufWriter::new(file);
        let cols = self.max_x - self.min_x + 1;
        let rows = self.max_y - self.min_y + 1;

        let mut encoder = png::Encoder::new(w, cols as u32, rows as u32);
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        let mut vec: Vec<u8> = vec![0; (cols*rows*3) as usize];
        for (key,val) in self.nature.iter() {
            let r;
            let g;
            let b;
            match val {
                0 => { r = 255; g = 0; b = 0; },
                1 => { r = 0; g = 255; b = 0; },
                2 => { r = 0; g = 0; b = 255; }
                _ => panic!("Unhandled nature")
            }
            let base_index: usize = (((key.x - self.min_x) + ((key.y - self.min_y)*cols))*3) as usize;
            vec[base_index] = r;
            vec[base_index+1] = g;
            vec[base_index+2] = b;
        }
        for p in self.robot_track.iter() {
            let base_index: usize = (((p.x -self.min_x) + ((p.y - self.min_y)*cols))*3) as usize;
            vec[base_index] = 255;
            vec[base_index+1] = 255;
            vec[base_index+2] = 255;
        }
        writer.write_image_data(&vec).unwrap();
    }

    fn display_image() {
        let image_tuple = (String::new(), PathBuf::from(IMAGE_FILENAME));
        let format = termimage::ops::guess_format(&image_tuple).unwrap();
        let img = termimage::ops::load_image(&image_tuple, format).unwrap();
        termimage::ops::write_ansi_truecolor(&mut stdout(), &img);
    }
}

impl InputCallback for AreaInfo {
    fn callback(&mut self) -> i64 {
        let cur_pos = self.robot_track.last().unwrap().clone();
        for m in 1..5 {
            let mut new_pos = cur_pos.clone();
            new_pos.do_move(m);
            if !self.distance.contains_key(&new_pos) {
                let new_distance = self.compute_min_known_distance(&new_pos);
                if self.leak_distance.is_some() && new_distance > self.leak_distance.unwrap() {
                    continue;
                }
                self.distance.insert(new_pos, new_distance);
                self.robot_track.push(new_pos);
                self.adjust_min_max(&new_pos);
                return m as i64;
            }
        }
        // no result found
        self.robot_track.pop();
        match self.robot_track.last() {
            None => {
                self.complete = true;
                0 // dummy move
            },
            Some(back) => {
                cur_pos.decode_move(back) // backtrack
            }
        }
    }
}


fn main() {
    let mut area = AreaInfo::new();
    area.map_area("input.txt");
}
