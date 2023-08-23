use emu6502::{cpu::CPU, instructions::group2::LDA, memory::Memory};

fn main() {
    let mut cpu = CPU::new();
    let mut memory = Memory::new();
    memory[0xfffc] = LDA;
    memory[0xfffd] = 0x42;
    cpu.exec(&mut memory, 2);
    println!("{:?}", cpu);
    // println!("{:?}", memory);
}
