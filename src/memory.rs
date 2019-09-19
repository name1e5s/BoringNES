/// This file contains some basic abstractions
/// about memory of NES.
use std::cell::RefCell;
use std::rc::Rc;
use crate::mapper::*;

// Interface of the memory.
pub trait Memory {
    fn load_b(&mut self, addr: u16) -> u8;
    fn store_b(&mut self, addr: u16, value: u8);

    fn load_w(&mut self, addr: u16) -> u16 {
        self.load_b(addr) as u16 | ((self.load_b(addr + 1) as u16) << 8)
    }
    fn store_w(&mut self, addr: u16, value: u16) {
        self.store_b(addr, (value & 0xff) as u8);
        self.store_b(addr + 1, (value >> 8) as u8);
    }
}

// NES's internal 2KB memory
pub struct InternalRAM {
    pub data : [u8; 0x800],
}

impl Memory for InternalRAM {
    fn load_b(&mut self, addr: u16) -> u8 {
        self.data[(addr & 0x7FF) as usize]
    }

    fn store_b(&mut self, addr: u16, value: u8) {
        self.data[(addr & 0x7FF) as usize] = value
    }
}

// NES's full memory map. 
pub struct MemoryMap {
    pub ram : InternalRAM,
    pub mapper: Rc<RefCell<Box<dyn Mapper + Send>>>,
}

impl Memory for MemoryMap {
    fn load_b(&mut self, addr: u16) -> u8 {
        if addr < 0x2000 {
            self.ram.load_b(addr & 0x7FF)
        } else if addr < 0x4018 {
            panic!("Not implemented!")
        } else {
            let mut tmp = self.mapper.borrow_mut();
            tmp.prg_load_b(addr)
        }
    }

    fn store_b(&mut self, addr: u16, value: u8) {
        if addr < 0x2000 {
            self.ram.store_b(addr & 0x7FF, value)
        } else if addr < 0x4018 {
            panic!("Not implemented!")
        } else {
            let mut tmp = self.mapper.borrow_mut();
            tmp.prg_store_b(addr, value)
        }
    }
}