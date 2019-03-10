
use std::env;

use image_meta::load_from_file;


fn main() {
    for arg in env::args().skip(1) {
        let dim = load_from_file(&arg).unwrap().dimensions;
        println!("{}x{}", dim.width, dim.height);
    }
}
