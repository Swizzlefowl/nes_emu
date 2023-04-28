use super::{address_mode::AddressingMode, cpu_6502::CPU, util::nth_bit};

impl CPU {
    // load the contents of register a into register y
    // and set the flags approprietly
    pub fn tay(&mut self, _____________________addr: &AddressingMode) {
        self.register_y = self.register_a;
        self.status.set_zero(self.register_y == 0);
        self.status.set_negative(nth_bit(self.register_y, 7));
    }

    // https://www.nesdev.org/obelisk-6502-guide/reference.html#JMP
    pub fn jmp(&mut self, addr: &AddressingMode) {
        self.pc = addr.fetch_argument_address(self);
    }

    //JSR - Jump to Subroutine
    pub fn jsr(&mut self, addr: &AddressingMode) {
        self.push_word(self.pc + addr.len() - 1);
        self.pc = addr.fetch_argument_address(self);
    }

    //LDX - Load X Register
    pub fn ldx(&mut self, addr: &AddressingMode) {
        self.register_x = addr.fetch_argument(self);
        self.status.set_zero(self.register_x == 0);
        self.status.set_negative(nth_bit(self.register_x, 7))
    }

    pub fn lda(&mut self, addr: &AddressingMode) {
        self.register_a = self.ram[addr.fetch_argument_address(self) as usize];
        self.status.set_zero(self.register_a == 0);
        self.status.set_negative(nth_bit(self.register_x, 7));
        println!(
            "arg:: {}",
            self.ram[addr.fetch_argument_address(self) as usize]
        )
    }

    pub fn and(&mut self, addr: &AddressingMode) {
        self.register_a &= addr.fetch_argument(self);
        self.status.set_zero(self.register_a == 0);
        self.status.set_negative(nth_bit(self.register_x, 7));
        println!(
            "arg:: {}",
            self.ram[addr.fetch_argument_address(self) as usize]
        )
    }
    // CLC - Clear Carry Flag
    pub fn clc(&mut self, addr: &AddressingMode) {
        self.status.set_carry(false);
    }

    // ADC - Add Memory to Accumulator with Carry
    pub fn adc(&mut self, addr: &AddressingMode) {
        let (data, overflow) = self.register_a.overflowing_add(addr.fetch_argument(self));
        self.status.set_carry(overflow);
        self.status.set_overflow(overflow);
        self.status.set_negative(nth_bit(data, 7));
        self.register_a = data
    }

    //STA - Store Accumulator in Memory
    pub fn sta(&mut self, addr: &AddressingMode) {
        self.ram[addr.fetch_argument_address(self) as usize] = self.register_a;
    }

    //RTS - Return From Subroutme
    pub fn rts(&mut self, addr: &AddressingMode) {
        self.pc = self.pop_word();
        self.pc += 1;
    }

    //SEC - Set Carry Flag
     pub fn sec(&mut self, addr: &AddressingMode) {
        self.status.set_carry(true);
     }
     //ASL - Arithmetic Shift Left
     

    //NOP - No Operation
    pub fn nop(&mut self, addr: &AddressingMode) {}

    pub fn brk(&mut self, addr: &AddressingMode) {}
}

pub struct Instruction {
    pub name: &'static str,
    pub function: fn(&mut CPU, &AddressingMode),
    pub mode: AddressingMode,
}

impl Instruction {
    pub fn new(
        name: &'static str,
        function: fn(&mut CPU, &AddressingMode),
        mode: AddressingMode,
    ) -> Self {
        Self {
            name,
            function,
            mode,
        }
    }
}

pub fn decode(opcode: u8) -> Instruction {
    match opcode {
        0xEA => Instruction::new("NOP", CPU::nop, AddressingMode::Implied),
        0x00 => Instruction::new("BRK", CPU::brk, AddressingMode::Implied),
        0x29 => Instruction::new("AND", CPU::and, AddressingMode::Immediate),
        0x18 => Instruction::new("CLC", CPU::clc, AddressingMode::Implied),
        0x38 => Instruction::new("SEC", CPU::sec, AddressingMode::Implied),
        0x4C => Instruction::new("JMP", CPU::jmp, AddressingMode::Absolute),
        0x6C => Instruction::new("JMP", CPU::jmp, AddressingMode::Indirect),
        0x20 => Instruction::new("JSR", CPU::jsr, AddressingMode::Absolute),
        0x60 => Instruction::new("RTS", CPU::rts, AddressingMode::Implied),

        0x69 => Instruction::new("ADC", CPU::adc, AddressingMode::Immediate),
        0x65 => Instruction::new("ADC", CPU::adc, AddressingMode::ZeroPage),
        0x75 => Instruction::new("ADC", CPU::adc, AddressingMode::ZeroPageX),
        0x6D => Instruction::new("ADC", CPU::adc, AddressingMode::Absolute),
        0x7D => Instruction::new("ADC", CPU::adc, AddressingMode::AbsoluteX),
        0x79 => Instruction::new("ADC", CPU::adc, AddressingMode::AbsoluteY),
        0x61 => Instruction::new("ADC", CPU::adc, AddressingMode::IndirectX),
        0x71 => Instruction::new("ADC", CPU::adc, AddressingMode::IndirectY),

         0x85 => Instruction::new("STA", CPU::sta,  AddressingMode::ZeroPage),
         0x95 => Instruction::new("STA", CPU::sta,  AddressingMode::ZeroPageX),
         0x8D => Instruction::new("STA", CPU::sta,  AddressingMode::Absolute),
         0x9D => Instruction::new("STA", CPU::sta,  AddressingMode::AbsoluteX),
         0x99 => Instruction::new("STA", CPU::sta,  AddressingMode::AbsoluteY),
         0x81 => Instruction::new("STA", CPU::sta,  AddressingMode::IndirectX),
         0x91 => Instruction::new("STA", CPU::sta,  AddressingMode::IndirectY),

        0x2A => Instruction::new("LDX", CPU::ldx, AddressingMode::Immediate),
        0xA6 => Instruction::new("LDX", CPU::ldx, AddressingMode::ZeroPage),
        0xB6 => Instruction::new("LDX", CPU::ldx, AddressingMode::ZeroPage),
        0xAE => Instruction::new("LDX", CPU::ldx, AddressingMode::Absolute),
        0xBE => Instruction::new("LDX", CPU::ldx, AddressingMode::AbsoluteY),

        0xA9 => Instruction::new("LDA", CPU::lda, AddressingMode::Immediate),
        0xA5 => Instruction::new("LDA", CPU::lda, AddressingMode::ZeroPage),
        0xB5 => Instruction::new("LDA", CPU::lda, AddressingMode::ZeroPageX),
        0xAD => Instruction::new("LDA", CPU::lda, AddressingMode::Absolute),
        0xDB => Instruction::new("LDA", CPU::lda, AddressingMode::AbsoluteX),
        0xD9 => Instruction::new("LDA", CPU::lda, AddressingMode::AbsoluteY),
        0xA1 => Instruction::new("LDA", CPU::lda, AddressingMode::IndirectX),
        0xB1 => Instruction::new("LDA", CPU::lda, AddressingMode::IndirectY),

        _ => unimplemented!("{opcode:2x}"),
    }
}
