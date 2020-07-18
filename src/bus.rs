use crate::ppu::PPU;
use crate::{Mapper, CPU};
use std::cell::RefCell;
use std::rc::Rc;

// Bus Memory map
// -------------------------------------------------------------------------------------------------
// Address range |  Size   |  Device
// $0000-$07FF	 |  $0800  |  2KB internal RAM
// $0800-$0FFF	 |  $0800  |  Mirrors of $0000-$07FF
// $1000-$17FF	 |  $0800  |  Mirrors of $0000-$07FF
// $1800-$1FFF	 |  $0800  |  Mirrors of $0000-$07FF
// $2000-$2007	 |  $0008  |  NES PPU registers
// $2008-$3FFF	 |  $1FF8  |  Mirrors of $2000-2007 (repeats every 8 bytes)
// $4000-$4017	 |  $0018  |  NES APU and I/O registers
// $4018-$401F	 |  $0008  |  APU and I/O functionality that is normally disabled. See CPU Test Mode
// $4020-$FFFF	 |  $BFE0  |  Cartridge space: PRG ROM, PRG RAM, and mapper registers (See Note)
pub struct Bus<T: Mapper> {
    ram: [u8; 0x0800],
    mapper: T,
    cpu: Rc<RefCell<CPU<T>>>,
    ppu: Rc<RefCell<PPU<T>>>,
}

impl<T: Mapper> Bus<T> {
    pub fn new(mapper: T, cpu: Rc<RefCell<CPU<T>>>, ppu: Rc<RefCell<PPU<T>>>) -> Self {
        Self {
            ram: [0; 0x0800],
            mapper: mapper,
            cpu: cpu,
            ppu: ppu,
        }
    }
    pub fn read8(&self, addr: u16) -> u8 {
        match addr {
            0..=0x1fff => {
                let a = addr % 0x0800;
                self.ram[a as usize]
            }
            0x2000..=0x3fff => todo!(),
            0x4000..=0x4013 => 0,
            0x4014 => todo!(),
            0x4015 => todo!(),
            0x4016 => todo!(),
            0x4017 => todo!(),
            0x4018..=0x401f => 0, // normally disabled, maybe should return Err
            0x4020..=0xffff => self.mapper.read(addr),
        }
    }

    pub fn write8(&mut self, addr: u16, v: u8) {
        match addr {
            0..=0x1fff => {
                let a = addr % 0x0800;
                self.ram[a as usize] = v;
            }
            0x2000..=0x3fff => todo!(),
            0x4000..=0x4013 => (), //FIXME todo!(),
            0x4014 => todo!(),
            0x4015 => (), //FIXME todo!(),
            0x4016 => todo!(),
            0x4017 => todo!(),
            0x4018..=0x401f => todo!(),
            0x4020..=0xffff => self.mapper.write(addr, v),
        }
    }
}
