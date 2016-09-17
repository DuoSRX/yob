extern crate yob;

use yob::cpu::Cpu;
use yob::registers::*;

#[cfg(test)]
fn reset() -> Cpu { Cpu::new() }

fn step(cpu: &mut Cpu, instr: u8, steps: u16) {
    let pc = cpu.registers.pc;
    cpu.store_byte(pc, instr);
    cpu.step();
    let steps_taken = cpu.registers.pc - pc;
    assert_eq!(steps_taken, steps);
}

#[test]
fn flags_ops() {
    let mut cpu = reset();
    assert!(!cpu.registers.test_flag(CARRY_FLAG));
    assert!(!cpu.registers.test_flag(ZERO_FLAG));

    step(&mut cpu, 0x37, 1); // SCF
    println!("{:?}", cpu);
    assert!(cpu.registers.test_flag(CARRY_FLAG));

    step(&mut cpu, 0x3F, 1); // CCF
    println!("{:?}", cpu);
    assert!(!cpu.registers.test_flag(CARRY_FLAG));
}

#[test]
fn inc_a() {
    let mut cpu = reset();
    cpu.registers.a = 0x7;
    step(&mut cpu, 0x3C, 1);
    assert_eq!(cpu.registers.a, 0x8);

    let mut cpu = reset();
    cpu.registers.a = 0xFF;
    step(&mut cpu, 0x3C, 1);
    assert_eq!(cpu.registers.a, 0);
    assert_eq!(cpu.registers.f, ZERO_FLAG);
}

#[test]
fn dec_a() {
    let mut cpu = reset();
    cpu.registers.a = 0x7;
    step(&mut cpu, 0x3D, 1);
    assert_eq!(cpu.registers.a, 0x6);

    let mut cpu = reset();
    cpu.registers.a = 0x1;
    step(&mut cpu, 0x3D, 1);
    assert_eq!(cpu.registers.a, 0x0);
    assert_eq!(cpu.registers.f, ZERO_FLAG);
}

#[test]
fn ld_b_immediate() {
    let mut cpu = reset();
    cpu.store_byte(0x1, 0x42);
    step(&mut cpu, 0x06, 2);
    assert_eq!(cpu.registers.b, 0x42);
}

#[test]
fn ld_ba() {
    let mut cpu = reset();
    cpu.registers.a = 0x42;
    step(&mut cpu, 0x47, 1);
    assert_eq!(cpu.registers.b, cpu.registers.a);
}

#[test]
fn ld_b_hl() {
    let mut cpu = reset();
    cpu.registers.h = 0x15;
    cpu.registers.l = 0x20;
    cpu.store_byte((0x15 << 8) | 0x20 , 0x42);
    step(&mut cpu, 0x46, 1);
    assert_eq!(cpu.registers.b, 0x42);
}

#[test]
fn ld_bc_a() {
    let mut cpu = reset();
    cpu.registers.a = 0x42;
    cpu.registers.b = 0x15;
    cpu.registers.c = 0x20;
    step(&mut cpu, 0x02, 1);
    let byte = cpu.load_byte((0x15 << 8) | 0x20);
    assert_eq!(byte, 0x42);
}

#[test]
fn add_b() {
    let mut cpu = reset();
    cpu.registers.a = 0x2;
    cpu.registers.b = 0x2;
    cpu.registers.set_carry(true);
    step(&mut cpu, 0x80, 1);
    assert_eq!(cpu.registers.a, 0x4);
    assert_eq!(cpu.registers.carry(), 0);

    let mut cpu = reset();
    cpu.registers.a = 0xFF;
    cpu.registers.b = 0x02;
    step(&mut cpu, 0x80, 1);
    assert_eq!(cpu.registers.a, 0x1);
    assert_eq!(cpu.registers.carry(), 1);
}

#[test]
fn adc_b() {
    let mut cpu = reset();
    cpu.registers.a = 0x2;
    cpu.registers.b = 0x3;
    cpu.registers.set_carry(true);
    step(&mut cpu, 0x88, 1);
    assert_eq!(cpu.registers.a, 0x6);
    assert_eq!(cpu.registers.carry(), 0);

    let mut cpu = reset();
    cpu.registers.a = 0xFF;
    cpu.registers.b = 0x01;
    step(&mut cpu, 0x88, 1);
    assert_eq!(cpu.registers.a, 0x0);
    assert_eq!(cpu.registers.carry(), 1);
}

#[test]
fn sub_b() {
    let mut cpu = reset();
    cpu.registers.a = 0x3;
    cpu.registers.b = 0x1;
    cpu.registers.set_carry(true);
    step(&mut cpu, 0x90, 1);
    assert_eq!(cpu.registers.a, 0x2);
    assert_eq!(cpu.registers.carry(), 0);

    let mut cpu = reset();
    cpu.registers.a = 0x01;
    cpu.registers.b = 0x02;
    step(&mut cpu, 0x90, 1);
    assert_eq!(cpu.registers.a, 0xFF);
    assert_eq!(cpu.registers.carry(), 1);
}

