use crate::instructions;
use crate::instructions::*;

#[allow(non_snake_case)]
pub struct CPU {
    PC: u16,
    SP: u8,
    A: u8,
    X: u8,
    Y: u8,
    status: u8,
    memory: [u8; 0x10000]
}

#[allow(dead_code)]
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
        print!("status = 0b{:08b}", self.status);
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
        if self.status & (1 << i) != 0 { true } else { false }
    }

    fn set_flag(&mut self, flag: Flags, value: bool) {
        let mask = 1u8 << (flag as u8);
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

    fn set_byte(&mut self, addr: u16, byte: u8) {
        self.memory[addr as usize] = byte;
    }

    fn fetch_and_decode(&self) -> DecodedOpcode {
        let (instruction, mode, _cycles) = OPCODES[self.get_byte(self.PC) as usize];
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
                Operand::Address(self.get_word(self.PC + 1) + self.X as u16),
            AddressingMode::AbsoluteY =>
                Operand::Address(self.get_word(self.PC + 1) + self.Y as u16),
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

        DecodedOpcode {instruction, operand, length: 1 + mode.operand_bytes() }
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
            }

            // LDX
            DecodedOpcode { instruction: Instruction::LDX, operand, length } => {
                let c = match operand {
                    Operand::Constant(c) => c,
                    Operand::Address(addr) => self.get_byte(addr),
                    _ => { panic!("Unknown operand type for LDX: {:?}", operand); }
                };

                self.set_flag(Flags::Z, c == 0);
                self.set_flag(Flags::N, (c & 0b10000000) != 0);
                self.X = c;

                self.PC += length;
            }

            // LDY
            DecodedOpcode { instruction: Instruction::LDY, operand, length } => {
                let c = match operand {
                    Operand::Constant(c) => c,
                    Operand::Address(addr) => self.get_byte(addr),
                    _ => { panic!("Unknown operand type for LDY: {:?}", operand); }
                };

                self.set_flag(Flags::Z, c == 0);
                self.set_flag(Flags::N, (c & 0b10000000) != 0);
                self.Y = c;

                self.PC += length;
            }

            // STA
            DecodedOpcode { instruction: Instruction::STA, operand, length} => {
                let addr = match operand {
                    Operand::Address(addr) => addr,
                    _ => { panic!("Unknown operand type for STA: {:?}", operand); }
                };

                self.memory[addr as usize] = self.A;

                self.PC += length;
            }

            // STX
            DecodedOpcode { instruction: Instruction::STX, operand, length } => {
                let addr = match operand {
                    Operand::Address(addr) => addr,
                    _ => { panic!("Unknown operand type for STX: {:?}", operand); }
                };

                self.memory[addr as usize] = self.X;

                self.PC += length;
            }

            // STY
            DecodedOpcode { instruction: Instruction::STY, operand, length } => {
                let addr = match operand {
                    Operand::Address(addr) => addr,
                    _ => { panic!("Unknown operand type for STY: {:?}", operand); }
                };

                self.memory[addr as usize] = self.Y;

                self.PC += length;
            }

            // TAX
            DecodedOpcode { instruction: Instruction::TAX, operand, length } => {
                match operand {
                    Operand::NoArg => {},
                    _ => { panic!("Unknown operand type for TAX: {:?}", operand); }
                }

                self.set_flag(Flags::Z, self.A == 0);
                self.set_flag(Flags::N, (self.A & 0b10000000) != 0);
                self.X = self.A;

                self.PC += length;
            }

            // TAY
            DecodedOpcode { instruction: Instruction::TAY, operand, length } => {
                match operand {
                    Operand::NoArg => {},
                    _ => { panic!("Unknown operand type for TAY: {:?}", operand); }
                }

                self.set_flag(Flags::Z, self.A == 0);
                self.set_flag(Flags::N, (self.A & 0b10000000) != 0);
                self.Y = self.A;

                self.PC += length;
            }

            // TXA
            DecodedOpcode { instruction: Instruction::TXA, operand, length } => {
                match operand {
                    Operand::NoArg => {},
                    _ => { panic!("Unknown operand type for TXA: {:?}", operand); }
                }

                self.set_flag(Flags::Z, self.X == 0);
                self.set_flag(Flags::N, (self.X & 0b10000000) != 0);
                self.A = self.X;

                self.PC += length;
            }

            // TYA
            DecodedOpcode { instruction: Instruction::TYA, operand, length } => {
                match operand {
                    Operand::NoArg => {},
                    _ => { panic!("Unknown operand type for TYA: {:?}", operand); }
                }

                self.set_flag(Flags::Z, self.Y == 0);
                self.set_flag(Flags::N, (self.Y & 0b10000000) != 0);
                self.A = self.Y;

                self.PC += length;
            }

            // DEX
            DecodedOpcode { instruction: Instruction::DEX, operand, length } => {
                match operand {
                    Operand::NoArg => {},
                    _ => { panic!("Unknown operand type for DEX: {:?}", operand); }
                }

                let c = self.X.wrapping_sub(1);
                self.set_flag(Flags::Z, c == 0);
                self.set_flag(Flags::N, (c & 0b10000000) != 0);
                self.X = c;

                self.PC += length;
            }

            // DEY
            DecodedOpcode { instruction: Instruction::DEY, operand, length } => {
                match operand {
                    Operand::NoArg => {},
                    _ => { panic!("Unknown operand type for DEY: {:?}", operand); }
                }

                let c = self.Y.wrapping_sub(1);
                self.set_flag(Flags::Z, c == 0);
                self.set_flag(Flags::N, (c & 0b10000000) != 0);
                self.Y = c;

                self.PC += length;
            }

            // DEC
            DecodedOpcode { instruction: Instruction::DEC, operand, length} => {
                let addr = match operand {
                    Operand::Address(addr) => addr,
                    _ => { panic!("Unknown operand type for DEC: {:?}", operand); }
                };
                let mut c = self.get_byte(addr);

                c = c.wrapping_sub(1);
                self.set_flag(Flags::Z, c == 0);
                self.set_flag(Flags::N, (c & 0b10000000) != 0);
                self.set_byte(addr, c);

                self.PC += length;
            }

            // INX
            DecodedOpcode { instruction: Instruction::INX, operand, length } => {
                match operand {
                    Operand::NoArg => {},
                    _ => { panic!("Unknown operand type for INX: {:?}", operand); }
                }

                let c = self.X.wrapping_add(1);
                self.set_flag(Flags::Z, c == 0);
                self.set_flag(Flags::N, (c & 0b10000000) != 0);
                self.X = c;

                self.PC += length;
            }

            // INY
            DecodedOpcode { instruction: Instruction::INY, operand, length } => {
                match operand {
                    Operand::NoArg => {},
                    _ => { panic!("Unknown operand type for INY: {:?}", operand); }
                }

                let c = self.Y.wrapping_add(1);
                self.set_flag(Flags::Z, c == 0);
                self.set_flag(Flags::N, (c & 0b10000000) != 0);
                self.Y = c;

                self.PC += length;
            }

            // INC
            DecodedOpcode { instruction: Instruction::INC, operand, length} => {
                let addr = match operand {
                    Operand::Address(addr) => addr,
                    _ => { panic!("Unknown operand type for INC: {:?}", operand); }
                };
                let mut c = self.get_byte(addr);

                c = c.wrapping_add(1);
                self.set_flag(Flags::Z, c == 0);
                self.set_flag(Flags::N, (c & 0b10000000) != 0);
                self.set_byte(addr, c);

                self.PC += length;
            }

            // TSX
            DecodedOpcode { instruction: Instruction::TSX, operand, length } => {
                match operand {
                    Operand::NoArg => {},
                    _ => { panic!("Unknown operand type for TSX: {:?}", operand); }
                }

                self.set_flag(Flags::Z, self.SP == 0);
                self.set_flag(Flags::N, (self.SP & 0b10000000) != 0);
                self.X = self.SP;

                self.PC += length;
            }

            // TXS
            DecodedOpcode { instruction: Instruction::TXS, operand, length } => {
                match operand {
                    Operand::NoArg => {},
                    _ => { panic!("Unknown operand type for TSX: {:?}", operand); }
                }

                self.SP = self.X;

                self.PC += length;
            }

            // CLC, CLD, CLI, CLV,
            DecodedOpcode { instruction: Instruction::CLC, operand, length }
            | DecodedOpcode { instruction: Instruction::CLD, operand, length }
            | DecodedOpcode { instruction: Instruction::CLI, operand, length }
            | DecodedOpcode { instruction: Instruction::CLV, operand, length }
            | DecodedOpcode { instruction: Instruction::SEC, operand, length }
            | DecodedOpcode { instruction: Instruction::SED, operand, length }
            | DecodedOpcode { instruction: Instruction::SEI, operand, length } => {
                let flag = match opcode.instruction {
                    Instruction::CLC | Instruction::SEC => Flags::C,
                    Instruction::CLD | Instruction::SED => Flags::D,
                    Instruction::CLI | Instruction::SEI => Flags::I,
                    Instruction::CLV => Flags::V,
                    _ => { panic!(); }
                };

                let value = match opcode.instruction {
                    Instruction::CLC | Instruction::CLD | Instruction::CLI | Instruction::CLV => false,
                    Instruction::SEC | Instruction::SED | Instruction::SEI => true,
                    _ => { panic!(); }
                };

                self.set_flag(flag, value);

                self.PC += length;
            }

            // JMP
            DecodedOpcode { instruction: Instruction::JMP, operand, .. } => {
                let addr = match operand {
                    Operand::Address(addr) => addr,
                    _ => { panic!("Unknown operand type for JMP: {:?}", operand); }
                };

                self.PC = addr;
            }

            _ => println!("Unknown opcode {:?}", opcode)
        }
    }

    pub fn load_at(&mut self, at: usize, data: &[u8]) {
        for (i, byte) in data.into_iter().enumerate() {
            self.memory[at + i] = *byte;
        }
    }

    pub fn new() -> CPU {
        CPU { PC: 0x600, SP: 0xff, A: 0, X: 0, Y: 0, status: 0b00100000, memory: [0; 0x10000] }
    }
}

mod test;