use crate::instructions::AddressingMode::*;
use crate::instructions::Instruction::*;

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

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    LDA, // Load A;            Modes: Immediate, ZP, ZPX, Absolute, AX, AY, IX, IY; Flags: N-----Z-
    LDX, // Load X;            Modes: Immediate, ZP, ZPY, Absolute, AY;             Flags: N-----Z-
    LDY, // Load Y;            Modes: Immediate, ZP, ZPX, Absolute, AX;             Flags: N-----Z-
    STA, // Store A;           Modes: ZP, ZPX, Absolute, AX, AY, IX, IY;            Flags: --------
    STX, // Store X;           Modes: ZP, ZPY, Absolute;                            Flags: --------
    STY, // Store Y;           Modes: ZP, ZPX, Absolute;                            Flags: --------
    JMP, // Jump to;           Modes: Absolute, Indirect;                           Flags: --------
    None
}

pub static OPCODES: [(Instruction, AddressingMode); 256] = [
    (None, Implied), // 0x00
    (None, Implied), // 0x01
    (None, Implied), // 0x02
    (None, Implied), // 0x03
    (None, Implied), // 0x04
    (None, Implied), // 0x05
    (None, Implied), // 0x06
    (None, Implied), // 0x07
    (None, Implied), // 0x08
    (None, Implied), // 0x09
    (None, Implied), // 0x0a
    (None, Implied), // 0x0b
    (None, Implied), // 0x0c
    (None, Implied), // 0x0d
    (None, Implied), // 0x0e
    (None, Implied), // 0x0f
    (None, Implied), // 0x10
    (None, Implied), // 0x11
    (None, Implied), // 0x12
    (None, Implied), // 0x13
    (None, Implied), // 0x14
    (None, Implied), // 0x15
    (None, Implied), // 0x16
    (None, Implied), // 0x17
    (None, Implied), // 0x18
    (None, Implied), // 0x19
    (None, Implied), // 0x1a
    (None, Implied), // 0x1b
    (None, Implied), // 0x1c
    (None, Implied), // 0x1d
    (None, Implied), // 0x1e
    (None, Implied), // 0x1f
    (None, Implied), // 0x20
    (None, Implied), // 0x21
    (None, Implied), // 0x22
    (None, Implied), // 0x23
    (None, Implied), // 0x24
    (None, Implied), // 0x25
    (None, Implied), // 0x26
    (None, Implied), // 0x27
    (None, Implied), // 0x28
    (None, Implied), // 0x29
    (None, Implied), // 0x2a
    (None, Implied), // 0x2b
    (None, Implied), // 0x2c
    (None, Implied), // 0x2d
    (None, Implied), // 0x2e
    (None, Implied), // 0x2f
    (None, Implied), // 0x30
    (None, Implied), // 0x31
    (None, Implied), // 0x32
    (None, Implied), // 0x33
    (None, Implied), // 0x34
    (None, Implied), // 0x35
    (None, Implied), // 0x36
    (None, Implied), // 0x37
    (None, Implied), // 0x38
    (None, Implied), // 0x39
    (None, Implied), // 0x3a
    (None, Implied), // 0x3b
    (None, Implied), // 0x3c
    (None, Implied), // 0x3d
    (None, Implied), // 0x3e
    (None, Implied), // 0x3f
    (None, Implied), // 0x40
    (None, Implied), // 0x41
    (None, Implied), // 0x42
    (None, Implied), // 0x43
    (None, Implied), // 0x44
    (None, Implied), // 0x45
    (None, Implied), // 0x46
    (None, Implied), // 0x47
    (None, Implied), // 0x48
    (None, Implied), // 0x49
    (None, Implied), // 0x4a
    (None, Implied), // 0x4b
    (JMP, Absolute), // 0x4c
    (None, Implied), // 0x4d
    (None, Implied), // 0x4e
    (None, Implied), // 0x4f
    (None, Implied), // 0x50
    (None, Implied), // 0x51
    (None, Implied), // 0x52
    (None, Implied), // 0x53
    (None, Implied), // 0x54
    (None, Implied), // 0x55
    (None, Implied), // 0x56
    (None, Implied), // 0x57
    (None, Implied), // 0x58
    (None, Implied), // 0x59
    (None, Implied), // 0x5a
    (None, Implied), // 0x5b
    (None, Implied), // 0x5c
    (None, Implied), // 0x5d
    (None, Implied), // 0x5e
    (None, Implied), // 0x5f
    (None, Implied), // 0x60
    (None, Implied), // 0x61
    (None, Implied), // 0x62
    (None, Implied), // 0x63
    (None, Implied), // 0x64
    (None, Implied), // 0x65
    (None, Implied), // 0x66
    (None, Implied), // 0x67
    (None, Implied), // 0x68
    (None, Implied), // 0x69
    (None, Implied), // 0x6a
    (None, Implied), // 0x6b
    (JMP, Indirect), // 0x6c
    (None, Implied), // 0x6d
    (None, Implied), // 0x6e
    (None, Implied), // 0x6f
    (None, Implied), // 0x70
    (None, Implied), // 0x71
    (None, Implied), // 0x72
    (None, Implied), // 0x73
    (None, Implied), // 0x74
    (None, Implied), // 0x75
    (None, Implied), // 0x76
    (None, Implied), // 0x77
    (None, Implied), // 0x78
    (None, Implied), // 0x79
    (None, Implied), // 0x7a
    (None, Implied), // 0x7b
    (None, Implied), // 0x7c
    (None, Implied), // 0x7d
    (None, Implied), // 0x7e
    (None, Implied), // 0x7f
    (None, Implied), // 0x80
    (STA, IndirectX), // 0x81
    (None, Implied), // 0x82
    (None, Implied), // 0x83
    (STY, ZeroPage), // 0x84
    (STA, ZeroPage), // 0x85
    (STX, ZeroPage), // 0x86
    (None, Implied), // 0x87
    (None, Implied), // 0x88
    (None, Implied), // 0x89
    (None, Implied), // 0x8a
    (None, Implied), // 0x8b
    (STY, Absolute), // 0x8c
    (STA, Absolute), // 0x8d
    (STX, Absolute), // 0x8e
    (None, Implied), // 0x8f
    (None, Implied), // 0x90
    (STA, IndirectY), // 0x91
    (None, Implied), // 0x92
    (None, Implied), // 0x93
    (STY, ZeroPageX), // 0x94
    (STA, ZeroPageX), // 0x95
    (STX, ZeroPageY), // 0x96
    (None, Implied), // 0x97
    (None, Implied), // 0x98
    (STA, AbsoluteY), // 0x99
    (None, Implied), // 0x9a
    (None, Implied), // 0x9b
    (None, Implied), // 0x9c
    (STA, AbsoluteX), // 0x9d
    (None, Implied), // 0x9e
    (None, Implied), // 0x9f
    (LDY, Immediate), // 0xa0
    (LDA, IndirectX), // 0xa1
    (LDX, Immediate), // 0xa2
    (None, Implied), // 0xa3
    (LDY, ZeroPage), // 0xa4
    (LDA, ZeroPage), // 0xa5
    (LDX, ZeroPage), // 0xa6
    (None, Implied), // 0xa7
    (None, Implied), // 0xa8
    (LDA, Immediate), // 0xa9
    (None, Implied), // 0xaa
    (None, Implied), // 0xab
    (LDY, Absolute), // 0xac
    (LDA, Absolute), // 0xad
    (LDX, Absolute), // 0xae
    (None, Implied), // 0xaf
    (None, Implied), // 0xb0
    (LDA, IndirectY), // 0xb1
    (None, Implied), // 0xb2
    (None, Implied), // 0xb3
    (LDY, ZeroPageX), // 0xb4
    (LDA, ZeroPageX), // 0xb5
    (LDX, ZeroPageY), // 0xb6
    (None, Implied), // 0xb7
    (None, Implied), // 0xb8
    (LDA, AbsoluteY), // 0xb9
    (None, Implied), // 0xba
    (None, Implied), // 0xbb
    (LDY, AbsoluteX), // 0xbc
    (LDA, AbsoluteX), // 0xbd
    (LDX, AbsoluteY), // 0xbe
    (None, Implied), // 0xbf
    (None, Implied), // 0xc0
    (None, Implied), // 0xc1
    (None, Implied), // 0xc2
    (None, Implied), // 0xc3
    (None, Implied), // 0xc4
    (None, Implied), // 0xc5
    (None, Implied), // 0xc6
    (None, Implied), // 0xc7
    (None, Implied), // 0xc8
    (None, Implied), // 0xc9
    (None, Implied), // 0xca
    (None, Implied), // 0xcb
    (None, Implied), // 0xcc
    (None, Implied), // 0xcd
    (None, Implied), // 0xce
    (None, Implied), // 0xcf
    (None, Implied), // 0xd0
    (None, Implied), // 0xd1
    (None, Implied), // 0xd2
    (None, Implied), // 0xd3
    (None, Implied), // 0xd4
    (None, Implied), // 0xd5
    (None, Implied), // 0xd6
    (None, Implied), // 0xd7
    (None, Implied), // 0xd8
    (None, Implied), // 0xd9
    (None, Implied), // 0xda
    (None, Implied), // 0xdb
    (None, Implied), // 0xdc
    (None, Implied), // 0xdd
    (None, Implied), // 0xde
    (None, Implied), // 0xdf
    (None, Implied), // 0xe0
    (None, Implied), // 0xe1
    (None, Implied), // 0xe2
    (None, Implied), // 0xe3
    (None, Implied), // 0xe4
    (None, Implied), // 0xe5
    (None, Implied), // 0xe6
    (None, Implied), // 0xe7
    (None, Implied), // 0xe8
    (None, Implied), // 0xe9
    (None, Implied), // 0xea
    (None, Implied), // 0xeb
    (None, Implied), // 0xec
    (None, Implied), // 0xed
    (None, Implied), // 0xee
    (None, Implied), // 0xef
    (None, Implied), // 0xf0
    (None, Implied), // 0xf1
    (None, Implied), // 0xf2
    (None, Implied), // 0xf3
    (None, Implied), // 0xf4
    (None, Implied), // 0xf5
    (None, Implied), // 0xf6
    (None, Implied), // 0xf7
    (None, Implied), // 0xf8
    (None, Implied), // 0xf9
    (None, Implied), // 0xfa
    (None, Implied), // 0xfb
    (None, Implied), // 0xfc
    (None, Implied), // 0xfd
    (None, Implied), // 0xfe
    (None, Implied), // 0xff
];
