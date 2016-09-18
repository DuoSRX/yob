extern crate yob;

use std::fs::File;

use yob::cartridge::Cartridge;
use yob::cpu::Cpu;

fn main() {
    // let mut file = File::open("roms/tetris.gb").unwrap();
    // let cartridge = Cartridge::load(&mut file);
    // println!("{:?}", cartridge.rom);
    let mut cpu = Cpu::new();
    cpu.reset();

    // for _ in 1..20 {
    loop {
        cpu.step();
    }

    // println!("{:?}", cpu);
    // cpu.step();
    // cpu.step();
    // cpu.step();
    // cpu.step();
}
