mod cpu;
mod instructions;

use crate::cpu::*;

fn main() {
    let mut cpu = CPU::new();
    cpu.load(&[0x6c, 0x03, 0x06, 0x12, 0x20]);
    cpu.execute();
    cpu.print();
}