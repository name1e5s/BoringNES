use crate::memory::Memory;

const C_FLAG: u8 = 1 << 0;
const Z_FLAG: u8 = 1 << 1;
const I_FLAG: u8 = 1 << 2;
const D_FLAG: u8 = 1 << 3;
const B_FLAG: u8 = 1 << 4;
const O_FLAG: u8 = 1 << 6;
const N_FLAG: u8 = 1 << 7;

pub struct CPU<T: Memory>{
    pub cycle: u64,
    pub regs: Reg,
    pub mem: T,
}

struct Reg {
    a: u8,
    x: u8,
    y: u8,
    s: u8,
    p: u8,
    pc: u16,
}

impl Reg {
    fn new() -> Reg {
        Reg {
            a: 0,
            x: 0,
            y: 0,
            s: 0xfd,
            p: 0x24,
            pc: 0xc000,
        }
    }

    // Flag helpers
    fn get_flag(&mut self, flag: u8) -> bool {
        self.p & flag != 0
    }

    fn set_flag(&mut self, flag: u8, val: bool) {
        if val {
            self.p |= flag;
        } else {
            self.p &= !flag;
        }
    }

    fn get_flags(&mut self) -> u8 {
        self.p
    }

    fn set_flags(&mut self, value: u8) {
        self.p = value;
    }
}