#[test]
fn sbc_b() {
    let mut cpu = reset();
    cpu.registers.a = 0x3;
    cpu.registers.b = 0x1;
    cpu.registers.set_carry(true);
    step(&mut cpu, 0x98, 1);
    assert_eq!(cpu.registers.a, 0x1);
    assert_eq!(cpu.registers.carry(), 0);

    let mut cpu = reset();
    cpu.registers.a = 0x01;
    cpu.registers.b = 0x02;
    step(&mut cpu, 0x98, 1);
    assert_eq!(cpu.registers.a, 0xFF);
    assert_eq!(cpu.registers.carry(), 1);
}

#[test]
fn and_b() {
    let mut cpu = reset();
    cpu.registers.a = 0b1010_0001;
    cpu.registers.b = 0b0001_0001;
    step(&mut cpu, 0xA0, 1);
    assert_eq!(cpu.registers.a, 0b0000_0001);
}

#[test]
fn xor_b() {
    let mut cpu = reset();
    cpu.registers.a = 0b1111_1111;
    cpu.registers.b = 0b1110_1111;
    step(&mut cpu, 0xA8, 1);
    assert_eq!(cpu.registers.a, 0b0001_0000);
}

#[test]
fn or_b() {
    let mut cpu = reset();
    cpu.registers.a = 0b1010_0001;
    cpu.registers.b = 0b0001_0001;
    step(&mut cpu, 0xB0, 1);
    assert_eq!(cpu.registers.a, 0b1011_0001);
}


#[test]
fn cp_b() {
    let mut cpu = reset();
    cpu.registers.a = 0x2;
    cpu.registers.b = 0x1;
    step(&mut cpu, 0xB8, 1);
    assert!(!cpu.registers.test_flag(ZERO_FLAG));

    let mut cpu = reset();
    cpu.registers.a = 0x2;
    cpu.registers.b = 0x2;
    step(&mut cpu, 0xB8, 1);
    assert!(cpu.registers.test_flag(ZERO_FLAG));
    assert_eq!(cpu.registers.a, 0x2); // Make sure CP doesn't write to A
}

#[test]
fn push_hl() {
    let mut cpu = reset();
    cpu.registers.h = 0x12;
    cpu.registers.l = 0x34;
    step(&mut cpu, 0xE5, 1);
    let byte = cpu.load_word(0xFFFC);
    assert_eq!(byte, 0x1234);
    assert_eq!(cpu.registers.sp, 0xFFFC);
}

#[test]
fn pop_hl() {
    let mut cpu = reset();
    cpu.store_word(0xFFFC, 0x1234);
    cpu.registers.sp = 0xFFFC;
    step(&mut cpu, 0xE1, 1);
    assert_eq!(cpu.registers.hl(), 0x1234);
    assert_eq!(cpu.registers.sp, 0xFFFC + 2);
}

#[test]
fn ret() {
    let mut cpu = reset();
    cpu.store_word(0xFFFC, 0x0);
    cpu.registers.sp = 0xFFFC;
    step(&mut cpu, 0xC9, 0);
    assert_eq!(cpu.registers.pc, 0);
}

#[test]
fn rlca() {
    let mut cpu = reset();
    cpu.registers.a = 0b1000_0010;
    step(&mut cpu, 0x07, 1);
    assert_eq!(cpu.registers.a, 0b0000_0101);
    assert_eq!(cpu.registers.carry(), 1);

    let mut cpu = reset();
    cpu.registers.a = 0b0100_0010;
    step(&mut cpu, 0x07, 1);
    assert_eq!(cpu.registers.a, 0b1000_0100);
    assert_eq!(cpu.registers.carry(), 0);
}

#[test]
fn rrca() {
    let mut cpu = reset();
    cpu.registers.a = 0b0010_0001;
    step(&mut cpu, 0x0F, 1);
    assert_eq!(cpu.registers.a, 0b1001_0000);
    assert_eq!(cpu.registers.carry(), 1);

    let mut cpu = reset();
    cpu.registers.a = 0b0010_0010;
    step(&mut cpu, 0x0F, 1);
    assert_eq!(cpu.registers.a, 0b0001_0001);
    assert_eq!(cpu.registers.carry(), 0);
}

#[test]
fn rla() {
    let mut cpu = reset();
    cpu.registers.a = 0b0111_0110;
    cpu.registers.set_carry(true);
    step(&mut cpu, 0x17, 1);
    assert_eq!(cpu.registers.a, 0b1110_1101);
    assert_eq!(cpu.registers.carry(), 0);

    let mut cpu = reset();
    cpu.registers.a = 0b1111_0110;
    step(&mut cpu, 0x17, 1);
    assert_eq!(cpu.registers.a, 0b1110_1100);
    assert_eq!(cpu.registers.carry(), 1);
}

#[test]
fn rra() {
    let mut cpu = reset();
    cpu.registers.a = 0b1110_0001;
    step(&mut cpu, 0x1F, 1);
    assert_eq!(cpu.registers.a, 0b0111_0000);
    assert_eq!(cpu.registers.carry(), 1);

    let mut cpu = reset();
    cpu.registers.a = 0b1110_0000;
    cpu.registers.set_carry(true);
    step(&mut cpu, 0x1F, 1);
    assert_eq!(cpu.registers.a, 0b1111_0000);
    assert_eq!(cpu.registers.carry(), 0);
}
