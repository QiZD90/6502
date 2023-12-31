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

    // CLC, CLD, CLI, CLV, SEC, SED, SEI
    #[test]
    fn test_flags_instructions() {
        let mut cpu = CPU::new();
        cpu.load_at(0x600, &[0x38, 0xf8, 0x78,    0x18, 0xd8, 0x58, 0xb8]);
        cpu.PC = 0x600;

        cpu.execute(); cpu.execute(); cpu.execute();
        assert_eq!(cpu.status, 0b00101101);

        cpu.execute(); cpu.execute(); cpu.execute(); cpu.execute();
        assert_eq!(cpu.status, 0b00100000);
    }

    #[test]
    fn test_php_plp() {
        let mut cpu = CPU::new();
        // Set all flags, push status, clear all flags, pull status
        cpu.load_at(0x0, &[0x38, 0xf8, 0x78, 0x08, 0x18, 0xd8, 0x58, 0xb8, 0x28]);
        cpu.PC = 0;

        cpu.execute(); cpu.execute(); cpu.execute(); cpu.execute();
        assert_eq!(cpu.status, 0b00101101);
        assert_eq!(cpu.SP, 0xfe);

        cpu.execute(); cpu.execute(); cpu.execute(); cpu.execute();
        assert_eq!(cpu.status, 0b00100000);

        cpu.execute();
        assert_eq!(cpu.status, 0b00101101);
        assert_eq!(cpu.SP, 0xff);
    }


    #[test]
    fn test_pha_pla() {
        let mut cpu = CPU::new();
        cpu.load_at(0x0, &[0x48, 0x48, 0x68, 0x68]); // push, push, pull, pull
        cpu.PC = 0;
        cpu.A = 0xfd;
        cpu.execute();
        assert_eq!(cpu.SP, 0xfe);

        cpu.A = 0x00;
        cpu.execute();
        assert_eq!(cpu.SP, 0xfd);

        cpu.execute();
        assert_eq!(cpu.SP, 0xfe);
        assert_eq!(cpu.A, 0x00);
        assert_eq!(cpu.status, 0b00100010);

        cpu.execute();
        assert_eq!(cpu.SP, 0xff);
        assert_eq!(cpu.A, 0xfd);
        assert_eq!(cpu.status, 0b10100000);
    }

    // BCC/BCS, BNE/BEQ, BPL/BMI, BVC/BVS
    #[test]
    fn test_branching() {
        let mut cpu = CPU::new();
        // lda #$11
        // sec
        // bcs label_1
        //
        // lda #$22
        //
        // negative_offset_label:
        // bpl label_2
        // lda #$22
        //
        // label_1:
        // clc
        // bcc negative_offset_label
        // lda #$22
        //
        // label_2:
        // tax
        cpu.load_at(0x600, &[0xa9, 0x11, 0x38, 0xb0, 0x06, 0xa9, 0x22, 0x10, 0x07, 0xa9, 0x22, 0x18, 0x90, 0xf9, 0xa9, 0x22, 0xaa]);
        cpu.PC = 0x600;
        cpu.execute(); cpu.execute(); cpu.execute(); cpu.execute(); cpu.execute(); cpu.execute(); cpu.execute();
        assert_eq!(cpu.A, 0x11);
        assert_eq!(cpu.X, 0x11);
    }

    #[test]
    fn test_push_and_pull_word() {
        let mut cpu = CPU::new();
        cpu.push_word_to_stack(0xdead);
        assert_eq!(cpu.get_byte(0x1ff), 0xde);
        assert_eq!(cpu.get_byte(0x1fe), 0xad);
        let word = cpu.pull_word_from_stack();
        assert_eq!(word, 0xdead);
    }

    #[test]
    fn test_jsr_rts() { // TODO: behavior is not consistent with easy6502. Maybe the site's version is wrong
        let mut cpu = CPU::new();
        // jsr label
        // lda #$22
        // label:
        // lda #$11
        // rts
        cpu.load_at(0x600, &[0x20, 0x05, 0x06, 0xa9, 0x22, 0xa9, 0x11, 0x60]);
        cpu.execute(); cpu.execute();
        assert_eq!(cpu.A, 0x11);
        cpu.execute(); cpu.execute();
        assert_eq!(cpu.A, 0x22);
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

    #[test]
    fn test_asl() {
        let mut cpu = CPU::new();
        // lda #$31
        // asl A
        // sta label
        // asl label
        // label:
        cpu.load_at(0x600, &[0xa9, 0x31, 0x0a, 0x8d, 0x09, 0x06, 0x0e, 0x09, 0x06]);
        cpu.PC = 0x600;
        cpu.execute(); cpu.execute(); cpu.execute(); cpu.execute();
        assert_eq!(cpu.A, 0x62);
        assert_eq!(cpu.get_byte(0x609), 0xc4);
    }

    #[test]
    fn test_cmp() {
        let mut cpu = CPU::new();
        // jmp label
        // value: nop
        // label:
        // lda #$20
        // sta value
        // lda #$1a
        // cmp value
        // lda #$45
        // cmp value
        // lda #$ff
        // cmp value
        cpu.load_at(0x600, &[
            0x4c, 0x04, 0x06, 0x20, 0xa9, 0x20, 0x8d, 0x03, 0x06, 0xa9, 0x1a, 0xcd, 0x03,
            0x06, 0xa9, 0x45, 0xcd, 0x03, 0x06, 0xa9, 0xff, 0xcd, 0x03, 0x06]);
        cpu.PC = 0x600;
        cpu.execute(); cpu.execute(); cpu.execute(); cpu.execute(); cpu.execute();
        assert_eq!(cpu.status, 0b10100000);
        cpu.execute(); cpu.execute();
        assert_eq!(cpu.status, 0b00100001);
        cpu.execute(); cpu.execute();
        assert_eq!(cpu.status, 0b10100001);
    }
}