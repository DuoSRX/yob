use std;
use std::fmt;

pub const ZERO_FLAG:       u8 = 0b1000_0000;
pub const SUB_FLAG:        u8 = 0b0100_0000;
pub const HALF_CARRY_FLAG: u8 = 0b0010_0000;
pub const CARRY_FLAG:      u8 = 0b0001_0000;

pub enum Register8 {
    A, B, C, D, E, F, H, L
}

pub enum Register16 {
    AF, BC, DE, HL, SP
}

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,

    pub pc: u16,
    pub sp: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0xFFFE,
        }
    }

    fn set_flag_if(&mut self, flag: u8, condition: bool) {
        if condition {
            self.f |= flag;
        } else {
            self.f &= !flag;
        }
    }

    pub fn set_zero(&mut self, condition: bool) {
        self.set_flag_if(ZERO_FLAG, condition);
    }

    pub fn set_carry(&mut self, condition: bool) {
        self.set_flag_if(CARRY_FLAG, condition);
    }

    pub fn carry(&self) -> u8 {
        if self.f & CARRY_FLAG != 0 { 1 } else { 0 }
    }

    pub fn test_flag(&self, flag: u8) -> bool {
        if self.f & flag != 0 { true } else { false }
    }

    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }

    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    // Not sure how I feel about these being here
    pub fn load_16(&self, register: Register16) -> u16 {
        match register {
            Register16::AF => self.af(),
            Register16::BC => self.bc(),
            Register16::DE => self.de(),
            Register16::HL => self.hl(),
            Register16::SP => self.sp,
        }
    }

    pub fn store_16(&mut self, register: Register16, value: u16) {
        let lo = (value & 0xFF) as u8;
        let hi = ((value >> 8) & 0xFF) as u8;

        match register {
            Register16::AF => { self.a = hi; self.f = lo },
            Register16::BC => { self.b = hi; self.c = lo },
            Register16::DE => { self.d = hi; self.e = lo },
            Register16::HL => { self.h = hi; self.l = lo },
            Register16::SP => { self.sp = value },
        }
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "A:{:02x} B:{:02x} C:{:02x} D:{:02x} E:{:02x} \
             H:{:02x} L:{:02x} F:{:04b} \
             PC:{:04x} SP:{:04x}",
            self.a, self.b, self.c, self.d, self.e,
            self.h, self.l, self.f,
            self.pc, self.sp
        ))
    }
}
