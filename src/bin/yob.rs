extern crate yob;

use yob::cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();
    println!("{:?}", cpu);
    cpu.execute_instruction(0x41);
    println!("{:?}", cpu);
    // cpu.execute_instruction(0x4C);
    // println!("{:?}", cpu);
    cpu.execute_instruction(0x04);
    println!("{:?}", cpu);
}
