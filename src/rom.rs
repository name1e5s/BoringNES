/// Load ines file into `ROM` struct.
/// See: http://wiki.nesdev.com/w/index.php/INES
use std::fmt;

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
