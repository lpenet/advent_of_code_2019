use std::fs;
use std::fmt;
use std::fs::File;
use std::io::BufWriter;

#[derive(Clone)]
struct Layer {
    buffer: Vec<u8>,
    rows: u32,
    cols: u32
}

impl Layer {
    fn create(src: &Vec<u8>, start: u32, rows: u32, cols: u32) -> Layer {
        let mut dst: Vec<u8> = vec![0 ;Layer::layer_size_static(rows,cols) as usize];
        let end = start+rows*cols;
        dst.clone_from_slice(&src[(start as usize)..(end as usize)]);
        Layer {
            buffer: dst,
            rows,
            cols
        }
    }

    fn layer_size_static(rows: u32, cols: u32) -> u32 {
       rows*cols
    }

    fn layer_size(&self) -> u32 {
        Layer::layer_size_static(self.rows, self.cols)
    }

    fn count_digit(&self, digit: u8) -> u32 {
        self.buffer.iter().map(|b| if *b == digit { 1 } else { 0 }).sum()
    }

    fn compose(&self, other: &Layer) -> Layer {
        let composed: Vec<u8> = self.buffer.iter().zip(other.buffer.iter())
            .map(|(first, second)| Layer::compose_pixels(*first,*second)).collect();
        Layer {
            buffer: composed,
            rows: self.rows,
            cols: self.cols
        }
    }

    fn compose_pixels(first: u8, second: u8) -> u8 {
        match first {
            0 => 0,
            1 => 1,
            _ => second
        }
    }

    fn get_pixel(&self, row: u32, col: u32) -> u8 {
        self.buffer[(row*self.cols + col) as usize]
    }

    fn output_image(&self, filename: &str) {
        let file = File::create(filename).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.cols, self.rows);
        encoder.set_color(png::ColorType::Grayscale);
        let mut writer = encoder.write_header().unwrap();
        let vec_255: Vec<u8> = self.buffer.iter().map(|b| b*255).collect();
        writer.write_image_data(&vec_255).unwrap();
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                res = res + &format!("{}", self.get_pixel(row,col));
            }
            res = res + "\n";
        }
        write!(f, "{}", res)
    }
}


fn main() {
    let rows = 6;
    let cols = 25;
    let layer_size = Layer::layer_size_static(rows, cols);
    let content: String = fs::read_to_string("input.txt")
        .expect("Error reading input");
    let content: Vec<u8> = content.as_bytes().iter().filter(|b| *b >= &('0' as u8) && *b <= &('9' as u8)).map(|b|(b - '0' as u8) as u8).collect();
    let len: u32 = content.len() as u32;
    let mut cur: u32 = 0;
    let mut layers: Vec<Layer> = Vec::<Layer>::new();
    while cur < len {
        layers.push(Layer::create(&content, cur, rows, cols));
        cur = cur + layer_size;
    }

    let mut min_val = std::u32::MAX;
    let mut min_index = std::usize::MAX;

    for (index, layer) in layers.iter().enumerate() {
        let zeros = layer.count_digit(0);
        if zeros < min_val {
            min_val = zeros;
            min_index = index;
        }
    }
    let one_time_two = layers[min_index].count_digit(1)*layers[min_index].count_digit(2);
    println!("{}", one_time_two);

    let mut it = layers.iter();
    let mut cur_layer: Layer = it.next().unwrap().clone();
    while let Some(next_layer) = it.next() {
        let new_layer = &cur_layer.compose(next_layer);
        cur_layer = new_layer.clone();
    }

    println!("{}", cur_layer);
    cur_layer.output_image("output.png");
}

