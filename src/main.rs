use emu6502::cpu::CPU;

fn main() {
    let mut cpu = CPU::new();
    cpu.exec(2);
}
