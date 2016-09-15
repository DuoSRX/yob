extern crate yob;

use yob::cpu::Cpu;
use yob::registers::*;

#[cfg(test)]
fn reset() -> Cpu { Cpu::new() }

fn step(cpu: &mut Cpu, instr: u8, steps: u16) {
    let pc = cpu.registers.pc;
    cpu.store_byte(0, instr);
    cpu.step();
    let steps_taken = cpu.registers.pc - pc;
    assert_eq!(steps_taken, steps);
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