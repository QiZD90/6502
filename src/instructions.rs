use crate::instructions::AddressingMode::*;
use crate::instructions::Instruction::*;
use crate::instructions::Cycles::*;

#[derive(Copy, Clone)]
pub enum AddressingMode {
    Implied = 0,
    Accumulator = 1,
    Immediate = 2,
    ZeroPage = 3,
    ZeroPageX = 4,
    ZeroPageY = 5,
    Relative = 6,
    Absolute = 7,
    AbsoluteX = 8,
    AbsoluteY = 9,
    Indirect = 10,
    IndirectX = 11,
    IndirectY = 12
}

impl AddressingMode {
    pub fn operand_bytes(self) -> u16 {
        return match self {
            Implied => 0,
            Accumulator => 0,
            Immediate => 1,
            ZeroPage => 1,
            ZeroPageX => 1,
            ZeroPageY => 1,
            Relative => 1,
            Absolute => 2,
            AbsoluteX => 2,
            AbsoluteY => 2,
            Indirect => 2,
            IndirectX => 1,
            IndirectY => 1
        }
    }
}

#[derive(Copy, Clone)]
pub enum Cycles {
    Exact(u16), // Exact amount of cycles
    PageBoundary(u16), // Exact amount of cycles + 1 if page boundary has been crossed
    Branching // 2 cycles if not taken; 3 cycles if taken + 1 if boundary has been crossed
}

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    LDA, // Load A;            Modes: Immediate, ZP, ZPX, Absolute, AX, AY, IX, IY; Flags: N-----Z-
    LDX, // Load X;            Modes: Immediate, ZP, ZPY, Absolute, AY;             Flags: N-----Z-
    LDY, // Load Y;            Modes: Immediate, ZP, ZPX, Absolute, AX;             Flags: N-----Z-
    STA, // Store A;           Modes: ZP, ZPX, Absolute, AX, AY, IX, IY;            Flags: --------
    STX, // Store X;           Modes: ZP, ZPY, Absolute;                            Flags: --------
    STY, // Store Y;           Modes: ZP, ZPX, Absolute;                            Flags: --------
    TAX, // Transfer A to X;   Modes: Implied;                                      Flags: N-----Z-
    TAY, // Transfer A to Y;   Modes: Implied;                                      Flags: N-----Z-
    TXA, // Transfer X to A;   Modes: Implied;                                      Flags: N-----Z-
    TYA, // Transfer Y to A;   Modes: Implied;                                      Flags: N-----Z-
    DEX, // Decrement X;       Modes: Implied;                                      Flags: N-----Z-
    DEY, // Decrement Y;       Modes: Implied;                                      Flags: N-----Z-
    DEC, // Increment memory;  Modes: ZP, ZPX, Absoulute, AX;                       Flags: N-----Z-
    INX, // Increment X;       Modes: Implied;                                      Flags: N-----Z-
    INY, // Increment Y;       Modes: Implied;                                      Flags: N-----Z-
    INC, // Increment memory;  Modes: ZP, ZPX, Absoulute, AX;                       Flags: N-----Z-
    TSX, // Transfer SP to X;  Modes: Implied;                                      Flags: N-----Z-
    TXS, // Transfer X to SP;  Modes: Implied;                                      Flags: --------
    JMP, // Jump to;           Modes: Absolute, Indirect;                           Flags: --------
    None
}

