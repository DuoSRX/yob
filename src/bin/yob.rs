extern crate yob;

use yob::cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();
    cpu.registers.a = 2;
    println!("{:?}", cpu);
    cpu.execute_instruction(0x87);
    println!("{:?}", cpu);
    // cpu.execute_instruction(0x41);
    // println!("{:?}", cpu);
    // cpu.execute_instruction(0x04);
    // println!("{:?}", cpu);
}
