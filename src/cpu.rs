use crate::memory::Memory;

pub struct Cpu {
    regs: [GPReg; 3],
    af: GPReg,
    sp: u16,
    pc: u16,
    memory: Memory,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            regs: [GPReg { hi: 0, lo: 0 }; 3],
            af: GPReg { hi: 0, lo: 0 },
            sp: 0x0000,
            pc: 0x0000,
            memory: Memory::new(),
        }
    }
    fn get_z(&self) -> bool {
        self.af.lo >> 7 == 1
    }
    fn get_n(&self) -> bool {
        (self.af.lo >> 6) & 0x1 == 1
    }
    fn get_h(&self) -> bool {
        (self.af.lo >> 5) & 0x1 == 1
    }
    fn get_c(&self) -> bool {
        (self.af.lo >> 4) & 0x1 == 1
    }
    fn get_a(&self) -> u8 {
        self.af.hi
    }
    fn get_f(&self) -> u8 {
        self.af.lo
    }
}

#[derive(Clone, Copy)]
struct GPReg {
    hi: u8,
    lo: u8,
}