pub static OPCODES: [(Instruction, AddressingMode, Cycles); 256] = [
    (None, Implied, Exact(0)), // 0x00
    (None, Implied, Exact(0)), // 0x01
    (None, Implied, Exact(0)), // 0x02
    (None, Implied, Exact(0)), // 0x03
    (None, Implied, Exact(0)), // 0x04
    (None, Implied, Exact(0)), // 0x05
    (None, Implied, Exact(0)), // 0x06
    (None, Implied, Exact(0)), // 0x07
    (None, Implied, Exact(0)), // 0x08
    (None, Implied, Exact(0)), // 0x09
    (None, Implied, Exact(0)), // 0x0a
    (None, Implied, Exact(0)), // 0x0b
    (None, Implied, Exact(0)), // 0x0c
    (None, Implied, Exact(0)), // 0x0d
    (None, Implied, Exact(0)), // 0x0e
    (None, Implied, Exact(0)), // 0x0f
    (None, Implied, Exact(0)), // 0x10
    (None, Implied, Exact(0)), // 0x11
    (None, Implied, Exact(0)), // 0x12
    (None, Implied, Exact(0)), // 0x13
    (None, Implied, Exact(0)), // 0x14
    (None, Implied, Exact(0)), // 0x15
    (None, Implied, Exact(0)), // 0x16
    (None, Implied, Exact(0)), // 0x17
    (None, Implied, Exact(0)), // 0x18
    (None, Implied, Exact(0)), // 0x19
    (None, Implied, Exact(0)), // 0x1a
    (None, Implied, Exact(0)), // 0x1b
    (None, Implied, Exact(0)), // 0x1c
    (None, Implied, Exact(0)), // 0x1d
    (None, Implied, Exact(0)), // 0x1e
    (None, Implied, Exact(0)), // 0x1f
    (None, Implied, Exact(0)), // 0x20
    (None, Implied, Exact(0)), // 0x21
    (None, Implied, Exact(0)), // 0x22
    (None, Implied, Exact(0)), // 0x23
    (None, Implied, Exact(0)), // 0x24
    (None, Implied, Exact(0)), // 0x25
    (None, Implied, Exact(0)), // 0x26
    (None, Implied, Exact(0)), // 0x27
    (None, Implied, Exact(0)), // 0x28
    (None, Implied, Exact(0)), // 0x29
    (None, Implied, Exact(0)), // 0x2a
    (None, Implied, Exact(0)), // 0x2b
    (None, Implied, Exact(0)), // 0x2c
    (None, Implied, Exact(0)), // 0x2d
    (None, Implied, Exact(0)), // 0x2e
    (None, Implied, Exact(0)), // 0x2f
    (None, Implied, Exact(0)), // 0x30
    (None, Implied, Exact(0)), // 0x31
    (None, Implied, Exact(0)), // 0x32
    (None, Implied, Exact(0)), // 0x33
    (None, Implied, Exact(0)), // 0x34
    (None, Implied, Exact(0)), // 0x35
    (None, Implied, Exact(0)), // 0x36
    (None, Implied, Exact(0)), // 0x37
    (None, Implied, Exact(0)), // 0x38
    (None, Implied, Exact(0)), // 0x39
    (None, Implied, Exact(0)), // 0x3a
    (None, Implied, Exact(0)), // 0x3b
    (None, Implied, Exact(0)), // 0x3c
    (None, Implied, Exact(0)), // 0x3d
    (None, Implied, Exact(0)), // 0x3e
    (None, Implied, Exact(0)), // 0x3f
    (None, Implied, Exact(0)), // 0x40
    (None, Implied, Exact(0)), // 0x41
    (None, Implied, Exact(0)), // 0x42
    (None, Implied, Exact(0)), // 0x43
    (None, Implied, Exact(0)), // 0x44
    (None, Implied, Exact(0)), // 0x45
    (None, Implied, Exact(0)), // 0x46
    (None, Implied, Exact(0)), // 0x47
    (None, Implied, Exact(0)), // 0x48
    (None, Implied, Exact(0)), // 0x49
    (None, Implied, Exact(0)), // 0x4a
    (None, Implied, Exact(0)), // 0x4b
    (JMP, Absolute, Exact(3)), // 0x4c
    (None, Implied, Exact(0)), // 0x4d
    (None, Implied, Exact(0)), // 0x4e
    (None, Implied, Exact(0)), // 0x4f
    (None, Implied, Exact(0)), // 0x50
    (None, Implied, Exact(0)), // 0x51
    (None, Implied, Exact(0)), // 0x52
    (None, Implied, Exact(0)), // 0x53
    (None, Implied, Exact(0)), // 0x54
    (None, Implied, Exact(0)), // 0x55
    (None, Implied, Exact(0)), // 0x56
    (None, Implied, Exact(0)), // 0x57
    (None, Implied, Exact(0)), // 0x58
    (None, Implied, Exact(0)), // 0x59
    (None, Implied, Exact(0)), // 0x5a
    (None, Implied, Exact(0)), // 0x5b
    (None, Implied, Exact(0)), // 0x5c
    (None, Implied, Exact(0)), // 0x5d
    (None, Implied, Exact(0)), // 0x5e
    (None, Implied, Exact(0)), // 0x5f
    (None, Implied, Exact(0)), // 0x60
    (None, Implied, Exact(0)), // 0x61
    (None, Implied, Exact(0)), // 0x62
    (None, Implied, Exact(0)), // 0x63
    (None, Implied, Exact(0)), // 0x64
    (None, Implied, Exact(0)), // 0x65
    (None, Implied, Exact(0)), // 0x66
    (None, Implied, Exact(0)), // 0x67
    (None, Implied, Exact(0)), // 0x68
    (None, Implied, Exact(0)), // 0x69
    (None, Implied, Exact(0)), // 0x6a
    (None, Implied, Exact(0)), // 0x6b
    (JMP, Indirect, Exact(5)), // 0x6c
    (None, Implied, Exact(0)), // 0x6d
    (None, Implied, Exact(0)), // 0x6e
    (None, Implied, Exact(0)), // 0x6f
    (None, Implied, Exact(0)), // 0x70
    (None, Implied, Exact(0)), // 0x71
    (None, Implied, Exact(0)), // 0x72
    (None, Implied, Exact(0)), // 0x73
    (None, Implied, Exact(0)), // 0x74
    (None, Implied, Exact(0)), // 0x75
    (None, Implied, Exact(0)), // 0x76
    (None, Implied, Exact(0)), // 0x77
    (None, Implied, Exact(0)), // 0x78
    (None, Implied, Exact(0)), // 0x79
    (None, Implied, Exact(0)), // 0x7a
    (None, Implied, Exact(0)), // 0x7b
    (None, Implied, Exact(0)), // 0x7c
    (None, Implied, Exact(0)), // 0x7d
    (None, Implied, Exact(0)), // 0x7e
    (None, Implied, Exact(0)), // 0x7f
    (None, Implied, Exact(0)), // 0x80
    (STA, IndirectX, Exact(6)), // 0x81
    (None, Implied, Exact(0)), // 0x82
    (None, Implied, Exact(0)), // 0x83
    (STY, ZeroPage, Exact(3)), // 0x84
    (STA, ZeroPage, Exact(3)), // 0x85
    (STX, ZeroPage, Exact(3)), // 0x86
    (None, Implied, Exact(0)), // 0x87
    (DEY, Implied, Exact(2)), // 0x88
    (None, Implied, Exact(0)), // 0x89
    (TXA, Implied, Exact(2)), // 0x8a
    (None, Implied, Exact(0)), // 0x8b
    (STY, Absolute, Exact(4)), // 0x8c
    (STA, Absolute, Exact(4)), // 0x8d
    (STX, Absolute, Exact(4)), // 0x8e
    (None, Implied, Exact(0)), // 0x8f
    (None, Implied, Exact(0)), // 0x90
    (STA, IndirectY, Exact(6)), // 0x91
    (None, Implied, Exact(0)), // 0x92
    (None, Implied, Exact(0)), // 0x93
    (STY, ZeroPageX, Exact(4)), // 0x94
    (STA, ZeroPageX, Exact(4)), // 0x95
    (STX, ZeroPageY, Exact(4)), // 0x96
    (None, Implied, Exact(0)), // 0x97
    (TYA, Implied, Exact(2)), // 0x98
    (STA, AbsoluteY, Exact(5)), // 0x99
    (TXS, Implied, Exact(2)), // 0x9a
    (None, Implied, Exact(0)), // 0x9b
    (None, Implied, Exact(0)), // 0x9c
    (STA, AbsoluteX, Exact(5)), // 0x9d
    (None, Implied, Exact(0)), // 0x9e
    (None, Implied, Exact(0)), // 0x9f
    (LDY, Immediate, Exact(2)), // 0xa0
    (LDA, IndirectX, Exact(6)), // 0xa1
    (LDX, Immediate, Exact(2)), // 0xa2
    (None, Implied, Exact(0)), // 0xa3
    (LDY, ZeroPage, Exact(3)), // 0xa4
    (LDA, ZeroPage, Exact(3)), // 0xa5
    (LDX, ZeroPage, Exact(3)), // 0xa6
    (None, Implied, Exact(0)), // 0xa7
    (TAY, Implied, Exact(2)), // 0xa8
    (LDA, Immediate, Exact(2)), // 0xa9
    (TAX, Implied, Exact(2)), // 0xaa
    (None, Implied, Exact(0)), // 0xab
    (LDY, Absolute, Exact(4)), // 0xac
    (LDA, Absolute, Exact(4)), // 0xad
    (LDX, Absolute, Exact(4)), // 0xae
    (None, Implied, Exact(0)), // 0xaf
    (None, Implied, Exact(0)), // 0xb0
    (LDA, IndirectY, PageBoundary(5)), // 0xb1
    (None, Implied, Exact(0)), // 0xb2
    (None, Implied, Exact(0)), // 0xb3
    (LDY, ZeroPageX, Exact(4)), // 0xb4
    (LDA, ZeroPageX, Exact(4)), // 0xb5
    (LDX, ZeroPageY, Exact(4)), // 0xb6
    (None, Implied, Exact(0)), // 0xb7
    (None, Implied, Exact(0)), // 0xb8
    (LDA, AbsoluteY, PageBoundary(4)), // 0xb9
    (TSX, Implied, Exact(2)), // 0xba
    (None, Implied, Exact(0)), // 0xbb
    (LDY, AbsoluteX, PageBoundary(4)), // 0xbc
    (LDA, AbsoluteX, PageBoundary(4)), // 0xbd
    (LDX, AbsoluteY, PageBoundary(4)), // 0xbe
    (None, Implied, Exact(0)), // 0xbf
    (None, Implied, Exact(0)), // 0xc0
    (None, Implied, Exact(0)), // 0xc1
    (None, Implied, Exact(0)), // 0xc2
    (None, Implied, Exact(0)), // 0xc3
    (None, Implied, Exact(0)), // 0xc4
    (None, Implied, Exact(0)), // 0xc5
    (DEC, ZeroPage, Exact(5)), // 0xc6
    (None, Implied, Exact(0)), // 0xc7
    (INY, Implied, Exact(2)), // 0xc8
    (None, Implied, Exact(0)), // 0xc9
    (DEX, Implied, Exact(2)), // 0xca
    (None, Implied, Exact(0)), // 0xcb
    (None, Implied, Exact(0)), // 0xcc
    (None, Implied, Exact(0)), // 0xcd
    (DEC, Absolute, Exact(6)), // 0xce
    (None, Implied, Exact(0)), // 0xcf
    (None, Implied, Exact(0)), // 0xd0
    (None, Implied, Exact(0)), // 0xd1
    (None, Implied, Exact(0)), // 0xd2
    (None, Implied, Exact(0)), // 0xd3
    (None, Implied, Exact(0)), // 0xd4
    (None, Implied, Exact(0)), // 0xd5
    (DEC, ZeroPageX, Exact(6)), // 0xd6
    (None, Implied, Exact(0)), // 0xd7
    (None, Implied, Exact(0)), // 0xd8
    (None, Implied, Exact(0)), // 0xd9
    (None, Implied, Exact(0)), // 0xda
    (None, Implied, Exact(0)), // 0xdb
    (None, Implied, Exact(0)), // 0xdc
    (None, Implied, Exact(0)), // 0xdd
    (DEC, AbsoluteX, Exact(7)), // 0xde
    (None, Implied, Exact(0)), // 0xdf
    (None, Implied, Exact(0)), // 0xe0
    (None, Implied, Exact(0)), // 0xe1
    (None, Implied, Exact(0)), // 0xe2
    (None, Implied, Exact(0)), // 0xe3
    (None, Implied, Exact(0)), // 0xe4
    (None, Implied, Exact(0)), // 0xe5
    (INC, ZeroPage, Exact(5)), // 0xe6
    (None, Implied, Exact(0)), // 0xe7
    (INX, Implied, Exact(2)), // 0xe8
    (None, Implied, Exact(0)), // 0xe9
    (None, Implied, Exact(0)), // 0xea
    (None, Implied, Exact(0)), // 0xeb
    (None, Implied, Exact(0)), // 0xec
    (None, Implied, Exact(0)), // 0xed
    (INC, Absolute, Exact(6)), // 0xee
    (None, Implied, Exact(0)), // 0xef
    (None, Implied, Exact(0)), // 0xf0
    (None, Implied, Exact(0)), // 0xf1
    (None, Implied, Exact(0)), // 0xf2
    (None, Implied, Exact(0)), // 0xf3
    (None, Implied, Exact(0)), // 0xf4
    (None, Implied, Exact(0)), // 0xf5
    (INC, ZeroPageX, Exact(6)), // 0xf6
    (None, Implied, Exact(0)), // 0xf7
    (None, Implied, Exact(0)), // 0xf8
    (None, Implied, Exact(0)), // 0xf9
    (None, Implied, Exact(0)), // 0xfa
    (None, Implied, Exact(0)), // 0xfb
    (None, Implied, Exact(0)), // 0xfc
    (None, Implied, Exact(0)), // 0xfd
    (INC, AbsoluteX, Exact(7)), // 0xfe
    (None, Implied, Exact(0)), // 0xff
];
