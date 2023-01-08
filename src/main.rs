pub mod cpu;
pub mod ram;

fn main() {
    use crate::ram::Ram;
    use crate::cpu::Cpu;

    let mut ram = Ram::create();
    ram.write(0x0, 0xA1);
    ram.write(0x1, 0x42);
    ram.write(0x42, 0x69);
    ram.write(0x43, 0x35);
    let mut cpu = Cpu::create(ram);
    cpu.read_instruction();
    println!("Hello, world!");
}
