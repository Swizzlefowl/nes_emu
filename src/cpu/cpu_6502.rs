//! https://www.pagetable.com/c64ref/6502/?tab=2
//! https://github.com/lukexor/tetanes

use crate::cpu::instructions;

use super::flags::Status;

const RAM_SIZE: usize = 0xFFFF;
const ROM_START: usize = 0x4020;
#[derive(Debug)]
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: Status,
    pub pc: u16,
    pub sp: u8,
    pub ram: [u8; RAM_SIZE]
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: Status::default()
                .with_interrupt_disable(true)
                .with_carry(true),
            pc: 0x600,
            sp: 0xFD,
            ram: [0; RAM_SIZE]
        }
    }

    pub fn reset(&mut self) {
        self.sp -= 3;
        self.register_x = 0;
        self.register_y = 0;
        self.register_a = 0;
    }

    pub fn load(&mut self, rom: &[u8]) {
        self.ram[0x600..0x600 + rom.len()].copy_from_slice(rom);
    }

    pub fn tick(&mut self) {
        let instr = instructions::decode(self.ram[self.pc as usize]);
        println!(
            "{:#04x}: A {:#x}, X {:#x}, Y {:#x}, {}",
            self.pc, self.register_a, self.register_x, self.register_y, instr.name
        );
        (instr.function)(self, &instr.mode);
        self.pc += instr.mode.len();
    }

    /// pop a byte from the stack
     pub fn pop_byte(&mut self) -> u8 {
        self.sp += 1;
        let data = self.ram[self.sp as usize];
        data
    }

    /// pop a word from the stack
    pub fn pop_word(&mut self) -> u16 {
        u16::from_le_bytes([self.pop_byte(), self.pop_byte()])
    }

    /// push a byte into the stack
    pub fn push_byte(&mut self, val: u8) {
        self.ram[self.sp as usize] = val;
        self.sp -= 1;
    }

    ///push a word into the stack
    pub fn push_word(&mut self, val: u16) {
        for byte in u16::to_be_bytes(val) {
            self.push_byte(byte);
        }
    }

    /// Read a u8 from the CPU's memory
    pub fn read_byte(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    /// Read a u16 from the CPU's memory
    pub fn read_word(&self, addr: u16) -> u16 {
        u16::from_le_bytes([self.read_byte(addr), self.read_byte(addr + 1)])
    }
}
