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
                    _ => { panic!("Unknown operand type for DET: {:?}", operand); }
                }

                let c = self.Y.wrapping_sub(1);
                self.set_flag(Flags::Z, c == 0);
                self.set_flag(Flags::N, (c & 0b10000000) != 0);
                self.Y = c;

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

            // JMP
            DecodedOpcode { instruction: Instruction::JMP, operand, .. } => {
                let addr = match operand {
                    Operand::Address(addr) => addr,
                    _ => { panic!("Unknown operand type for STA: {:?}", operand); }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set_flag() {
        let mut cpu = CPU::new();
        cpu.set_flag(Flags::S, false);
        assert!(!cpu.get_flag(Flags::S));
        cpu.set_flag(Flags::S, true);
        assert!(cpu.get_flag(Flags::S));
    }

    // TODO: cover LDA by unit tests
    // TODO: cover STA by unit tests

    // LDX
    #[test]
    fn test_ldx_immediate() {
        let mut cpu = CPU::new();
        cpu.load_at(0x600, &[0xa2, 0xff]);
        cpu.execute();
        assert_eq!(cpu.X, 0xff);
        assert_eq!(cpu.status, 0b10100000);
    }

    #[test]
    fn test_ldx_zeropage() {
        let mut cpu = CPU::new();
        cpu.load_at(0, &[0xa6, 0x02, 0xde]);
        cpu.PC = 0;
        cpu.execute();
        assert_eq!(cpu.X, 0xde);
        assert_eq!(cpu.status, 0b10100000);
    }

    #[test]
    fn test_ldx_zeropage_y() {
        let mut cpu = CPU::new();
        cpu.load_at(0, &[0xb6, 0x02, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x14]);
        cpu.PC = 0;
        cpu.Y = 0x8;
        cpu.execute();
        assert_eq!(cpu.X, 0x14);
        assert_eq!(cpu.status, 0b00100000);
    }

    #[test]
    fn test_ldx_absolute() {
        let mut cpu = CPU::new();
        cpu.load_at(0x600, &[0xae, 0x33, 0x20]);
        cpu.PC = 0x600;
        cpu.memory[0x2033] = 0xef;
        cpu.execute();
        assert_eq!(cpu.X, 0xef);
        assert_eq!(cpu.status, 0b10100000);
    }

    #[test]
    fn test_ldx_absolute_y() {
        let mut cpu = CPU::new();
        cpu.load_at(0x600, &[0xbe, 0x33, 0x20]);
        cpu.PC = 0x600;
        cpu.Y = 0x10;
        cpu.memory[0x2033] = 0xef;
        cpu.memory[0x2043] = 0x14;
        cpu.execute();
        assert_eq!(cpu.X, 0x14);
        assert_eq!(cpu.status, 0b00100000);
    }

    // LDY
    #[test]
    fn test_ldy_immediate() {
        let mut cpu = CPU::new();
        cpu.load_at(0x600, &[0xa0, 0x00]);
        cpu.execute();
        assert_eq!(cpu.Y, 0x00);
        assert_eq!(cpu.status, 0b00100010);
    }

    #[test]
    fn test_ldy_zeropage() {
        let mut cpu = CPU::new();
        cpu.load_at(0, &[0xa4, 0x02, 0xde]);
        cpu.PC = 0;
        cpu.execute();
        assert_eq!(cpu.Y, 0xde);
        assert_eq!(cpu.status, 0b10100000);
    }

    #[test]
    fn test_ldy_zeropage_x() {
        let mut cpu = CPU::new();
        cpu.load_at(0, &[0xb4, 0x02, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x14]);
        cpu.PC = 0;
        cpu.X = 0x8;
        cpu.execute();
        assert_eq!(cpu.Y, 0x14);
        assert_eq!(cpu.status, 0b00100000);
    }

    #[test]
    fn test_ldy_absolute() {
        let mut cpu = CPU::new();
        cpu.load_at(0x600, &[0xac, 0x33, 0x20]);
        cpu.PC = 0x600;
        cpu.memory[0x2033] = 0xef;
        cpu.execute();
        assert_eq!(cpu.Y, 0xef);
        assert_eq!(cpu.status, 0b10100000);
    }

    #[test]
    fn test_ldy_absolute_x() {
        let mut cpu = CPU::new();
        cpu.load_at(0x600, &[0xbc, 0x33, 0x20]);
        cpu.PC = 0x600;
        cpu.X = 0x10;
        cpu.memory[0x2033] = 0xef;
        cpu.memory[0x2043] = 0x14;
        cpu.execute();
        assert_eq!(cpu.Y, 0x14);
        assert_eq!(cpu.status, 0b00100000);
    }

    // STX
    #[test]
    fn test_stx_zeropage() {
        let mut cpu = CPU::new();
        cpu.load_at(0x0, &[0x86, 0x33]);
        cpu.PC = 0x0;
        cpu.X = 0xbb;
        cpu.execute();
        assert_eq!(cpu.memory[0x0033], 0xbb);
        assert_eq!(cpu.status, 0b00100000);
    }

    #[test]
    fn test_stx_zeropage_y() {
        let mut cpu = CPU::new();
        cpu.load_at(0x0, &[0x96, 0x33]);
        cpu.PC = 0x0;
        cpu.X = 0x71;
        cpu.Y = 0x22;
        cpu.execute();
        assert_eq!(cpu.memory[0x0055], 0x71);
        assert_eq!(cpu.status, 0b00100000);
    }

    #[test]
    fn test_stx_absolute() {
        let mut cpu = CPU::new();
        cpu.load_at(0x600, &[0x8e, 0x64, 0x65]);
        cpu.PC = 0x600;
        cpu.X = 0x71;
        cpu.execute();
        assert_eq!(cpu.memory[0x6564], 0x71);
        assert_eq!(cpu.status, 0b00100000);
    }

    // STY
    #[test]
    fn test_sty_zeropage() {
        let mut cpu = CPU::new();
        cpu.load_at(0x0, &[0x84, 0x33]);
        cpu.PC = 0x0;
        cpu.Y = 0xbb;
        cpu.execute();
        assert_eq!(cpu.memory[0x0033], 0xbb);
        assert_eq!(cpu.status, 0b00100000);
    }

    #[test]
    fn test_sty_zeropage_x() {
        let mut cpu = CPU::new();
        cpu.load_at(0x0, &[0x94, 0x33]);
        cpu.PC = 0x0;
        cpu.Y = 0x71;
        cpu.X = 0x22;
        cpu.execute();
        assert_eq!(cpu.memory[0x0055], 0x71);
        assert_eq!(cpu.status, 0b00100000);
    }

    #[test]
    fn test_sty_absolute() {
        let mut cpu = CPU::new();
        cpu.load_at(0x600, &[0x8c, 0x64, 0x65]);
        cpu.PC = 0x600;
        cpu.Y = 0x71;
        cpu.execute();
        assert_eq!(cpu.memory[0x6564], 0x71);
        assert_eq!(cpu.status, 0b00100000);
    }

    // TAX
    #[test]
    fn test_tax() {
        let mut cpu = CPU::new();
        cpu.load_at(0x600, &[0xaa, 0xaa]);
        cpu.PC = 0x600;
        cpu.A = 0xde;
        cpu.X = 0xad;
        cpu.execute();
        assert_eq!(cpu.X, 0xde);
        assert_eq!(cpu.status, 0b10100000);

        cpu.A = 0x00;
        cpu.X = 0xad;
        cpu.execute();
        assert_eq!(cpu.X, 0);
        assert_eq!(cpu.status, 0b00100010);
    }

    // TAY
    #[test]
    fn test_tay() {
        let mut cpu = CPU::new();
        cpu.load_at(0x600, &[0xa8, 0xa8]);
        cpu.PC = 0x600;
        cpu.A = 0xde;
        cpu.Y = 0xad;
        cpu.execute();
        assert_eq!(cpu.Y, 0xde);
        assert_eq!(cpu.status, 0b10100000);

        cpu.A = 0x00;
        cpu.Y = 0xad;
        cpu.execute();
        assert_eq!(cpu.Y, 0);
        assert_eq!(cpu.status, 0b00100010);
    }

    // TXA
    #[test]
    fn test_txa() {
        let mut cpu = CPU::new();
        cpu.load_at(0x600, &[0x8a, 0x8a]);
        cpu.PC = 0x600;
        cpu.A = 0xde;
        cpu.X = 0xad;
        cpu.execute();
        assert_eq!(cpu.A, 0xad);
        assert_eq!(cpu.status, 0b10100000);

        cpu.A = 0x00;
        cpu.X = 0xad;
        cpu.execute();
        assert_eq!(cpu.A, 0xad);
        assert_eq!(cpu.status, 0b10100000);
    }

    // TYA
    #[test]
    fn test_tya() {
        let mut cpu = CPU::new();
        cpu.load_at(0x600, &[0x98, 0x98]);
        cpu.PC = 0x600;
        cpu.A = 0xde;
        cpu.Y = 0xad;
        cpu.execute();
        assert_eq!(cpu.A, 0xad);
        assert_eq!(cpu.status, 0b10100000);

        cpu.A = 0x00;
        cpu.Y = 0xad;
        cpu.execute();
        assert_eq!(cpu.A, 0xad);
        assert_eq!(cpu.status, 0b10100000);
    }

    // DEX
    #[test]
    fn test_dex() {
        let mut cpu = CPU::new();
        cpu.load_at(0, &[0xca, 0xca]);
        cpu.X = 0x01;
        cpu.PC = 0x0;
        cpu.execute();
        assert_eq!(cpu.X, 0x00);
        assert_eq!(cpu.status, 0b00100010);
        cpu.execute();
        assert_eq!(cpu.X, 0xff);
        assert_eq!(cpu.status, 0b10100000);
    }

    // DEY
    #[test]
    fn test_dey() {
        let mut cpu = CPU::new();
        cpu.load_at(0, &[0x88, 0x88]);
        cpu.Y = 0x01;
        cpu.PC = 0x0;
        cpu.execute();
        assert_eq!(cpu.Y, 0x00);
        assert_eq!(cpu.status, 0b00100010);
        cpu.execute();
        assert_eq!(cpu.Y, 0xff);
        assert_eq!(cpu.status, 0b10100000);
    }

    // INX
    #[test]
    fn test_inx() {
        let mut cpu = CPU::new();
        cpu.load_at(0, &[0xe8, 0xe8]);
        cpu.X = 0xfe;
        cpu.PC = 0x0;
        cpu.execute();
        assert_eq!(cpu.X, 0xff);
        assert_eq!(cpu.status, 0b10100000);
        cpu.execute();
        assert_eq!(cpu.X, 0x00);
        assert_eq!(cpu.status, 0b00100010);
    }

    // INY
    #[test]
    fn test_iny() {
        let mut cpu = CPU::new();
        cpu.load_at(0, &[0xc8, 0xc8]);
        cpu.Y = 0xfe;
        cpu.PC = 0x0;
        cpu.execute();
        assert_eq!(cpu.Y, 0xff);
        assert_eq!(cpu.status, 0b10100000);
        cpu.execute();
        assert_eq!(cpu.Y, 0x00);
        assert_eq!(cpu.status, 0b00100010);
    }

    // JMP
    #[test]
    fn test_jmp_indirect() {
        let mut cpu = CPU::new();
        cpu.load_at(0x600, &[0x6c, 0x03, 0x06, 0x12, 0x20]);
        cpu.PC = 0x600;
        cpu.execute();
        assert_eq!(cpu.PC, 0x2012);
        assert_eq!(cpu.status, 0b00100000);
    }

    #[test]
    fn test_jmp_absolute() {
        let mut cpu = CPU::new();
        cpu.load_at(0x600, &[0x4c, 0x12, 0x20]);
        cpu.PC = 0x600;
        cpu.execute();
        assert_eq!(cpu.PC, 0x2012);
        assert_eq!(cpu.status, 0b00100000);
    }
}