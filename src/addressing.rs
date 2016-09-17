use std;
use std::fmt;

use cpu::Cpu;
use registers::*;

pub trait Storage {
    fn load(&self, &mut Cpu) -> u8;
    // I wonder if I should split Storage in Input/Output to avoid this default impl?
    fn store(&self, &mut Cpu, u8) { panic!("Impossible storage type") }
}

pub struct ImmediateStorage;
impl Storage for ImmediateStorage {
    fn load(&self, cpu: &mut Cpu) -> u8 {
        cpu.load_byte_and_inc_pc()
    }
}

pub enum Indirect { BC, DE, HL, ZeroPage, ZeroPageRegC }
impl Storage for Indirect {
    fn load(&self, cpu: &mut Cpu) -> u8 {
        // TODO: Refactor this. I's duplicated in load and store. Put in CPU maybe?
        let address = match *self {
            Indirect::BC => cpu.registers.bc(),
            Indirect::DE => cpu.registers.de(),
            Indirect::HL => cpu.registers.hl(),
            Indirect::ZeroPage => 0xFF00 as u16 | cpu.load_byte_and_inc_pc() as u16,
            Indirect::ZeroPageRegC => 0xFF00 as u16 | cpu.registers.c as u16,
        };

        cpu.load_byte(address)
    }

    fn store(&self, cpu: &mut Cpu, value: u8) {
        // TODO: Refactor this. I's duplicated in load and store. Put in CPU maybe?
        let address = match *self {
            Indirect::BC => cpu.registers.bc(),
            Indirect::DE => cpu.registers.de(),
            Indirect::HL => cpu.registers.hl(),
            Indirect::ZeroPage => 0xFF00 as u16 | cpu.load_byte_and_inc_pc() as u16,
            Indirect::ZeroPageRegC => 0xFF00 as u16 | cpu.registers.c as u16,
        };

        cpu.store_byte(address, value);
    }
}

impl Storage for Register8 {
    fn load(&self, cpu: &mut Cpu) -> u8 {
        match *self {
            Register8::A => cpu.registers.a,
            Register8::B => cpu.registers.b,
            Register8::C => cpu.registers.c,
            Register8::D => cpu.registers.d,
            Register8::E => cpu.registers.e,
            Register8::F => cpu.registers.f,
            Register8::H => cpu.registers.h,
            Register8::L => cpu.registers.l,
        }
    }

    fn store(&self, cpu: &mut Cpu, value: u8) {
        match *self {
            Register8::A => cpu.registers.a = value,
            Register8::B => cpu.registers.b = value,
            Register8::C => cpu.registers.c = value,
            Register8::D => cpu.registers.d = value,
            Register8::E => cpu.registers.e = value,
            Register8::F => cpu.registers.f = value,
            Register8::H => cpu.registers.h = value,
            Register8::L => cpu.registers.l = value,
        }
    }
}

impl Storage for Register16 {
    fn load(&self, cpu: &mut Cpu) -> u8 {
        let address = match *self {
            Register16::AF => cpu.registers.af(),
            Register16::BC => cpu.registers.bc(),
            Register16::DE => cpu.registers.de(),
            Register16::HL => cpu.registers.hl(),
            Register16::SP => cpu.registers.sp,
        };

        cpu.load_byte(address)
    }

    fn store(&self, cpu: &mut Cpu, value: u8) {
        let address = match *self {
            Register16::AF => cpu.registers.af(),
            Register16::BC => cpu.registers.bc(),
            Register16::DE => cpu.registers.de(),
            Register16::HL => cpu.registers.hl(),
            Register16::SP => cpu.registers.sp,
        };

        cpu.store_byte(address, value)
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.registers))
    }
}
