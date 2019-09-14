use nes::utility::read_to_array;
use std::fs::File;
use std::path::Path;
use std::env;

fn main() {
    let mut header = [0u8; 16];
    let path = env::args().nth(1).unwrap_or("sp.nes".to_string());
    match read_to_array(&mut header, &mut File::open(&Path::new(&path)).unwrap()) {
        Ok(_x) => {
            println!("Done")
        }
        _ => println!("Fuck")
    }
    println!("File path: {:?}", path);
    println!("Header data: {:?}", header);
}
