use addressing::*;
use memory::Memory;
use registers::*;

// TODO: Better flag management (haven't implemented N and H)
// TODO: Interrupts
// TODO: CB Instructions
// FIXME: Refactor JP/JR/CALL/RET conditionals. There's lot of duplication.
pub struct Cpu {
    pub registers: Registers,
    pub memory: Memory,
    pub halt: bool,
    pub interrupt: bool,
    pub cycles: u64
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
            memory: Memory::new(),
            halt: false,
            interrupt: false,
            cycles: 0,
        }
    }

    pub fn reset(&mut self) {
        // TODO: Set default memory values
        // See http://gbdev.gg8.se/wiki/articles/Power_Up_Sequence
        self.registers.reset();
        self.registers.pc = 0x100;
    }

    pub fn step(&mut self) -> u64 {
        // let pc = self.registers.pc;
        // let sp = self.registers.sp;
        // let b = self.load_byte(pc);
        // print!("{:04x} {:02x}  {:?}", pc, b, self);
        // println!(" Stack:{:04x}", self.load_word(sp));

        // Hard coded fake interrupts to boot Tetris
        match self.registers.pc {
            0x02B4 => { self.memory.gpu.ly = 0x94 }
            0x287E => { self.memory.gpu.ly = 0x91 }
            _ => {}
        }
        let instruction = self.load_byte_and_inc_pc();
        self.execute_instruction(instruction);

        if self.interrupt {
            self.interrupt();
        }

        1u64 // FIXME: replace with real cycles count
    }

    pub fn execute_instruction(&mut self, instr: u8) {
        use registers::Register8::{A,B,C,D,E,H,L};
        use registers::Register16::{AF,BC,DE,HL,SP};

        match instr {
            0x00 => { }, // NOP
            0x01 => self.ld_word_immediate(BC),
            0x02 => self.ld(BC, A),
            0x03 => self.inc_16(BC),
            0x04 => self.inc(B),
            0x05 => self.dec(B),
            0x06 => self.ld(B, ImmediateStorage),
            0x07 => self.rlca(),
            0x08 => self.ld_indirect_sp(),
            0x09 => self.add_hl(BC),
            0x0A => self.ld(A, BC),
            0x0B => self.dec_16(BC),
            0x0C => self.inc(C),
            0x0D => self.dec(C),
            0x0E => self.ld(C, ImmediateStorage),
            0x0F => self.rrca(),
            0x10 => panic!("Got STOP!"),
            0x11 => self.ld_word_immediate(DE),
            0x12 => self.ld(DE, A),
            0x13 => self.inc_16(DE),
            0x14 => self.inc(D),
            0x15 => self.dec(D),
            0x16 => self.ld(D, ImmediateStorage),
            0x17 => self.rla(),
            0x18 => self.jr(),
            0x19 => self.add_hl(DE),
            0x1A => self.ld(A, DE),
            0x1B => self.dec(DE),
            0x1C => self.inc(E),
            0x1D => self.dec(E),
            0x1E => self.ld(E, ImmediateStorage),
            0x1F => self.rra(),
            0x20 => self.jr_unless(ZERO_FLAG),
            0x21 => self.ld_word_immediate(HL),
            0x22 => self.ld(Indirect::HLI, A),
            0x23 => self.inc_16(HL),
            0x24 => self.inc(H),
            0x25 => self.dec(H),
            0x26 => self.ld(H, ImmediateStorage),
            0x27 => self.daa(),
            0x28 => self.jr_if(ZERO_FLAG),
            0x29 => self.add_hl(HL),
            0x2A => self.ld(A, Indirect::HLI),
            0x2B => self.dec_16(HL),
            0x2C => self.inc(L),
            0x2D => self.dec(L),
            0x2E => self.ld(L, ImmediateStorage),
            0x2F => self.cpl(),
            0x30 => self.jr_unless(CARRY_FLAG),
            0x31 => self.ld_word_immediate(SP),
            0x32 => self.ld(Indirect::HLD, A),
            0x33 => self.inc(SP),
            0x34 => self.inc(HL),
            0x35 => self.dec(HL),
            0x36 => self.ld(HL, ImmediateStorage),
            0x37 => self.scf(),
            0x38 => self.jr_if(CARRY_FLAG),
            0x39 => self.add_hl(SP),
            0x3A => self.ld(A, Indirect::HLD),
            0x3B => self.dec_16(SP),
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
            0x76 => { self.halt = true },
            0x77 => self.ld(HL, A),
            0x78 => self.ld(A, B),
            0x79 => self.ld(A, C),
            0x7A => self.ld(A, D),
            0x7B => self.ld(A, E),
            0x7C => self.ld(A, H),
            0x7D => self.ld(A, L),
            0x7E => self.ld(A, HL),
            0x7F => self.ld(A, A),
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
            0xC0 => self.ret_unless(ZERO_FLAG),
            0xC1 => self.pop(BC),
            0xC2 => self.jp_unless(ZERO_FLAG),
            0xC3 => self.jp(),
            0xC4 => self.call_unless(ZERO_FLAG),
            0xC5 => self.push(BC),
            0xC6 => self.add(ImmediateStorage),
            0xC7 => self.rst(0x00),
            0xC8 => self.ret_if(ZERO_FLAG),
            0xC9 => self.ret(),
            0xCA => self.jp_if(ZERO_FLAG),
            0xCB => self.cb(),
            0xCC => self.call_if(ZERO_FLAG),
            0xCD => self.call(),
            0xCE => self.adc(ImmediateStorage),
            0xCF => self.rst(0x08),
            0xD0 => self.ret_unless(CARRY_FLAG),
            0xD1 => self.pop(DE),
            0xD2 => self.jp_unless(CARRY_FLAG),
            0xD3 => self.illegal(instr),
            0xD4 => self.call_unless(CARRY_FLAG),
            0xD5 => self.push(DE),
            0xD6 => self.sub(ImmediateStorage),
            0xD7 => self.rst(0x10),
            0xD8 => self.ret_if(CARRY_FLAG),
            0xD9 => self.reti(),
            0xDA => self.jp_if(CARRY_FLAG),
            0xDB => self.illegal(instr),
            0xDC => self.call_if(CARRY_FLAG),
            0xDD => self.illegal(instr),
            0xDE => self.sbc(ImmediateStorage),
            0xDF => self.rst(0x18),
            0xE0 => self.ld(Indirect::ZeroPage, A),
            0xE1 => self.pop(HL),
            0xE2 => self.ld(Indirect::ZeroPageRegC, A),
            0xE3 => self.illegal(instr),
            0xE4 => self.illegal(instr),
            0xE5 => self.push(HL),
            0xE6 => self.and(ImmediateStorage),
            0xE7 => self.rst(0x20),
            0xE8 => self.add_sp(),
            0xE9 => self.jp_hl(),
            0xEA => self.ld(Indirect::Immediate, A),
            0xEB => self.illegal(instr),
            0xEC => self.illegal(instr),
            0xED => self.illegal(instr),
            0xEE => self.xor(ImmediateStorage),
            0xEF => self.rst(0x28),
            0xF0 => self.ld(A, Indirect::ZeroPage),
            0xF1 => self.pop(AF),
            0xF2 => self.ld(A, Indirect::ZeroPageRegC),
            0xF3 => self.di(),
            0xF4 => self.illegal(instr),
            0xF5 => self.push(AF),
            0xF6 => self.or(ImmediateStorage),
            0xF7 => self.rst(0x30),
            0xF8 => self.ld_hl_sp(),
            0xF9 => self.ld_sp_hl(),
            0xFA => self.ld(A, Indirect::Immediate),
            0xFB => self.ei(),
            0xFC => self.illegal(instr),
            0xFD => self.illegal(instr),
            0xFE => self.cp(ImmediateStorage),
            0xFF => self.rst(0x38),

            instr => panic!("{}: Instruction not implemented yet", instr)
        }
    }

    pub fn execute_cb_instruction(&mut self, instr: u8) {
        use registers::Register8::{A,B,C,D,E,H,L};
        // use registers::Register16::{AF,BC,DE,HL,SP};

        match instr {
            0x30 => self.swap(B),
            0x31 => self.swap(C),
            0x32 => self.swap(D),
            0x33 => self.swap(E),
            0x34 => self.swap(H),
            0x35 => self.swap(L),
            // 0x36 => self.swap(HL), TODO
            0x37 => self.swap(A),
            0x47 => self.bit(0, A),
            // 0x80 => self.res(0, B),
            // 0x81 => self.res(0, C),
            // 0x82 => self.res(0, D),
            // 0x83 => self.res(0, E),
            // 0x84 => self.res(0, H),
            // 0x85 => self.res(0, L),
            // 0x86 => self.res(0, Indirect::HL),
            0x87 => self.res(0, A),
            // 0x88 => self.res(1, B),
            // 0x89 => self.res(1, C),
            // 0x8A => self.res(1, D),
            // 0x8B => self.res(1, E),
            // 0x8C => self.res(1, H),
            // 0x8D => self.res(1, L),
            // 0x8E => self.res(1, Indirect::HL),
            // 0x8F => self.res(1, A),
            0xC7 => self.set(0, A),

            _ => panic!("{:02x}: CB Instruction not implemented yet", instr)
        }
    }

    // Utility functions

    pub fn load_byte_and_inc_pc(&mut self) -> u8 {
        let pc = self.registers.pc;
        self.registers.pc += 1;
        self.load_byte(pc)
    }

    pub fn load_byte(&mut self, address: u16) -> u8 {
        self.cycles += 1;
        self.memory.load(address)
    }

    pub fn store_byte(&mut self, address: u16, value: u8) {
        self.cycles += 1;
        self.memory.store(address, value);
    }

    pub fn pop_byte(&mut self) -> u8 {
        let sp = self.registers.sp;
        let byte = self.load_byte(sp);
        self.registers.sp += 1;
        byte
    }

    pub fn load_word_and_inc_pc(&mut self) -> u16 {
        let pc = self.registers.pc;
        self.registers.pc += 2;
        self.load_word(pc)
    }

    pub fn load_word(&mut self, address: u16) -> u16 {
        self.cycles += 2;
        let hi = (self.memory.load(address + 1) as u16) << 8;
        let lo = self.memory.load(address) as u16;
        hi | lo
    }

    pub fn store_word(&mut self, address: u16, value: u16) {
        self.cycles += 2;
        let lo = value & 0xFF;
        let hi = (value >> 8) & 0xFF;
        self.store_byte(address, lo as u8);
        self.store_byte(address + 1, hi as u8);
    }

    pub fn push_word(&mut self, value: u16) {
        let sp = self.registers.sp.wrapping_sub(2);
        if value == 0xF0C9 {
            panic!("{:?}", self);
        }
        self.store_word(sp, value);
        self.registers.sp = sp;
    }

    pub fn pop_word(&mut self) -> u16 {
        let sp = self.registers.sp;
        let word = self.load_word(sp);
        self.registers.sp += 2;
        word
    }

    /*
     * http://gbdev.gg8.se/files/docs/mirrors/pandocs.html#interrupts
     * Bit 0: V-Blank  INT 40h
     * Bit 1: LCD STAT INT 48h
     * Bit 2: Timer    INT 50h
     * Bit 3: Serial   INT 58h
     * Bit 4: Joypad   INT 60h
     */
    fn interrupt(&mut self) {
        let int_enable = self.memory.interrupt_enable;
        let int_flags = self.memory.interrupt_flags;
        let interrupts = int_enable & int_flags;

        if interrupts != 0 {
            let int_number = interrupts.trailing_zeros();
            // TODO: Check for nested interrupts
            // Reset the triggered interrupt flag
            self.memory.interrupt_flags &= !(1 << int_number);
            self.interrupt = false;
            match int_number {
                0 => self.rst(0x40),
                _ => { } // TODO: Other interrupts
            }
        }
    }

    // Instructions implementations

    fn cb(&mut self) {
        let instruction = self.load_byte_and_inc_pc();
        self.execute_cb_instruction(instruction);
    }

    fn illegal(&self, instruction: u8) {
        panic!("Illegal opcode {}", instruction);
    }

    fn ld<In: Storage, Out: Storage>(&mut self, a: Out, b: In) {
        let value = b.load(self);
        a.store(self, value);
    }

    fn ld_sp_hl(&mut self) {
        let value = self.registers.hl();
        self.registers.sp = value;
    }

    // LD HL,SP+e
    fn ld_hl_sp(&mut self) {
        // TODO: Handle flags
        // FIXME: Signed arithmetics?
        let offset = self.load_byte_and_inc_pc() as u16;
        let sp = self.registers.sp;
        let address = offset.wrapping_add(sp);
        self.registers.store_16(Register16::HL, address);
    }

    fn ld_word_immediate(&mut self, register: Register16) {
        let value = self.load_word_and_inc_pc();
        self.registers.store_16(register, value);
    }

    fn ld_indirect_sp(&mut self) {
        let value = self.registers.sp;
        let address = self.load_word_and_inc_pc();
        self.store_word(address, value);
    }

    fn pop(&mut self, register: Register16) {
        let word = self.pop_word();
        self.registers.store_16(register, word);
    }

    fn push(&mut self, register: Register16) {
        let value = self.registers.load_16(register);
        self.push_word(value);
    }

    fn and<S: Storage>(&mut self, s: S) {
        let value = self.registers.a & s.load(self);
        self.registers.a = value;
        self.registers.f = 0;
        self.registers.set_zero(value == 0);
    }

    fn or<S: Storage>(&mut self, s: S) {
        let value = self.registers.a | s.load(self);
        self.registers.a = value;
        self.registers.f = 0;
        self.registers.set_zero(value == 0);
    }

    fn xor<S: Storage>(&mut self, s: S) {
        let value = self.registers.a ^ s.load(self);
        self.registers.a = value;
        self.registers.f = 0;
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

    // ADD HL, rr
    fn add_hl(&mut self, register: Register16) {
        let hl = self.registers.hl();
        let value = self.registers.load_16(register);
        let result = value.wrapping_add(hl);
        self.registers.store_16(Register16::HL, result);
    }

    // ADD SP, nn
    fn add_sp(&mut self) {
        // TODO: handle flags
        let sp = self.registers.sp;
        // Crazy trick to do signed addition.
        // u8 -> i8 -> i16 -> u16
        let value = self.load_byte_and_inc_pc() as i8 as i16 as u16;
        let result = value.wrapping_add(sp);
        self.registers.store_16(Register16::SP, result);
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
        let carry = self.registers.test_flag(CARRY_FLAG);
        self.registers.set_zero(value == 0);
        self.registers.set_carry(carry);
        storage.store(self, value);
    }

    fn inc_16(&mut self, register: Register16) {
        let value = self.registers.load_16(register.clone()).wrapping_add(1);
        let carry = self.registers.test_flag(CARRY_FLAG);
        self.registers.set_zero(value == 0);
        self.registers.set_carry(carry);
        self.registers.store_16(register, value);
    }

    fn dec<S: Storage>(&mut self, storage: S) {
        let value = storage.load(self).wrapping_sub(1);
        let carry = self.registers.test_flag(CARRY_FLAG);
        self.registers.set_zero(value == 0);
        self.registers.set_carry(carry);
        storage.store(self, value);
    }

    fn dec_16(&mut self, register: Register16) {
        let value = self.registers.load_16(register.clone()).wrapping_sub(1);
        let carry = self.registers.test_flag(CARRY_FLAG);
        self.registers.set_zero(value == 0);
        self.registers.set_carry(carry);
        self.registers.store_16(register, value);
    }

    fn jp(&mut self) {
        let address = self.load_word_and_inc_pc();
        self.registers.pc = address;
        self.cycles += 1;
    }

    fn jp_hl(&mut self) {
        let address = self.registers.hl();
        self.registers.pc = address;
        self.cycles += 1;
    }

    fn jp_if(&mut self, flag: u8) {
        let address = self.load_word_and_inc_pc();
        if self.registers.test_flag(flag) {
            self.registers.pc = address;
            self.cycles += 1;
        }
    }

    fn jp_unless(&mut self, flag: u8) {
        let address = self.load_word_and_inc_pc();
        if !self.registers.test_flag(flag) {
            self.registers.pc = address;
            self.cycles += 1;
        }
    }

    fn jr(&mut self) {
        let offset = self.load_byte_and_inc_pc() as i8;
        self.registers.pc = (self.registers.pc as i16 + offset as i16) as u16;
        self.cycles += 1;
    }

    fn jr_if(&mut self, flag: u8) {
        let offset = self.load_byte_and_inc_pc() as i8;
        if self.registers.test_flag(flag) {
            self.registers.pc = (self.registers.pc as i16 + offset as i16) as u16;
            self.cycles += 1;
        }
    }

    fn jr_unless(&mut self, flag: u8) {
        let offset = self.load_byte_and_inc_pc() as i8;
        if !self.registers.test_flag(flag) {
            self.registers.pc = (self.registers.pc as i16 + offset as i16) as u16;
            self.cycles += 1;
        }
    }

    fn call_op(&mut self, address: u16) {
        //let return_address = self.load_word_and_inc_pc();
        let return_address = self.registers.pc;
        self.push_word(return_address);
        self.registers.pc = address;
        self.cycles += 1;
    }

    fn call(&mut self) {
        let address = self.load_word_and_inc_pc();
        self.call_op(address);
    }

    fn call_if(&mut self, flag: u8) {
        let address = self.load_word_and_inc_pc();
        if self.registers.test_flag(flag) {
            self.call_op(address);
        }
    }

    fn call_unless(&mut self, flag: u8) {
        let address = self.load_word_and_inc_pc();
        if !self.registers.test_flag(flag) {
            self.call_op(address);
        }
    }

    fn ret(&mut self) {
        self.registers.pc = self.pop_word();
        self.cycles += 1;
    }

    fn ret_if(&mut self, flag: u8) {
        if self.registers.test_flag(flag) {
            self.ret();
        }
    }

    fn ret_unless(&mut self, flag: u8) {
        if !self.registers.test_flag(flag) {
            self.ret();
        }
    }

    fn reti(&mut self) {
        self.interrupt = false;
        self.ret();
    }

    fn ei(&mut self) {
        self.interrupt = true;
    }

    fn di(&mut self) {
        self.interrupt = false;
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

    fn rst(&mut self, address: u16) {
        let pc = self.registers.pc;
        self.push_word(pc);
        self.registers.pc = address;
        self.cycles += 1;
    }

    fn daa(&mut self) {
        println!("{:?}", self);
        panic!("DAA not implemented yet");
    }

    // CB Instructions

    fn swap<S: Storage>(&mut self, register: S) {
        let original = register.load(self);
        let value = (original >> 4) | (original << 4);
        self.registers.f = 0;
        self.registers.set_zero(value == 0);
        register.store(self, value);
    }

    fn bit<S: Storage>(&mut self, bit: u8, register: S) {
        let value = register.load(self) & (1 << bit);
        self.registers.set_zero(value == 0);
    }

    fn res<S: Storage>(&mut self, bit: u8, register: S) {
        let value = register.load(self);
        register.store(self, value & !(1 << bit));
    }

    fn set<S: Storage>(&mut self, bit: u8, register: S) {
        let value = register.load(self);
        register.store(self, value | (1 << bit));
    }
}
