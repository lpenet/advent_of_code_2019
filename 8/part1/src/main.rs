use std::fs;

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
}


fn main() {
    let rows = 25;
    let cols = 6;
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
}

