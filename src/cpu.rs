use crate::instructions::*;

pub struct CPU {
    PC: u16,
    SP: u8,
    A: u8,
    X: u8,
    Y: u8,
    status: u8,
    memory: [u8; 0x10000]
}

enum Flags {
    C = 0, // Carry flag
    Z = 1, // Zero flag
    I = 2, // Interrupt mask
    D = 3, // BCD flag
    B = 4, // Break flag
    S = 5, // Stub flag (always set to 1)
    V = 6, // Overflow flag
    N = 7  // Negative flag
}

#[derive(Debug)]
struct DecodedOpcode {
    instruction: Instruction,
    operand: Operand,
    length: u16
}

#[derive(Debug)]
enum Operand {
    NoArg,
    Accumulator,
    Constant(u8),
    Address(u16)
}

impl CPU {
    pub fn print(&self) {
        print!("CPU {{ ");
        print!("A = 0x{:02x}, ", self.A);
        print!("X = 0x{:02x}, ", self.X);
        print!("Y = 0x{:02x}, ", self.Y);

        print!("PC = 0x{:04x}, ", self.PC);
        print!("SP = 0x{:02x}, ", self.SP);
        print!("status = 0x{:08b}", self.status);
        println!(" }} ");
    }

    pub fn print_memory(&self) {
        for i in 0..=0xff {
            print!("0x{:2x}00: ", i);
            for j in 0..=0xff {
                print!("{:2x} ", self.memory[i * 0x100 + j]);
            }
            println!();
        }
    }

    fn get_flag(&self, flag: Flags) -> bool {
        let i = flag as u8;
        if self.status & (1 << i) >> i != 0 { true } else { false }
    }

    fn set_flag(&mut self, flag: Flags, value: bool) {
        let mask = (value as u8) << (flag as u8);
        if value {
            self.status |= mask;
        } else {
            self.status &= !mask;
        }
    }

    fn get_byte(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn get_byte_as_i16(&self, addr: u16) -> i16 {
        (self.memory[addr as usize] as i8) as i16
    }

    fn get_word(&self, addr: u16) -> u16 {
        ((self.memory[addr as usize + 1] as u16) << 8) + (self.memory[addr as usize] as u16)
    }

    fn fetch_and_decode(&self) -> DecodedOpcode {
        let (instruction, mode) = OPCODES[self.get_byte(self.PC) as usize];
        let operand =  match mode {
            AddressingMode::Implied => Operand::NoArg,
            AddressingMode::Accumulator => Operand::Accumulator,
            AddressingMode::Immediate =>
                Operand::Constant(self.get_byte(self.PC + 1)),
            AddressingMode::ZeroPage =>
                Operand::Address(self.get_byte(self.PC + 1) as u16),
            AddressingMode::ZeroPageX =>
                Operand::Address(self.get_byte(self.PC + 1).wrapping_add(self.X) as u16),
            AddressingMode::ZeroPageY =>
                Operand::Address(self.get_byte(self.PC + 1).wrapping_add(self.Y) as u16),
            AddressingMode::Relative => {
                let addr = self.PC.checked_add_signed(self.get_byte_as_i16(self.PC + 1)).unwrap();
                Operand::Address(addr)
            },
            AddressingMode::Absolute =>
                Operand::Address(self.get_word(self.PC + 1)),
            AddressingMode::AbsoluteX =>
                Operand::Address(self.get_word(self.PC) + self.X as u16),
            AddressingMode::AbsoluteY =>
                Operand::Address(self.get_word(self.PC) + self.Y as u16),
            AddressingMode::Indirect =>
                Operand::Address(self.get_word(self.get_word(self.PC + 1))),
            AddressingMode::IndirectX => {
                let addr = self.get_byte(self.PC + 1).wrapping_add(self.X) as u16;
                Operand::Address(addr)
            },
            AddressingMode::IndirectY => {
                let addr = self.get_word(self.PC + 1) + self.Y as u16;
                Operand::Address(addr)
            }
        };

        DecodedOpcode {instruction: instruction, operand: operand, length: 1 + mode.operand_bytes() }
    }

    pub fn execute(&mut self) {
        let opcode = self.fetch_and_decode();
        match opcode {
            // LDA
            DecodedOpcode { instruction: Instruction::LDA, operand, length } => {
                let c = match operand {
                    Operand::Constant(c) => c,
                    Operand::Address(addr) => self.get_byte(addr),
                    _ => { panic!("Unknown operand type for LDA: {:?}", operand); }
                };

                self.set_flag(Flags::Z, c == 0);
                self.set_flag(Flags::N, (c & 0b10000000) != 0);
                self.A = c;

                self.PC += length;
            },

            // STA
            DecodedOpcode { instruction: Instruction::STA, operand, length} => {
                let addr = match operand {
                    Operand::Address(addr) => addr,
                    _ => { panic!("Unknown operand type for STA: {:?}", operand); }
                };

                self.memory[addr as usize] = self.A;

                self.PC += length;
            }
            _ => println!("Unknown opcode {:?}", opcode)
        }
    }

    pub fn load(&mut self, data: &[u8]) {
        for (i, byte) in data.into_iter().enumerate() {
            self.memory[0x0600 + i] = *byte;
        }
    }

    pub fn new() -> CPU {
        CPU { PC: 0x600, SP: 0xff, A: 0, X: 0, Y: 0, status: 0b00100000, memory: [0; 0x10000] }
    }
}