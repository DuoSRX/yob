use std;
use std::fmt;

use memory::Memory;
use registers::*;

// TODO: Flag management (haven't implemented N H Z)
// TODO: Interrupts
// TODO: Branching instructions
// TODO: More 16 bits operations
// TODO: CB Instructions
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

    pub fn pop_byte(&mut self) -> u8 {
        let sp = self.registers.sp;
        let byte = self.load_byte(sp);
        self.registers.sp += 1;
        byte
    }

    pub fn load_word(&mut self, address: u16) -> u16 {
        let hi = (self.memory.load(address + 1) as u16) << 8;
        let lo = self.memory.load(address) as u16;
        hi | lo
    }

    pub fn store_word(&mut self, address: u16, value: u16) {
        let lo = value & 0xFF;
        let hi = (value >> 8) & 0xFF;
        self.store_byte(address, lo as u8);
        self.store_byte(address + 1, hi as u8);
    }

    pub fn pop_word(&mut self) -> u16 {
        let sp = self.registers.sp;
        let word = self.load_word(sp);
        self.registers.sp += 2;
        word
    }

    pub fn step(&mut self) {
        let instr = self.load_byte_and_inc_pc();
        self.execute_instruction(instr);
    }

    pub fn execute_instruction(&mut self, instr: u8) {
        let prefix: u16 = (instr as u16) >> 8;

        match prefix {
            0xCB => self.execute_cb_instruction(instr),
            _    => self.execute_regular_instruction(instr),
        }
    }

    fn execute_regular_instruction(&mut self, instr: u8) {
        use registers::Register8::{A,B,C,D,E,H,L};
        use registers::Register16::{AF,BC,DE,HL};

        match instr {
            0x00 => { }, // NOP
            // 0x01 => LD BC,&0000
            0x02 => self.ld(BC, A),
            0x03 => self.inc(BC),
            0x04 => self.inc(B),
            0x05 => self.dec(B),
            0x06 => self.ld(B, ImmediateStorage),
            0x07 => self.rlca(),
            // 0x08 => LD (nn), SP
            // 0x09 => self.add(HL, BC),
            0x0A => self.ld(A, BC),
            0x0B => self.dec(BC),
            0x0C => self.inc(C),
            0x0D => self.dec(C),
            0x0E => self.ld(C, ImmediateStorage),
            0x0F => self.rrca(),
            0x10 => panic!("Got STOP!"),
            // 0x11 => LD DE,&0000
            0x12 => self.ld(DE, A),
            0x13 => self.inc(DE),
            0x14 => self.inc(D),
            0x15 => self.dec(D),
            0x16 => self.ld(D, ImmediateStorage),
            0x17 => self.rla(),
            // 0x18 => JR &4546,
            // 0x19 => self.add(HL, DE),
            0x1A => self.ld(A, DE),
            0x1B => self.dec(DE),
            0x1C => self.inc(E),
            0x1D => self.dec(E),
            0x1E => self.ld(E, ImmediateStorage),
            0x1F => self.rra(),
            // 0x20 => JR NZ,&4546
            // 0x21 => LD HL,*0000
            // 0x22 => LDI (HL),A
            0x23 => self.inc(HL),
            0x24 => self.inc(H),
            0x25 => self.dec(H),
            0x26 => self.ld(H, ImmediateStorage),
            // 0x27 => DAA,
            // 0x28 => JR Z, &4546
            // 0x29 => self.add(HL, HL),
            // 0x2A => LDI A,(HL)
            0x2B => self.dec(HL),
            0x2C => self.inc(L),
            0x2D => self.dec(L),
            0x2E => self.ld(L, ImmediateStorage),
            0x2F => self.cpl(),
            // 0x30 => JR NC,&4546,
            // 0x31 => LD SP,&0000
            // 0x32 => LDD A,(HL)
            // 0x33 => self.inc(SP),
            0x34 => self.inc(HL),
            0x35 => self.dec(HL),
            0x36 => self.ld(HL, ImmediateStorage),
            0x37 => self.scf(),
            // 0x38 => JR C,&4546
            // 0x39 => self.add(HL, SP)
            // 0x3A => LDD A,(HL)
            // 0x3B => self.dec(SP),
            0x3C => self.inc(A),
            0x3D => self.dec(A),
            0x3E => self.ld(A, ImmediateStorage),
            0x3F => self.ccf(),
            0x40 => self.ld(B, B),
            0x41 => self.ld(B, C),
            0x42 => self.ld(B, D),
            0x43 => self.ld(B, E),
            0x44 => self.ld(B, H),
            0x45 => self.ld(B, L),
            0x46 => self.ld(B, HL),
            0x47 => self.ld(B, A),
            0x48 => self.ld(C, B),
            0x49 => self.ld(C, C),
            0x4A => self.ld(C, D),
            0x4B => self.ld(C, E),
            0x4C => self.ld(C, H),
            0x4D => self.ld(C, L),
            0x4E => self.ld(C, HL),
            0x4F => self.ld(C, A),
            0x50 => self.ld(D, B),
            0x51 => self.ld(D, C),
            0x52 => self.ld(D, D),
            0x53 => self.ld(D, E),
            0x54 => self.ld(D, H),
            0x55 => self.ld(D, L),
            0x56 => self.ld(D, HL),
            0x57 => self.ld(D, A),
            0x58 => self.ld(E, B),
            0x59 => self.ld(E, C),
            0x5A => self.ld(E, D),
            0x5B => self.ld(E, E),
            0x5C => self.ld(E, H),
            0x5D => self.ld(E, L),
            0x5E => self.ld(E, HL),
            0x5F => self.ld(E, A),
            0x60 => self.ld(H, B),
            0x61 => self.ld(H, C),
            0x62 => self.ld(H, D),
            0x63 => self.ld(H, E),
            0x64 => self.ld(H, H),
            0x65 => self.ld(H, L),
            0x66 => self.ld(H, HL),
            0x67 => self.ld(H, A),
            0x68 => self.ld(L, B),
            0x69 => self.ld(L, C),
            0x6A => self.ld(L, D),
            0x6B => self.ld(L, E),
            0x6C => self.ld(L, H),
            0x6D => self.ld(L, L),
            0x6E => self.ld(L, HL),
            0x6F => self.ld(L, A),
            0x70 => self.ld(HL, B),
            0x71 => self.ld(HL, C),
            0x72 => self.ld(HL, D),
            0x73 => self.ld(HL, E),
            0x74 => self.ld(HL, H),
            0x75 => self.ld(HL, L),
            // 0x76 => HALT,
            0x77 => self.ld(HL, A),
            0x78 => self.ld(A, B),
            0x79 => self.ld(A, C),
            0x7A => self.ld(A, D),
            0x7B => self.ld(A, E),
            0x7C => self.ld(A, H),
            0x7D => self.ld(A, L),
            0x7E => self.ld(A, HL),
            0x7F => self.ld(A, A), // NOP?
            0x80 => self.add(B),
            0x81 => self.add(C),
            0x82 => self.add(D),
            0x83 => self.add(E),
            0x84 => self.add(H),
            0x85 => self.add(L),
            0x86 => self.add(HL),
            0x87 => self.add(A),
            0x88 => self.adc(B),
            0x89 => self.adc(C),
            0x8A => self.adc(D),
            0x8B => self.adc(E),
            0x8C => self.adc(H),
            0x8D => self.adc(L),
            0x8E => self.adc(HL),
            0x8F => self.adc(A),
            0x90 => self.sub(B),
            0x91 => self.sub(C),
            0x92 => self.sub(D),
            0x93 => self.sub(E),
            0x94 => self.sub(H),
            0x95 => self.sub(L),
            0x96 => self.sub(HL),
            0x97 => self.sub(A),
            0x98 => self.sbc(B),
            0x99 => self.sbc(C),
            0x9A => self.sbc(D),
            0x9B => self.sbc(E),
            0x9C => self.sbc(H),
            0x9D => self.sbc(L),
            0x9E => self.sbc(HL),
            0x9F => self.sbc(A),
            0xA0 => self.and(B),
            0xA1 => self.and(C),
            0xA2 => self.and(D),
            0xA3 => self.and(E),
            0xA4 => self.and(H),
            0xA5 => self.and(L),
            0xA6 => self.and(HL),
            0xA7 => self.and(A),
            0xA8 => self.xor(B),
            0xA9 => self.xor(C),
            0xAA => self.xor(D),
            0xAB => self.xor(E),
            0xAC => self.xor(H),
            0xAD => self.xor(L),
            0xAE => self.xor(HL),
            0xAF => self.xor(A),
            0xB0 => self.or(B),
            0xB1 => self.or(C),
            0xB2 => self.or(D),
            0xB3 => self.or(E),
            0xB4 => self.or(H),
            0xB5 => self.or(L),
            0xB6 => self.or(HL),
            0xB7 => self.or(A),
            0xB8 => self.cp(B),
            0xB9 => self.cp(C),
            0xBA => self.cp(D),
            0xBB => self.cp(E),
            0xBC => self.cp(H),
            0xBD => self.cp(L),
            0xBE => self.cp(HL),
            0xBF => self.cp(A),
            // 0xC0 => RET NZ
            0xC1 => self.pop(BC),
            // 0xC2 => JP NZ,&0000
            // 0xC3 => JP &0000
            // 0xC4 => CALL NZ,&0000
            0xC5 => self.push(BC),
            0xC6 => self.add(ImmediateStorage),
            // 0xC7 => RST &00
            // 0xC8 => RET Z
            0xC9 => self.ret(),
            // 0xCA => JP Z,&0000
            // 0xCB => ????
            // 0xCC=> CALL Z,&0000,
            // 0xCD => CALL &0000,
            0xCE => self.adc(ImmediateStorage),
            // 0xCF => RST &08,
            // 0xD0 => RET NC
            0xD1 => self.pop(DE),
            // 0xD2 => JP NC,&0000,
            // 0xD3 => illegal
            // 0xD4 => CALL NC,&0000
            0xD5 => self.push(DE),
            0xD6 => self.sub(ImmediateStorage),
            // 0xD7 => RST &10
            // 0xD8 => RET C
            // 0xD9 => RETI
            // 0xDA => JP C,&0000
            // 0xDB => illegal
            // 0xDC => CALL C,&0000
            // 0xDD => illegal
            0xDE => self.sbc(ImmediateStorage),
            // 0xDF => RST &18
            // 0xE0 => LD (FF00+n),A
            0xE1 => self.pop(HL),
            // 0xE2 => LD (FF00+C),A
            // 0xE3 => illegal
            // 0xE4 => illegal
            0xE5 => self.push(HL),
            0xE6 => self.and(ImmediateStorage),
            // 0xE7 => RST &20
            // 0xE8 => ADD SP,dd
            // 0xE9 => JP (HL)
            // 0xEA => LD (nn),A
            // 0xEB => illegal
            // 0xEC => illegal
            // 0xED => illegal
            0xEE => self.xor(ImmediateStorage),
            // 0xEF => RST &28
            // Not familiar with z80 yet but I think this is zero page?
            // 0xF0 => LD A,(FF00+n)
            0xF1 => self.pop(AF),
            // 0xF2 => LD A,(FF00+C)
            // 0xF3 => DI
            // 0xF4 => illegal
            0xF5 => self.push(AF),
            0xF6 => self.or(ImmediateStorage),
            // 0xF7 => RST &30
            // 0xF8 => LD HL,SP+dd
            // 0xF9 => LD SP,HL
            // 0xFA => LD A,(nn),
            // 0xFB => EI
            // 0xFC => illegal
            // 0xFD => illegal
            0xFE => self.cp(ImmediateStorage),

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

    fn pop(&mut self, register: Register16) {
        let sp = self.registers.sp;
        let word = self.load_word(sp);
        self.registers.sp = sp.wrapping_add(2);
        self.registers.store_16(register, word);
    }

    fn push(&mut self, register: Register16) {
        let sp = self.registers.sp.wrapping_sub(2);
        let value = self.registers.load_16(register);
        self.store_word(sp, value);
        self.registers.sp = sp;
    }

    fn and<S: Storage>(&mut self, s: S) {
        let value = self.registers.a & s.load(self);
        self.registers.a = value;
        self.registers.set_zero(value == 0);
    }

    fn or<S: Storage>(&mut self, s: S) {
        let value = self.registers.a | s.load(self);
        self.registers.a = value;
        self.registers.set_zero(value == 0);
    }

    fn xor<S: Storage>(&mut self, s: S) {
        let value = self.registers.a ^ s.load(self);
        self.registers.a = value;
        self.registers.set_zero(value == 0);
    }

    fn rlca(&mut self) {
        let value = self.registers.a.rotate_left(1);
        self.registers.set_carry(value & 1 != 0);
        self.registers.a = value;
        // let a = self.registers.a;
        // self.registers.set_carry(a & 0x80 != 0);
        // self.registers.a = (a << 1) | self.registers.carry();
    }

    fn rrca(&mut self) {
        let value = self.registers.a.rotate_right(1);
        self.registers.set_carry(value & 0x80 != 0);
        self.registers.a = value;
        // let a = self.registers.a;
        // self.registers.set_carry(a & 1 != 0);
        // self.registers.a = (a >> 1) | (self.registers.carry() << 7);
    }

    fn rla(&mut self) {
        let a = self.registers.a;
        let value = (a << 1) | self.registers.carry();
        self.registers.set_carry(a & 0x80 != 0);
        self.registers.a = value;
    }

    fn rra(&mut self) {
        let a = self.registers.a;
        let value = (a >> 1) | (self.registers.carry() << 7);
        self.registers.set_carry(a & 1 != 0);
        self.registers.a = value;
    }

    // Generic addition for ADD and SBC
    fn add_op(&mut self, value: u8, carry: bool) {
        let mut result = (value as u16).wrapping_add(self.registers.a as u16);
        if carry { result = result.wrapping_add(self.registers.carry() as u16) }
        self.registers.set_carry(result & 0x100 != 0);
        self.registers.set_zero(result == 0);
        self.registers.a = (result as u8) & 0xFF;
    }

    fn add<S: Storage>(&mut self, s: S) {
        let value = s.load(self);
        self.add_op(value, false);
    }

    fn adc<S: Storage>(&mut self, s: S) {
        let value = s.load(self);
        self.add_op(value, true);
    }

    // Generic subtraction for SUB, SBC and CP
    fn sub_op(&mut self, value: u8, carry: bool) -> u8 {
        let mut result = (self.registers.a as u16).wrapping_sub(value as u16);
        if carry { result = result.wrapping_sub(self.registers.carry() as u16) }
        self.registers.set_carry(result & 0x100 != 0);
        self.registers.set_zero(result == 0);
        (result as u8) & 0xFF
    }

    fn sub<S: Storage>(&mut self, s: S) {
        let value = s.load(self);
        self.registers.a = self.sub_op(value, false);
    }

    fn sbc<S: Storage>(&mut self, s: S) {
        let value = s.load(self);
        self.registers.a = self.sub_op(value, true);
    }

    fn cp<S: Storage>(&mut self, s: S) {
        let value = s.load(self);
        self.sub_op(value, false);
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

    fn ret(&mut self) {
        self.registers.pc = self.pop_word();
    }

    fn ccf(&mut self) {
        // TODO: Reset N and H
        let carry = self.registers.test_flag(CARRY_FLAG);
        self.registers.set_carry(!carry);
    }

    fn scf(&mut self) {
        // TODO: Reset N and H
        self.registers.set_carry(true);
    }

    fn cpl(&mut self) {
        self.registers.a = !self.registers.a;
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

impl Storage for Register16 {
    fn load(&self, cpu: &mut Cpu) -> u8 {
        let address = match *self {
            Register16::AF => cpu.registers.af(),
            Register16::BC => cpu.registers.bc(),
            Register16::DE => cpu.registers.de(),
            Register16::HL => cpu.registers.hl(),
        };
        cpu.load_byte(address)
    }

    fn store(&self, cpu: &mut Cpu, value: u8) {
        let address = match *self {
            Register16::AF => cpu.registers.af(),
            Register16::BC => cpu.registers.bc(),
            Register16::DE => cpu.registers.de(),
            Register16::HL => cpu.registers.hl(),
        };
        cpu.store_byte(address, value)
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.registers))
    }
}
