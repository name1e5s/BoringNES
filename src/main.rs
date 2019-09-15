use nes::rom::NESHeader;
use nes::utility::read_to_array;
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let mut header = [0u8; 16];
    let path = env::args().nth(1).unwrap_or("sp.nes".to_string());
    match read_to_array(&mut header, &mut File::open(&Path::new(&path)).unwrap()) {
        Ok(_x) => println!("Done"),
        _ => println!("Fuck"),
    }
    println!("File path: {:?}", path);
    println!("Header data: {:?}", header);
    let header_info = NESHeader::new(header);
    println!("Header info: {}", header_info);
}
