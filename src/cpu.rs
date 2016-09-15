use std;
use std::fmt;

use memory::Memory;
use registers::{Registers, Register8};

// TODO: Shit ton of instructions
// TODO: CB Instructions
// TODO: Memory addressing
// TODO: 16 bit registers
// TODO: Flag management in INC, DEC, LD and... all the rest

pub struct Cpu {
    pub registers: Registers,
    pub memory: Memory,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
            memory: Memory::new(),
        }
    }

    pub fn load_byte_and_inc_pc(&mut self) -> u8 {
        let pc = self.registers.pc;
        self.registers.pc += 1;
        self.load_byte(pc)
    }

    pub fn load_byte(&mut self, address: u16) -> u8 {
        self.memory.load(address)
    }

    pub fn store_byte(&mut self, address: u16, value: u8) {
        self.memory.store(address, value);
    }

    pub fn step(&mut self) {
        let instr = self.load_byte_and_inc_pc();
        println!("Executing {:02x}", instr);
        self.execute_instruction(instr);
    }

    pub fn execute_instruction(&mut self, instr: u8) {
        use registers::Register8::{A,B,C,D,E,H,L};

        match instr {
            0x00 => { }, // NOP

            // 0x01 TODO: LD BC,&0000
            // 0x02 TODO: LD (BC),A
            // 0x03 => self.inc(BC),
            0x04 => self.inc(B),
            0x05 => self.dec(B),
            0x06 => self.ld(B, ImmediateStorage),
            0x0C => self.inc(C),
            0x0D => self.dec(C),
            0x0E => self.ld(C, ImmediateStorage),
            0x14 => self.inc(D),
            0x15 => self.dec(D),
            0x16 => self.ld(D, ImmediateStorage),
            0x1C => self.inc(E),
            0x1D => self.dec(E),
            0x1E => self.ld(E, ImmediateStorage),
            0x24 => self.inc(H),
            0x25 => self.dec(H),
            0x26 => self.ld(H, ImmediateStorage),
            0x2C => self.inc(L),
            0x2D => self.dec(L),
            0x2E => self.ld(L, ImmediateStorage),
            0x3C => self.inc(A),
            0x3D => self.dec(A),
            0x3E => self.ld(A, ImmediateStorage),

            0x40 => self.ld(B, B),
            0x41 => self.ld(B, C),
            0x42 => self.ld(B, D),
            0x43 => self.ld(B, E),
            0x44 => self.ld(B, H),
            0x45 => self.ld(B, L),
            // 0x46 TODO: LD B,(HL)
            0x47 => self.ld(B, A),
            0x48 => self.ld(C, B),
            0x49 => self.ld(C, C),
            0x4A => self.ld(C, D),
            0x4B => self.ld(C, E),
            0x4C => self.ld(C, H),
            0x4D => self.ld(C, L),
            // 0x4E TODO: LD C,(HL)
            0x4F => self.ld(C, A),
            0x50 => self.ld(D, B),
            0x51 => self.ld(D, C),
            0x52 => self.ld(D, D),
            0x53 => self.ld(D, E),
            0x54 => self.ld(D, H),
            0x55 => self.ld(D, L),
            // 0x56 TODO: LD D,(HL)
            0x57 => self.ld(D, A),
            0x58 => self.ld(E, B),
            0x59 => self.ld(E, C),
            0x5A => self.ld(E, D),
            0x5B => self.ld(E, E),
            0x5C => self.ld(E, H),
            0x5D => self.ld(E, L),
            // 0x5E TODO: LD E,(HL)
            0x5F => self.ld(E, A),
            0x60 => self.ld(H, B),
            0x61 => self.ld(H, C),
            0x62 => self.ld(H, D),
            0x63 => self.ld(H, E),
            0x64 => self.ld(H, H),
            0x65 => self.ld(H, L),
            // 0x66 TODO: LD H,(HL)
            0x67 => self.ld(H, A),
            0x68 => self.ld(L, B),
            0x69 => self.ld(L, C),
            0x6A => self.ld(L, D),
            0x6B => self.ld(L, E),
            0x6C => self.ld(L, H),
            0x6D => self.ld(L, L),
            // 0x6E TODO: LD L,(HL)
            0x6F => self.ld(L, A),

            0x87 => self.add(A),
            0x8F => self.adc(A),

            instr => panic!("{}: Instruction not implemented yet", instr)
        }
    }

    pub fn execute_cb_instruction(&mut self, instr: u8) {
        match instr {
            _ => panic!("{}: CB Instruction not implemented yet", instr)
        }
    }

    fn ld<In: Storage, Out: Storage>(&mut self, a: Out, b: In) {
        let value = b.load(self);
        a.store(self, value);
    }

    fn add<S: Storage>(&mut self, s: S) {
        let value = s.load(self);
        self.registers.a = self.registers.a.wrapping_add(value);
    }

    fn adc<S: Storage>(&mut self, s: S) {
        let value = s.load(self);
        let carry = self.registers.carry();
        let result = self.registers.a.wrapping_add(value).wrapping_add(carry);
        self.registers.set_zero(value == 0);
        self.registers.set_carry(self.registers.a as u16, value as u16);
        self.registers.a = result;
    }

    fn inc<S: Storage>(&mut self, storage: S) {
        let value = storage.load(self).wrapping_add(1);
        self.registers.set_zero(value == 0);
        storage.store(self, value);
    }

    fn dec<S: Storage>(&mut self, storage: S) {
        let value = storage.load(self).wrapping_sub(1);
        self.registers.set_zero(value == 0);
        storage.store(self, value);
    }
}

pub trait Storage {
    fn load(&self, &mut Cpu) -> u8;
    fn store(&self, &mut Cpu, u8);
}

struct ImmediateStorage;
impl Storage for ImmediateStorage {
    fn load(&self, cpu: &mut Cpu) -> u8 {
        cpu.load_byte_and_inc_pc()
    }
    // I wonder if I should split Storage in Input/Output to avoid this?
    fn store(&self, _cpu: &mut Cpu, _value: u8) { panic!("Can't store immediae") }
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

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.registers))
    }
}
