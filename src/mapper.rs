pub trait Mapper {
    fn prg_load_b(&mut self, addr: u16) -> u8;
    fn prg_store_b(&mut self, addr: u16, value: u8);
    fn chr_load_b(&mut self, addr: u16) -> u8;
    fn chr_store_b(&mut self, addr: u16, value: u8);
}