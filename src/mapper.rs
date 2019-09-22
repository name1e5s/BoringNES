use crate::rom::Rom;

pub trait Mapper {
    fn prg_load_b(&mut self, addr: u16) -> u8;
    fn prg_store_b(&mut self, addr: u16, value: u8);
    fn chr_load_b(&mut self, addr: u16) -> u8;
    fn chr_store_b(&mut self, addr: u16, value: u8);
}

pub struct Mapper0 {
    pub rom : Box<Rom>,
}

impl Mapper for Mapper0 {
    fn prg_load_b(&mut self, addr: u16) -> u8 {
        if addr < 0x8000 {
            0
        } else if self.rom.prg_rom.len() > 16 * 1024 {
            self.rom.prg_rom[(addr & 0x7FFF) as usize]
        } else {
            self.rom.prg_rom[(addr & 0x3FFF) as usize]
        }
    }

    // Store operation is not supported here
    fn prg_store_b(&mut self, _addr: u16, _value: u8) {}

    fn chr_load_b(&mut self, addr: u16) -> u8 {
        self.rom.chr_rom[addr as usize]
    }

    fn chr_store_b(&mut self, _addr: u16, _value: u8) {}
}