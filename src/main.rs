use nes::rom::Rom;
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let path = env::args().nth(1).unwrap_or("sp.nes".to_string());
    let rom = Rom::load(&mut File::open(&Path::new(&path)).unwrap());
    match rom {
        Ok(_x) => println!("Load Done."),
        Err(x) => println!("Error: {:?}", x),
    }
}
