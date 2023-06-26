mod cpu;
mod instructions;

use crate::cpu::*;

fn main() {
    let mut cpu = CPU::new();
    cpu.load(&[0xa9, 0x01, 0x8d, 0x00, 0x02, 0xa9, 0x05, 0x8d, 0x01, 0x02, 0xa9, 0x08, 0x8d, 0x02, 0x02 ]);
    cpu.execute();
    cpu.print();
    cpu.execute();
    cpu.print();
    cpu.execute();
    cpu.print();
    cpu.execute();
    cpu.print();
    cpu.execute();
    cpu.print();
    cpu.execute();
    cpu.print();
    cpu.print_memory();
}