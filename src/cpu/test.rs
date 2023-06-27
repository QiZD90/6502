#[cfg(test)]
mod test {
    use super::super::*;

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

    // DEC
    #[test]
    fn test_dec_zeropage() {
        let mut cpu = CPU::new();
        cpu.load_at(0, &[0xc6, 0x10, 0xc6, 0x10]);
        cpu.PC = 0;
        cpu.memory[0x10] = 0x01;
        cpu.execute();
        assert_eq!(cpu.memory[0x10], 0x00);
        assert_eq!(cpu.status, 0b00100010);
        cpu.execute();
        assert_eq!(cpu.memory[0x10], 0xff);
        assert_eq!(cpu.status, 0b10100000);
    }

    #[test]
    fn test_dec_zeropage_x() {
        let mut cpu = CPU::new();
        cpu.load_at(0, &[0xd6, 0xb, 0xd6, 0xb]);
        cpu.PC = 0;
        cpu.X = 0x5;
        cpu.memory[0xb] = 0xdd;
        cpu.memory[0x10] = 0x01;
        cpu.execute();
        assert_eq!(cpu.memory[0x10], 0x00);
        assert_eq!(cpu.status, 0b00100010);
        cpu.execute();
        assert_eq!(cpu.memory[0x10], 0xff);
        assert_eq!(cpu.status, 0b10100000);
    }

    #[test]
    fn test_dec_absolute() {
        let mut cpu = CPU::new();
        cpu.load_at(0, &[0xce, 0x33, 0x34, 0xce, 0x33, 0x34]);
        cpu.PC = 0;
        cpu.memory[0x3433] = 0x01;
        cpu.execute();
        assert_eq!(cpu.memory[0x3433], 0x00);
        assert_eq!(cpu.status, 0b00100010);
        cpu.execute();
        assert_eq!(cpu.memory[0x3433], 0xff);
        assert_eq!(cpu.status, 0b10100000);
    }

    #[test]
    fn test_dec_absolute_x() {
        let mut cpu = CPU::new();
        cpu.load_at(0, &[0xde, 0x32, 0x34, 0xde, 0x32, 0x34]);
        cpu.PC = 0;
        cpu.X = 0x1;
        cpu.memory[0x3433] = 0x01;
        cpu.execute();
        assert_eq!(cpu.memory[0x3433], 0x00);
        assert_eq!(cpu.status, 0b00100010);
        cpu.execute();
        assert_eq!(cpu.memory[0x3433], 0xff);
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

    // INC
    #[test]
    fn test_inc_zeropage() {
        let mut cpu = CPU::new();
        cpu.load_at(0, &[0xe6, 0x10, 0xe6, 0x10]);
        cpu.PC = 0;
        cpu.memory[0x10] = 0xfe;
        cpu.execute();
        assert_eq!(cpu.memory[0x10], 0xff);
        assert_eq!(cpu.status, 0b10100000);
        cpu.execute();
        assert_eq!(cpu.memory[0x10], 0x00);
        assert_eq!(cpu.status, 0b00100010);
    }

    #[test]
    fn test_inc_zeropagex() {
        let mut cpu = CPU::new();
        cpu.load_at(0, &[0xf6, 0x05, 0xf6, 0x05]);
        cpu.PC = 0;
        cpu.X = 0x0b;
        cpu.memory[0x10] = 0xfe;
        cpu.execute();
        assert_eq!(cpu.memory[0x10], 0xff);
        assert_eq!(cpu.status, 0b10100000);
        cpu.execute();
        assert_eq!(cpu.memory[0x10], 0x00);
        assert_eq!(cpu.status, 0b00100010);
    }

    #[test]
    fn test_inc_absolute() {
        let mut cpu = CPU::new();
        cpu.load_at(0, &[0xee, 0x12, 0x20, 0xee, 0x12, 0x20]);
        cpu.PC = 0;
        cpu.memory[0x2012] = 0xfe;
        cpu.execute();
        assert_eq!(cpu.memory[0x2012], 0xff);
        assert_eq!(cpu.status, 0b10100000);
        cpu.execute();
        assert_eq!(cpu.memory[0x2012], 0x00);
        assert_eq!(cpu.status, 0b00100010);
    }

    #[test]
    fn test_inc_absolute_x() {
        let mut cpu = CPU::new();
        cpu.load_at(0, &[0xfe, 0x12, 0x20, 0xfe, 0x12, 0x20]);
        cpu.PC = 0;
        cpu.X = 0x10;
        cpu.memory[0x2012] = 0xde;
        cpu.memory[0x2022] = 0xfe;
        cpu.execute();
        assert_eq!(cpu.memory[0x2022], 0xff);
        assert_eq!(cpu.status, 0b10100000);
        cpu.execute();
        assert_eq!(cpu.memory[0x2022], 0x00);
        assert_eq!(cpu.status, 0b00100010);
    }

    // TSX
    #[test]
    fn test_tsx() {
        let mut cpu = CPU::new();
        cpu.load_at(0, &[0xba, 0xba, 0xba]);
        cpu.SP = 0x22;
        cpu.PC = 0;
        cpu.execute();
        assert_eq!(cpu.SP, 0x22);
        assert_eq!(cpu.X, 0x22);
        assert_eq!(cpu.status, 0b00100000);

        cpu.SP = 0x00;
        cpu.execute();
        assert_eq!(cpu.SP, 0x00);
        assert_eq!(cpu.X, 0x00);
        assert_eq!(cpu.status, 0b00100010);

        cpu.SP = 0xfa;
        cpu.execute();
        assert_eq!(cpu.SP, 0xfa);
        assert_eq!(cpu.X, 0xfa);
        assert_eq!(cpu.status, 0b10100000);
    }

    // TXS
    #[test]
    fn test_txs() {
        let mut cpu = CPU::new();
        cpu.load_at(0, &[0x9a, 0x9a, 0x9a]);
        cpu.X = 0x22;
        cpu.PC = 0;
        cpu.execute();
        assert_eq!(cpu.SP, 0x22);
        assert_eq!(cpu.X, 0x22);
        assert_eq!(cpu.status, 0b00100000);

        cpu.X = 0x00;
        cpu.execute();
        assert_eq!(cpu.SP, 0x00);
        assert_eq!(cpu.X, 0x00);
        assert_eq!(cpu.status, 0b00100000);

        cpu.X = 0xfa;
        cpu.execute();
        assert_eq!(cpu.SP, 0xfa);
        assert_eq!(cpu.X, 0xfa);
        assert_eq!(cpu.status, 0b00100000);
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