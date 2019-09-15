use crate::utility::read_to_array;
/// Load ines file into `ROM` struct.
/// See: http://wiki.nesdev.com/w/index.php/INES
use std::fmt;
use std::io::{self, Read};

pub struct NESHeader {
    /// "NES\x1a"
    pub magic: [u8; 4],
    /// Size of PRG ROM in 16KB units
    pub prg_rom_size: u8,
    /// Size of CHR ROM in 8 KB units
    pub chr_rom_size: u8,
    /// Flags 6 - Mapper, mirroring, battery, trainer
    pub flags_6: u8,
    /// Flags 7 - Mapper, VS/Playchoice, NES 2.0
    pub flags_7: u8,
    /// Flags 8 - PRG-RAM size (rarely used extension)
    pub prg_ram_size: u8,
    /// Flags 9 - TV system (rarely used extension)
    pub flags_9: u8,
    /// Flags 10 - TV system, PRG-RAM presence (unofficial, rarely used extension)
    pub flags_10: u8,
    /// Unused padding
    pub unused_padding: [u8; 5],
}

impl NESHeader {
    /// Build a new iNES Header from array.
    pub fn new(arr: [u8; 16]) -> Self {
        NESHeader {
            magic: [arr[0], arr[1], arr[2], arr[3]],
            prg_rom_size: arr[4],
            chr_rom_size: arr[5],
            flags_6: arr[6],
            flags_7: arr[7],
            prg_ram_size: arr[8],
            flags_9: arr[9],
            flags_10: arr[10],
            unused_padding: [0; 5],
        }
    }

    /// Return the Mapper ID of the NES rom.
    fn mapper_id(&self) -> u8 {
        ((self.flags_6 & 0xF0) >> 4) | (self.flags_7 & 0xF0)
    }
}

impl fmt::Display for NESHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            " PRG-ROM {} KB, CHR-ROM {} KB, MAPPER ID {}",
            self.prg_rom_size * 16,
            self.chr_rom_size * 8,
            self.mapper_id()
        )
    }
}

pub struct Rom {
    pub header: NESHeader,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

#[derive(Debug)]
pub enum LoadRomError {
    /// Error in IO.
    IOError(io::Error),
    /// Error in iNES header
    NESFormatError,
}

impl From<io::Error> for LoadRomError {
    fn from(err: io::Error) -> Self {
        LoadRomError::IOError(err)
    }
}

impl Rom {
    pub fn load(r: &mut dyn Read) -> Result<Rom, LoadRomError> {
        let mut header_arr = [0u8; 16];
        read_to_array(&mut header_arr, r)?;
        let header = NESHeader::new(header_arr);

        if header.magic != *b"NES\x1a" {
            println!("Format ERROR!");
            Err(LoadRomError::NESFormatError)
        } else {
            println!("Header info: {}", header);
            let mut prg_rom = vec![0u8; header.prg_rom_size as usize * 16384];
            let mut chr_rom = vec![0u8; header.chr_rom_size as usize * 8192];
            read_to_array(&mut prg_rom, r)?;
            read_to_array(&mut chr_rom, r)?;
            Ok(Rom {
                header: header,
                prg_rom: prg_rom,
                chr_rom: chr_rom,
            })
        }
    }
}
