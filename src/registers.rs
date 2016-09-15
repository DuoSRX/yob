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
            sp: 0
        }
    }

    pub fn set_zero_flag(&mut self, condition: bool) {
        if condition {
            self.f |= ZERO_FLAG;
        } else {
            self.f &= !ZERO_FLAG;
        }
    }

    fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
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
