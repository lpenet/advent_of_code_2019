extern crate termimage;
extern crate image;

use std::fs::File;
use std::io::{BufWriter, stdin, stdout};
use std::path::PathBuf;
use std::collections::HashSet;

mod intcode_computer;
use intcode_computer::Processor;

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
struct TileCoords {
    values: [i32; 2]
}

struct StageInfo {
    processor: Processor,
    empty: HashSet<TileCoords>,
    walls: HashSet<TileCoords>,
    blocks: HashSet<TileCoords>,
    horizontal_paddle: HashSet<TileCoords>,
    ball: HashSet<TileCoords>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    score: i32
}

static IMAGE_FILENAME: &str = "screen.png";

impl StageInfo {

    fn load(program_filename: &str) -> StageInfo {
        StageInfo {
            empty: HashSet::<TileCoords>::new(), 
            walls: HashSet::<TileCoords>::new(), 
            blocks: HashSet::<TileCoords>::new(),
            horizontal_paddle: HashSet::<TileCoords>::new(),
            ball: HashSet::<TileCoords>::new(),
            processor: Processor::init(program_filename),
            min_x: i32::max_value(),
            max_x: i32::min_value(),
            min_y: i32::max_value(),
            max_y: i32::min_value(),
            score: 0
        }
    }

    fn fill_stage_from_processor(&mut self) {
        let mut infos:[i32; 3] = [0; 3];
        let mut cur_index = 0;
        while let Some(output) = self.processor.process(&|| -> i64 { 0 } ) {
            infos[cur_index] = output as i32;
            cur_index = cur_index + 1;
            if cur_index == 3 {
                cur_index = 0;
                if infos[0] == -1 && infos[1] == 0 {
                    self.score = infos[2]
                } else {
                    self.add_tile(&infos[0..2], infos[2]);
                }
            }
        }
    }

    fn add_tile(&mut self,coords_param: &[i32], tile_type: i32) {
        let mut coords: TileCoords = Default::default();
        coords.values.copy_from_slice(coords_param);
        self.empty.remove(&coords);
        self.walls.remove(&coords);
        self.blocks.remove(&coords);
        self.horizontal_paddle.remove(&coords);
        self.ball.remove(&coords);
        if coords_param[0] > self.max_x {
            self.max_x = coords_param[0];
        }
        if coords_param[0] < self.min_x {
            self.min_x = coords_param[0];
        }
        if coords_param[1] > self.max_y {
            self.max_y = coords_param[1];
        }
        if coords_param[1] < self.min_y {
            self.min_y = coords_param[1];
        }

        match tile_type {
            0 => self.empty.insert(coords),
            1 => self.walls.insert(coords),
            2 => self.blocks.insert(coords),
            3 => self.horizontal_paddle.insert(coords),
            4 => self.ball.insert(coords),
            _ => panic!("Unhandled tile type ")
        };
    }

    fn insert_coins(&mut self) {
        self.processor.memory[0] = 2;
        self.processor.reset_pointer();
    }

    unsafe fn play(&mut self, auto: bool) {
        let mut infos:[i32; 3] = [0; 3];
        let mut cur_index = 0;
        let mut score_outputed = false;
        let callback: &dyn Fn() -> i64;
        if auto {
            callback = &auto_value;
        } else {
            callback = &ask_value;
        }
        while let Some(output) = self.processor.process(callback) {
            infos[cur_index] = output as i32;
            cur_index = cur_index + 1;
            if cur_index == 3 {
                cur_index = 0;
                if infos[0] == -1 && infos[1] == 0 {
                    self.score = infos[2];
                    score_outputed = true;
                } else {
                    self.add_tile(&infos[0..2], infos[2]);
                }
                if score_outputed && !auto{
                    self.output_image();
                    print!("{}[2J", 27 as char);
                    println!("Score: {}", self.score);
                    StageInfo::display_image();
                }
                //BEEEEEEEEEERK
                let ball = self.ball.iter().next();
                if ball.is_some() {
                    BALL_X = ball.unwrap().values[0];
                }
                let paddle = self.horizontal_paddle.iter().next();
                if paddle.is_some() {
                    PADDLE_X = paddle.unwrap().values[0];
                }
            }
        }
        println!("Final score: {}", self.score);
    }

    fn paint_hashset(hash_set: &HashSet<TileCoords>, vec: &mut Vec<u8>, cols: i32, r: u8, g: u8, b: u8) {
        for cur in hash_set {
            let base_index: usize = ((cur.values[0] + (cur.values[1]*cols))*3) as usize;
            vec[base_index] = r;
            vec[base_index+1] = g;
            vec[base_index+2] = b;
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
        StageInfo::paint_hashset(&self.empty, &mut vec, cols, 0, 0, 0);       
        StageInfo::paint_hashset(&self.walls, &mut vec, cols, 191, 85, 33);       
        StageInfo::paint_hashset(&self.blocks, &mut vec, cols, 252, 255, 15);       
        StageInfo::paint_hashset(&self.horizontal_paddle, &mut vec, cols, 0, 255, 0);       
        StageInfo::paint_hashset(&self.ball, &mut vec, cols, 0, 0, 255);       
        writer.write_image_data(&vec).unwrap();
    }

    fn display_image() {
        let image_tuple = (String::new(), PathBuf::from(IMAGE_FILENAME));
        let format = termimage::ops::guess_format(&image_tuple).unwrap();
        let img = termimage::ops::load_image(&image_tuple, format).unwrap();
        termimage::ops::write_ansi_truecolor(&mut stdout(), &img);
    }

}

fn part1() {
    let mut stage_info = StageInfo::load("input.txt");
    stage_info.fill_stage_from_processor();
    let block_tile_count = stage_info.blocks.len();
    println!("Block tiles: {}", block_tile_count);
}

// Very dirty, as I do not know how to do that cleanly in RUST
fn ask_value() -> i64 {
    let mut s = String::new();
    let user_input: i64;
    stdin().read_line(&mut s).expect("Could not read string");
    if let Some('\n')=s.chars().next_back() {
       s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    let parsed = s.parse::<i64>();
    match parsed {
        Ok(val) => {
            match val {
                4 => user_input = -1,
                6 => user_input = 1,
                _ => user_input = 0
            }
        },
        Err(_) => user_input = 0
    }
    user_input
}

static mut BALL_X: i32 = 0;
static mut PADDLE_X: i32 = 0;

fn auto_value() -> i64 {
    unsafe {
        if BALL_X > PADDLE_X {
            return 1;
        } else if BALL_X < PADDLE_X {
            return -1;
        } else {
            return 0;
        }
    }
}

unsafe fn part2() {
    let mut stage_info = StageInfo::load("input.txt");
    stage_info.fill_stage_from_processor();
    stage_info.insert_coins();
    // pass false to interactively play...pong
    stage_info.play(true);
}

fn main() {
    part1();
    unsafe {
        part2();
    }
}
