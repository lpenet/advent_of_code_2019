use std::collections::HashSet;

mod intcode_computer;
use intcode_computer::Processor;

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
struct TileCoords {
    values: [i32; 2]
}

struct StageInfo {
    empty: HashSet<TileCoords>,
    walls: HashSet<TileCoords>,
    blocks: HashSet<TileCoords>,
    horizontal_paddle: HashSet<TileCoords>,
    ball: HashSet<TileCoords>
}

impl StageInfo {
    fn load(program_filename: &str) -> StageInfo {
        let mut ret = StageInfo {
            empty: HashSet::<TileCoords>::new(), 
            walls: HashSet::<TileCoords>::new(), 
            blocks: HashSet::<TileCoords>::new(),
            horizontal_paddle: HashSet::<TileCoords>::new(),
            ball: HashSet::<TileCoords>::new() 
        };
        let mut computer = Processor::init(program_filename);
        let mut infos:[i32; 3] = [0; 3];
        let mut cur_index = 0;
        while let Some(output) = computer.process(&vec!()) {
            infos[cur_index] = output as i32;
            cur_index = cur_index + 1;
            if cur_index == 3 {
                cur_index = 0;
                ret.add_tile(&infos[0..2], infos[2]);
            }
        }
        ret
    }

    fn add_tile(&mut self,coords_param: &[i32], tile_type: i32) {
        let mut coords: TileCoords = Default::default();
        coords.values.copy_from_slice(coords_param);
        self.empty.remove(&coords);
        self.walls.remove(&coords);
        self.blocks.remove(&coords);
        self.horizontal_paddle.remove(&coords);
        self.ball.remove(&coords);

        match tile_type {
            0 => self.empty.insert(coords),
            1 => self.walls.insert(coords),
            2 => self.blocks.insert(coords),
            3 => self.horizontal_paddle.insert(coords),
            4 => self.ball.insert(coords),
            _ => panic!("Unhandled tile type ")
        };
    }
}

fn part1() {
    let stage_info = StageInfo::load("input.txt");
    let block_tile_count = stage_info.blocks.len();
    println!("Block tiles: {}", block_tile_count);
}

fn main() {
    part1();
}
