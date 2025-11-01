// REMINDER: Read AGENTS.md file before continuing development
//
// CPU Instructions - Instruction implementations
//
// This file contains the actual implementation of each CPU instruction.
// Each instruction manipulates registers, memory, or flags according to the
// Game Boy's CPU specification. Instructions are grouped by type.

use super::Cpu;
use crate::mmu::Mmu;

// Register identifiers for ld_r_r and similar operations
pub const REG_A: u8 = 0;
pub const REG_B: u8 = 1;
pub const REG_C: u8 = 2;
pub const REG_D: u8 = 3;
pub const REG_E: u8 = 4;
pub const REG_H: u8 = 5;
pub const REG_L: u8 = 6;

/// This helper reads an 8-bit immediate value from PC and advances PC
fn read_u8(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let value = mmu.read_byte(cpu.registers.pc);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);
    value
}

/// This helper reads a 16-bit immediate value from PC and advances PC
fn read_u16(cpu: &mut Cpu, mmu: &Mmu) -> u16 {
    let value = mmu.read_word(cpu.registers.pc);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(2);
    value
}

/// This helper reads an 8-bit signed immediate value from PC and advances PC
fn read_i8(cpu: &mut Cpu, mmu: &Mmu) -> i8 {
    read_u8(cpu, mmu) as i8
}

/// This helper gets a register value by ID FOR CB INSTRUCTIONS
/// CB instructions use different encoding: 0=B, 1=C, 2=D, 3=E, 4=H, 5=L, 6=(HL), 7=A
fn get_reg_cb(cpu: &Cpu, reg: u8) -> u8 {
    match reg {
        0 => cpu.registers.b,
        1 => cpu.registers.c,
        2 => cpu.registers.d,
        3 => cpu.registers.e,
        4 => cpu.registers.h,
        5 => cpu.registers.l,
        7 => cpu.registers.a,
        _ => 0,
    }
}

/// This helper sets a register value by ID FOR CB INSTRUCTIONS
/// CB instructions use different encoding: 0=B, 1=C, 2=D, 3=E, 4=H, 5=L, 6=(HL), 7=A
fn set_reg_cb(cpu: &mut Cpu, reg: u8, value: u8) {
    match reg {
        0 => cpu.registers.b = value,
        1 => cpu.registers.c = value,
        2 => cpu.registers.d = value,
        3 => cpu.registers.e = value,
        4 => cpu.registers.h = value,
        5 => cpu.registers.l = value,
        7 => cpu.registers.a = value,
        _ => {}
    }
}

/// This helper gets a register value by ID (for non-CB instructions)
fn get_reg(cpu: &Cpu, reg: u8) -> u8 {
    match reg {
        REG_A => cpu.registers.a,
        REG_B => cpu.registers.b,
        REG_C => cpu.registers.c,
        REG_D => cpu.registers.d,
        REG_E => cpu.registers.e,
        REG_H => cpu.registers.h,
        REG_L => cpu.registers.l,
        _ => 0,
    }
}

/// This helper sets a register value by ID
fn set_reg(cpu: &mut Cpu, reg: u8, value: u8) {
    match reg {
        REG_A => cpu.registers.a = value,
        REG_B => cpu.registers.b = value,
        REG_C => cpu.registers.c = value,
        REG_D => cpu.registers.d = value,
        REG_E => cpu.registers.e = value,
        REG_H => cpu.registers.h = value,
        REG_L => cpu.registers.l = value,
        _ => {}
    }
}

// ===== Misc/Control Instructions =====

/// NOP - No Operation - does nothing, takes 1 M-cycle
pub fn nop(_cpu: &Cpu) -> u8 {
    1
}

/// STOP - Enters low power mode until button press (2 bytes: 0x10 0x00)
pub fn stop(cpu: &mut Cpu) -> u8 {
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1); // Skip next byte
    // TODO: Implement actual STOP behavior
    1
}

/// HALT - Enters halt mode until interrupt occurs
pub fn halt(cpu: &mut Cpu) -> u8 {
    cpu.halted = true;
    1
}

/// DI - Disable Interrupts
pub fn di(cpu: &mut Cpu) -> u8 {
    cpu.ime = false;
    1
}

/// EI - Enable Interrupts (takes effect after next instruction)
pub fn ei(cpu: &mut Cpu) -> u8 {
    cpu.ime = true; // TODO: Should be delayed by one instruction
    1
}

/// This handles illegal/undefined opcodes by panicking
pub fn illegal_opcode(opcode: u8) -> u8 {
    panic!("Illegal opcode: 0x{:02X}", opcode);
}

// ===== 8-bit Load Instructions =====

/// LD r,r - Load register to register
pub fn ld_r_r(cpu: &mut Cpu, dest: u8, src: u8) -> u8 {
    let value = get_reg(cpu, src);
    set_reg(cpu, dest, value);
    1
}

/// LD r,u8 - Load immediate 8-bit value into register
pub fn ld_b_u8(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    cpu.registers.b = read_u8(cpu, mmu);
    2
}

pub fn ld_c_u8(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    cpu.registers.c = read_u8(cpu, mmu);
    2
}

pub fn ld_d_u8(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    cpu.registers.d = read_u8(cpu, mmu);
    2
}

pub fn ld_e_u8(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    cpu.registers.e = read_u8(cpu, mmu);
    2
}

pub fn ld_h_u8(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    cpu.registers.h = read_u8(cpu, mmu);
    2
}

pub fn ld_l_u8(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    cpu.registers.l = read_u8(cpu, mmu);
    2
}

pub fn ld_a_u8(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    cpu.registers.a = read_u8(cpu, mmu);
    2
}

/// LD r,(HL) - Load value from memory address HL into register
pub fn ld_b_hl(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    cpu.registers.b = mmu.read_byte(cpu.registers.hl());
    2
}

pub fn ld_c_hl(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    cpu.registers.c = mmu.read_byte(cpu.registers.hl());
    2
}

pub fn ld_d_hl(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    cpu.registers.d = mmu.read_byte(cpu.registers.hl());
    2
}

pub fn ld_e_hl(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    cpu.registers.e = mmu.read_byte(cpu.registers.hl());
    2
}

pub fn ld_h_hl(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    cpu.registers.h = mmu.read_byte(cpu.registers.hl());
    2
}

pub fn ld_l_hl(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    cpu.registers.l = mmu.read_byte(cpu.registers.hl());
    2
}

pub fn ld_a_hl(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    cpu.registers.a = mmu.read_byte(cpu.registers.hl());
    2
}

/// LD (HL),r - Load register into memory address HL
pub fn ld_hl_b(cpu: &Cpu, mmu: &mut Mmu) -> u8 {
    mmu.write_byte(cpu.registers.hl(), cpu.registers.b);
    2
}

pub fn ld_hl_c(cpu: &Cpu, mmu: &mut Mmu) -> u8 {
    mmu.write_byte(cpu.registers.hl(), cpu.registers.c);
    2
}

pub fn ld_hl_d(cpu: &Cpu, mmu: &mut Mmu) -> u8 {
    mmu.write_byte(cpu.registers.hl(), cpu.registers.d);
    2
}

pub fn ld_hl_e(cpu: &Cpu, mmu: &mut Mmu) -> u8 {
    mmu.write_byte(cpu.registers.hl(), cpu.registers.e);
    2
}

pub fn ld_hl_h(cpu: &Cpu, mmu: &mut Mmu) -> u8 {
    mmu.write_byte(cpu.registers.hl(), cpu.registers.h);
    2
}

pub fn ld_hl_l(cpu: &Cpu, mmu: &mut Mmu) -> u8 {
    mmu.write_byte(cpu.registers.hl(), cpu.registers.l);
    2
}

pub fn ld_hl_a(cpu: &Cpu, mmu: &mut Mmu) -> u8 {
    mmu.write_byte(cpu.registers.hl(), cpu.registers.a);
    2
}

/// LD (HL),u8 - Load immediate value into memory address HL
pub fn ld_hl_u8(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 {
    let value = read_u8(cpu, mmu);
    mmu.write_byte(cpu.registers.hl(), value);
    3
}

/// LD A,(BC) - Load value from memory address BC into A
pub fn ld_a_bc(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    cpu.registers.a = mmu.read_byte(cpu.registers.bc());
    2
}

/// LD A,(DE) - Load value from memory address DE into A
pub fn ld_a_de(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    cpu.registers.a = mmu.read_byte(cpu.registers.de());
    2
}

/// LD (BC),A - Load A into memory address BC
pub fn ld_bc_a(cpu: &Cpu, mmu: &mut Mmu) -> u8 {
    mmu.write_byte(cpu.registers.bc(), cpu.registers.a);
    2
}

/// LD (DE),A - Load A into memory address DE
pub fn ld_de_a(cpu: &Cpu, mmu: &mut Mmu) -> u8 {
    mmu.write_byte(cpu.registers.de(), cpu.registers.a);
    2
}

/// LD A,(HL+) / LD A,(HLI) - Load from HL into A, increment HL
pub fn ld_a_hli(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    cpu.registers.a = mmu.read_byte(cpu.registers.hl());
    cpu.registers.set_hl(cpu.registers.hl().wrapping_add(1));
    2
}

/// LD (HL+),A / LD (HLI),A - Load A into HL, increment HL
pub fn ld_hli_a(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 {
    mmu.write_byte(cpu.registers.hl(), cpu.registers.a);
    cpu.registers.set_hl(cpu.registers.hl().wrapping_add(1));
    2
}

/// LD A,(HL-) / LD A,(HLD) - Load from HL into A, decrement HL
pub fn ld_a_hld(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    cpu.registers.a = mmu.read_byte(cpu.registers.hl());
    cpu.registers.set_hl(cpu.registers.hl().wrapping_sub(1));
    2
}

/// LD (HL-),A / LD (HLD),A - Load A into HL, decrement HL
pub fn ld_hld_a(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 {
    mmu.write_byte(cpu.registers.hl(), cpu.registers.a);
    cpu.registers.set_hl(cpu.registers.hl().wrapping_sub(1));
    2
}

/// LD A,(u16) - Load value from immediate 16-bit address into A
pub fn ld_a_u16(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 {
    let address = read_u16(cpu, mmu);
    cpu.registers.a = mmu.read_byte(address);
    4
}

/// LD (u16),A - Load A into immediate 16-bit address
pub fn ld_u16_a(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 {
    let address = read_u16(cpu, mmu);
    mmu.write_byte(address, cpu.registers.a);
    4
}

/// LDH (u8),A / LD ($FF00+u8),A - Load A into high memory (0xFF00 + u8)
pub fn ldh_u8_a(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 {
    let offset = read_u8(cpu, mmu);
    mmu.write_byte(0xFF00 + offset as u16, cpu.registers.a);
    3
}

/// LDH A,(u8) / LD A,($FF00+u8) - Load from high memory into A
pub fn ldh_a_u8(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 {
    let offset = read_u8(cpu, mmu);
    cpu.registers.a = mmu.read_byte(0xFF00 + offset as u16);
    3
}

/// LDH (C),A / LD ($FF00+C),A - Load A into high memory (0xFF00 + C)
pub fn ldh_c_a(cpu: &Cpu, mmu: &mut Mmu) -> u8 {
    mmu.write_byte(0xFF00 + cpu.registers.c as u16, cpu.registers.a);
    2
}

/// LDH A,(C) / LD A,($FF00+C) - Load from high memory (0xFF00 + C) into A
pub fn ldh_a_c(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    cpu.registers.a = mmu.read_byte(0xFF00 + cpu.registers.c as u16);
    2
}

// ===== 16-bit Load Instructions =====

/// LD BC,u16 - Load 16-bit immediate into BC
pub fn ld_bc_u16(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let value = read_u16(cpu, mmu);
    cpu.registers.set_bc(value);
    3
}

/// LD DE,u16 - Load 16-bit immediate into DE
pub fn ld_de_u16(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let value = read_u16(cpu, mmu);
    cpu.registers.set_de(value);
    3
}

/// LD HL,u16 - Load 16-bit immediate into HL
pub fn ld_hl_u16(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let value = read_u16(cpu, mmu);
    cpu.registers.set_hl(value);
    3
}

/// LD SP,u16 - Load 16-bit immediate into SP
pub fn ld_sp_u16(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    cpu.registers.sp = read_u16(cpu, mmu);
    3
}

/// LD (u16),SP - Load SP into memory at immediate 16-bit address
pub fn ld_u16_sp(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 {
    let address = read_u16(cpu, mmu);
    mmu.write_word(address, cpu.registers.sp);
    5
}

/// LD SP,HL - Load HL into SP
pub fn ld_sp_hl(cpu: &mut Cpu) -> u8 {
    cpu.registers.sp = cpu.registers.hl();
    2
}

/// LD HL,SP+i8 - Load SP + signed 8-bit immediate into HL
pub fn ld_hl_sp_i8(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let offset = read_i8(cpu, mmu);
    let sp = cpu.registers.sp;
    let result = sp.wrapping_add(offset as u16);
    
    // We set flags based on the addition using the UNSIGNED byte value for flag calculation
    // The offset is signed for the actual addition, but flags check overflow in lower byte
    let offset_u8 = offset as u8;
    cpu.registers.set_flag_z(false);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(((sp as u8) & 0x0F) + (offset_u8 & 0x0F) > 0x0F);
    cpu.registers.set_flag_c(((sp as u8) as u16) + (offset_u8 as u16) > 0xFF);
    
    cpu.registers.set_hl(result);
    3
}

// ===== 8-bit Arithmetic Instructions =====

/// INC r - Increment register
pub fn inc_b(cpu: &mut Cpu) -> u8 {
    cpu.registers.b = inc_u8(cpu, cpu.registers.b);
    1
}

pub fn inc_c(cpu: &mut Cpu) -> u8 {
    cpu.registers.c = inc_u8(cpu, cpu.registers.c);
    1
}

pub fn inc_d(cpu: &mut Cpu) -> u8 {
    cpu.registers.d = inc_u8(cpu, cpu.registers.d);
    1
}

pub fn inc_e(cpu: &mut Cpu) -> u8 {
    cpu.registers.e = inc_u8(cpu, cpu.registers.e);
    1
}

pub fn inc_h(cpu: &mut Cpu) -> u8 {
    cpu.registers.h = inc_u8(cpu, cpu.registers.h);
    1
}

pub fn inc_l(cpu: &mut Cpu) -> u8 {
    cpu.registers.l = inc_u8(cpu, cpu.registers.l);
    1
}

pub fn inc_a(cpu: &mut Cpu) -> u8 {
    cpu.registers.a = inc_u8(cpu, cpu.registers.a);
    1
}

/// INC (HL) - Increment value at memory address HL
pub fn inc_hl_mem(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 {
    let address = cpu.registers.hl();
    let value = mmu.read_byte(address);
    let result = inc_u8(cpu, value);
    mmu.write_byte(address, result);
    3
}

/// This helper implements 8-bit increment with proper flag setting
fn inc_u8(cpu: &mut Cpu, value: u8) -> u8 {
    let result = value.wrapping_add(1);
    cpu.registers.set_flag_z(result == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h((value & 0x0F) == 0x0F);
    result
}

/// DEC r - Decrement register
pub fn dec_b(cpu: &mut Cpu) -> u8 {
    cpu.registers.b = dec_u8(cpu, cpu.registers.b);
    1
}

pub fn dec_c(cpu: &mut Cpu) -> u8 {
    cpu.registers.c = dec_u8(cpu, cpu.registers.c);
    1
}

pub fn dec_d(cpu: &mut Cpu) -> u8 {
    cpu.registers.d = dec_u8(cpu, cpu.registers.d);
    1
}

pub fn dec_e(cpu: &mut Cpu) -> u8 {
    cpu.registers.e = dec_u8(cpu, cpu.registers.e);
    1
}

pub fn dec_h(cpu: &mut Cpu) -> u8 {
    cpu.registers.h = dec_u8(cpu, cpu.registers.h);
    1
}

pub fn dec_l(cpu: &mut Cpu) -> u8 {
    cpu.registers.l = dec_u8(cpu, cpu.registers.l);
    1
}

pub fn dec_a(cpu: &mut Cpu) -> u8 {
    cpu.registers.a = dec_u8(cpu, cpu.registers.a);
    1
}

/// DEC (HL) - Decrement value at memory address HL
pub fn dec_hl_mem(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 {
    let address = cpu.registers.hl();
    let value = mmu.read_byte(address);
    let result = dec_u8(cpu, value);
    mmu.write_byte(address, result);
    3
}

/// This helper implements 8-bit decrement with proper flag setting
fn dec_u8(cpu: &mut Cpu, value: u8) -> u8 {
    let result = value.wrapping_sub(1);
    cpu.registers.set_flag_z(result == 0);
    cpu.registers.set_flag_n(true);
    cpu.registers.set_flag_h((value & 0x0F) == 0);
    result
}

/// ADD A,r - Add register to A
pub fn add_a_r(cpu: &mut Cpu, reg: u8) -> u8 {
    let value = get_reg(cpu, reg);
    add_a(cpu, value);
    1
}

/// ADD A,(HL) - Add value at HL to A
pub fn add_a_hl(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let value = mmu.read_byte(cpu.registers.hl());
    add_a(cpu, value);
    2
}

/// ADD A,u8 - Add immediate to A
pub fn add_a_u8(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let value = read_u8(cpu, mmu);
    add_a(cpu, value);
    2
}

/// This helper implements ADD operation with proper flag setting
fn add_a(cpu: &mut Cpu, value: u8) {
    let a = cpu.registers.a;
    let result = a.wrapping_add(value);
    
    cpu.registers.set_flag_z(result == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h((a & 0x0F) + (value & 0x0F) > 0x0F);
    cpu.registers.set_flag_c(a as u16 + value as u16 > 0xFF);
    
    cpu.registers.a = result;
}

/// ADC A,r - Add register + carry flag to A
pub fn adc_a_r(cpu: &mut Cpu, reg: u8) -> u8 {
    let value = get_reg(cpu, reg);
    adc_a(cpu, value);
    1
}

/// ADC A,(HL) - Add value at HL + carry to A
pub fn adc_a_hl(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let value = mmu.read_byte(cpu.registers.hl());
    adc_a(cpu, value);
    2
}

/// ADC A,u8 - Add immediate + carry to A
pub fn adc_a_u8(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let value = read_u8(cpu, mmu);
    adc_a(cpu, value);
    2
}

/// This helper implements ADC operation
fn adc_a(cpu: &mut Cpu, value: u8) {
    let a = cpu.registers.a;
    let carry = if cpu.registers.flag_c() { 1 } else { 0 };
    let result = a.wrapping_add(value).wrapping_add(carry);
    
    cpu.registers.set_flag_z(result == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h((a & 0x0F) + (value & 0x0F) + carry > 0x0F);
    cpu.registers.set_flag_c(a as u16 + value as u16 + carry as u16 > 0xFF);
    
    cpu.registers.a = result;
}

/// SUB A,r - Subtract register from A
pub fn sub_a_r(cpu: &mut Cpu, reg: u8) -> u8 {
    let value = get_reg(cpu, reg);
    sub_a(cpu, value);
    1
}

/// SUB A,(HL) - Subtract value at HL from A
pub fn sub_a_hl(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let value = mmu.read_byte(cpu.registers.hl());
    sub_a(cpu, value);
    2
}

/// SUB A,u8 - Subtract immediate from A
pub fn sub_a_u8(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let value = read_u8(cpu, mmu);
    sub_a(cpu, value);
    2
}

/// This helper implements SUB operation
fn sub_a(cpu: &mut Cpu, value: u8) {
    let a = cpu.registers.a;
    let result = a.wrapping_sub(value);
    
    cpu.registers.set_flag_z(result == 0);
    cpu.registers.set_flag_n(true);
    cpu.registers.set_flag_h((a & 0x0F) < (value & 0x0F));
    cpu.registers.set_flag_c(a < value);
    
    cpu.registers.a = result;
}

/// SBC A,r - Subtract register + carry from A
pub fn sbc_a_r(cpu: &mut Cpu, reg: u8) -> u8 {
    let value = get_reg(cpu, reg);
    sbc_a(cpu, value);
    1
}

/// SBC A,(HL) - Subtract value at HL + carry from A
pub fn sbc_a_hl(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let value = mmu.read_byte(cpu.registers.hl());
    sbc_a(cpu, value);
    2
}

/// SBC A,u8 - Subtract immediate + carry from A
pub fn sbc_a_u8(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let value = read_u8(cpu, mmu);
    sbc_a(cpu, value);
    2
}

/// This helper implements SBC operation
fn sbc_a(cpu: &mut Cpu, value: u8) {
    let a = cpu.registers.a;
    let carry = if cpu.registers.flag_c() { 1 } else { 0 };
    let result = a.wrapping_sub(value).wrapping_sub(carry);
    
    cpu.registers.set_flag_z(result == 0);
    cpu.registers.set_flag_n(true);
    cpu.registers.set_flag_h((a & 0x0F) < (value & 0x0F) + carry);
    cpu.registers.set_flag_c((a as u16) < (value as u16 + carry as u16));
    
    cpu.registers.a = result;
}

/// AND A,r - Bitwise AND register with A
pub fn and_a_r(cpu: &mut Cpu, reg: u8) -> u8 {
    let value = get_reg(cpu, reg);
    and_a(cpu, value);
    1
}

/// AND A,(HL) - Bitwise AND value at HL with A
pub fn and_a_hl(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let value = mmu.read_byte(cpu.registers.hl());
    and_a(cpu, value);
    2
}

/// AND A,u8 - Bitwise AND immediate with A
pub fn and_a_u8(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let value = read_u8(cpu, mmu);
    and_a(cpu, value);
    2
}

/// This helper implements AND operation
fn and_a(cpu: &mut Cpu, value: u8) {
    cpu.registers.a &= value;
    cpu.registers.set_flag_z(cpu.registers.a == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(true);
    cpu.registers.set_flag_c(false);
}

/// XOR A,r - Bitwise XOR register with A
pub fn xor_a_r(cpu: &mut Cpu, reg: u8) -> u8 {
    let value = get_reg(cpu, reg);
    xor_a(cpu, value);
    1
}

/// XOR A,(HL) - Bitwise XOR value at HL with A
pub fn xor_a_hl(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let value = mmu.read_byte(cpu.registers.hl());
    xor_a(cpu, value);
    2
}

/// XOR A,u8 - Bitwise XOR immediate with A
pub fn xor_a_u8(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let value = read_u8(cpu, mmu);
    xor_a(cpu, value);
    2
}

/// This helper implements XOR operation
fn xor_a(cpu: &mut Cpu, value: u8) {
    cpu.registers.a ^= value;
    cpu.registers.set_flag_z(cpu.registers.a == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_c(false);
}

/// OR A,r - Bitwise OR register with A
pub fn or_a_r(cpu: &mut Cpu, reg: u8) -> u8 {
    let value = get_reg(cpu, reg);
    or_a(cpu, value);
    1
}

/// OR A,(HL) - Bitwise OR value at HL with A
pub fn or_a_hl(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let value = mmu.read_byte(cpu.registers.hl());
    or_a(cpu, value);
    2
}

/// OR A,u8 - Bitwise OR immediate with A
pub fn or_a_u8(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let value = read_u8(cpu, mmu);
    or_a(cpu, value);
    2
}

/// This helper implements OR operation
fn or_a(cpu: &mut Cpu, value: u8) {
    cpu.registers.a |= value;
    cpu.registers.set_flag_z(cpu.registers.a == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_c(false);
}

/// CP A,r - Compare register with A (subtract but don't store result)
pub fn cp_a_r(cpu: &mut Cpu, reg: u8) -> u8 {
    let value = get_reg(cpu, reg);
    cp_a(cpu, value);
    1
}

/// CP A,(HL) - Compare value at HL with A
pub fn cp_a_hl(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let value = mmu.read_byte(cpu.registers.hl());
    cp_a(cpu, value);
    2
}

/// CP A,u8 - Compare immediate with A
pub fn cp_a_u8(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let value = read_u8(cpu, mmu);
    cp_a(cpu, value);
    2
}

/// This helper implements CP operation (compare, sets flags like SUB but doesn't store)
fn cp_a(cpu: &mut Cpu, value: u8) {
    let a = cpu.registers.a;
    cpu.registers.set_flag_z(a == value);
    cpu.registers.set_flag_n(true);
    cpu.registers.set_flag_h((a & 0x0F) < (value & 0x0F));
    cpu.registers.set_flag_c(a < value);
}

// ===== 16-bit Arithmetic Instructions =====

/// INC rr - Increment 16-bit register
pub fn inc_bc(cpu: &mut Cpu) -> u8 {
    cpu.registers.set_bc(cpu.registers.bc().wrapping_add(1));
    2
}

pub fn inc_de(cpu: &mut Cpu) -> u8 {
    cpu.registers.set_de(cpu.registers.de().wrapping_add(1));
    2
}

pub fn inc_hl(cpu: &mut Cpu) -> u8 {
    cpu.registers.set_hl(cpu.registers.hl().wrapping_add(1));
    2
}

pub fn inc_sp(cpu: &mut Cpu) -> u8 {
    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
    2
}

/// DEC rr - Decrement 16-bit register
pub fn dec_bc(cpu: &mut Cpu) -> u8 {
    cpu.registers.set_bc(cpu.registers.bc().wrapping_sub(1));
    2
}

pub fn dec_de(cpu: &mut Cpu) -> u8 {
    cpu.registers.set_de(cpu.registers.de().wrapping_sub(1));
    2
}

pub fn dec_hl(cpu: &mut Cpu) -> u8 {
    cpu.registers.set_hl(cpu.registers.hl().wrapping_sub(1));
    2
}

pub fn dec_sp(cpu: &mut Cpu) -> u8 {
    cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
    2
}

/// ADD HL,rr - Add 16-bit register to HL
pub fn add_hl_bc(cpu: &mut Cpu) -> u8 {
    add_hl(cpu, cpu.registers.bc());
    2
}

pub fn add_hl_de(cpu: &mut Cpu) -> u8 {
    add_hl(cpu, cpu.registers.de());
    2
}

pub fn add_hl_hl(cpu: &mut Cpu) -> u8 {
    let hl = cpu.registers.hl();
    add_hl(cpu, hl);
    2
}

pub fn add_hl_sp(cpu: &mut Cpu) -> u8 {
    add_hl(cpu, cpu.registers.sp);
    2
}

/// This helper implements 16-bit ADD to HL
fn add_hl(cpu: &mut Cpu, value: u16) {
    let hl = cpu.registers.hl();
    let result = hl.wrapping_add(value);
    
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h((hl & 0x0FFF) + (value & 0x0FFF) > 0x0FFF);
    cpu.registers.set_flag_c(hl as u32 + value as u32 > 0xFFFF);
    
    cpu.registers.set_hl(result);
}

/// ADD SP,i8 - Add signed 8-bit immediate to SP
pub fn add_sp_i8(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let offset = read_i8(cpu, mmu);
    let sp = cpu.registers.sp;
    
    // We set flags based on the addition using the UNSIGNED byte value for flag calculation
    // The offset is signed for the actual addition, but flags check overflow in lower byte
    let offset_u8 = offset as u8;
    cpu.registers.set_flag_z(false);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(((sp as u8) & 0x0F) + (offset_u8 & 0x0F) > 0x0F);
    cpu.registers.set_flag_c(((sp as u8) as u16) + (offset_u8 as u16) > 0xFF);
    
    cpu.registers.sp = sp.wrapping_add(offset as u16);
    4
}

// ===== Rotate and Shift Instructions =====

/// RLCA - Rotate A left, old bit 7 to carry
pub fn rlca(cpu: &mut Cpu) -> u8 {
    let a = cpu.registers.a;
    let carry = (a & 0x80) != 0;
    cpu.registers.a = (a << 1) | (if carry { 1 } else { 0 });
    
    cpu.registers.set_flag_z(false);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_c(carry);
    1
}

/// RRCA - Rotate A right, old bit 0 to carry
pub fn rrca(cpu: &mut Cpu) -> u8 {
    let a = cpu.registers.a;
    let carry = (a & 0x01) != 0;
    cpu.registers.a = (a >> 1) | (if carry { 0x80 } else { 0 });
    
    cpu.registers.set_flag_z(false);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_c(carry);
    1
}

/// RLA - Rotate A left through carry
pub fn rla(cpu: &mut Cpu) -> u8 {
    let a = cpu.registers.a;
    let old_carry = if cpu.registers.flag_c() { 1 } else { 0 };
    let new_carry = (a & 0x80) != 0;
    cpu.registers.a = (a << 1) | old_carry;
    
    cpu.registers.set_flag_z(false);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_c(new_carry);
    1
}

/// RRA - Rotate A right through carry
pub fn rra(cpu: &mut Cpu) -> u8 {
    let a = cpu.registers.a;
    let old_carry = if cpu.registers.flag_c() { 0x80 } else { 0 };
    let new_carry = (a & 0x01) != 0;
    cpu.registers.a = (a >> 1) | old_carry;
    
    cpu.registers.set_flag_z(false);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_c(new_carry);
    1
}

/// DAA - Decimal adjust A for BCD arithmetic
/// This instruction adjusts register A after BCD (Binary Coded Decimal) addition/subtraction
/// to produce a valid BCD result. Each 4-bit nibble represents one decimal digit (0-9).
/// DAA - Decimal adjust A for BCD arithmetic
/// This instruction adjusts register A after BCD (Binary Coded Decimal) addition/subtraction
/// to produce a valid BCD result. Each 4-bit nibble represents one decimal digit (0-9).
pub fn daa(cpu: &mut Cpu) -> u8 {
    // We start with the current value of A from the previous operation
    let mut a = cpu.registers.a;
    
    if !cpu.registers.flag_n() {
        // After addition: adjust if (half-)carry occurred or if result is out of bounds
        if cpu.registers.flag_c() || a > 0x99 {
            a = a.wrapping_add(0x60);
            cpu.registers.set_flag_c(true);
        }
        if cpu.registers.flag_h() || (a & 0x0F) > 0x09 {
            a = a.wrapping_add(0x6);
        }
    } else {
        // After subtraction: only adjust if (half-)carry occurred
        if cpu.registers.flag_c() {
            a = a.wrapping_sub(0x60);
        }
        if cpu.registers.flag_h() {
            a = a.wrapping_sub(0x6);
        }
    }
    
    // These flags are always updated
    cpu.registers.set_flag_z(a == 0);
    cpu.registers.set_flag_h(false);
    // Note: carry flag is set above if needed, never cleared
    cpu.registers.a = a;
    1
}

/// CPL - Complement A (flip all bits)
pub fn cpl(cpu: &mut Cpu) -> u8 {
    cpu.registers.a = !cpu.registers.a;
    cpu.registers.set_flag_n(true);
    cpu.registers.set_flag_h(true);
    1
}

/// SCF - Set carry flag
pub fn scf(cpu: &mut Cpu) -> u8 {
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_c(true);
    1
}

/// CCF - Complement carry flag
pub fn ccf(cpu: &mut Cpu) -> u8 {
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_c(!cpu.registers.flag_c());
    1
}

// ===== Jump Instructions =====

/// JP u16 - Unconditional jump to immediate address
pub fn jp_u16(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    cpu.registers.pc = read_u16(cpu, mmu);
    4
}

/// JP cc,u16 - Conditional jump to immediate address
pub fn jp_nz_u16(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let address = read_u16(cpu, mmu);
    if !cpu.registers.flag_z() {
        cpu.registers.pc = address;
        4
    } else {
        3
    }
}

pub fn jp_z_u16(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let address = read_u16(cpu, mmu);
    if cpu.registers.flag_z() {
        cpu.registers.pc = address;
        4
    } else {
        3
    }
}

pub fn jp_nc_u16(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let address = read_u16(cpu, mmu);
    if !cpu.registers.flag_c() {
        cpu.registers.pc = address;
        4
    } else {
        3
    }
}

pub fn jp_c_u16(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let address = read_u16(cpu, mmu);
    if cpu.registers.flag_c() {
        cpu.registers.pc = address;
        4
    } else {
        3
    }
}

/// JP (HL) - Jump to address in HL
pub fn jp_hl(cpu: &mut Cpu) -> u8 {
    cpu.registers.pc = cpu.registers.hl();
    1
}

/// JR i8 - Relative jump by signed offset
pub fn jr_i8(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let offset = read_i8(cpu, mmu);
    cpu.registers.pc = cpu.registers.pc.wrapping_add(offset as u16);
    3
}

/// JR cc,i8 - Conditional relative jump
pub fn jr_nz_i8(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let offset = read_i8(cpu, mmu);
    if !cpu.registers.flag_z() {
        cpu.registers.pc = cpu.registers.pc.wrapping_add(offset as u16);
        3
    } else {
        2
    }
}

pub fn jr_z_i8(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let offset = read_i8(cpu, mmu);
    if cpu.registers.flag_z() {
        cpu.registers.pc = cpu.registers.pc.wrapping_add(offset as u16);
        3
    } else {
        2
    }
}

pub fn jr_nc_i8(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let offset = read_i8(cpu, mmu);
    if !cpu.registers.flag_c() {
        cpu.registers.pc = cpu.registers.pc.wrapping_add(offset as u16);
        3
    } else {
        2
    }
}

pub fn jr_c_i8(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let offset = read_i8(cpu, mmu);
    if cpu.registers.flag_c() {
        cpu.registers.pc = cpu.registers.pc.wrapping_add(offset as u16);
        3
    } else {
        2
    }
}

// ===== Call and Return Instructions =====

/// CALL u16 - Unconditional call to address
pub fn call_u16(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 {
    let address = read_u16(cpu, mmu);
    push_u16(cpu, mmu, cpu.registers.pc);
    cpu.registers.pc = address;
    6
}

/// CALL cc,u16 - Conditional call
pub fn call_nz_u16(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 {
    let address = read_u16(cpu, mmu);
    if !cpu.registers.flag_z() {
        push_u16(cpu, mmu, cpu.registers.pc);
        cpu.registers.pc = address;
        6
    } else {
        3
    }
}

pub fn call_z_u16(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 {
    let address = read_u16(cpu, mmu);
    if cpu.registers.flag_z() {
        push_u16(cpu, mmu, cpu.registers.pc);
        cpu.registers.pc = address;
        6
    } else {
        3
    }
}

pub fn call_nc_u16(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 {
    let address = read_u16(cpu, mmu);
    if !cpu.registers.flag_c() {
        push_u16(cpu, mmu, cpu.registers.pc);
        cpu.registers.pc = address;
        6
    } else {
        3
    }
}

pub fn call_c_u16(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 {
    let address = read_u16(cpu, mmu);
    if cpu.registers.flag_c() {
        push_u16(cpu, mmu, cpu.registers.pc);
        cpu.registers.pc = address;
        6
    } else {
        3
    }
}

/// RET - Unconditional return from call
pub fn ret(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    cpu.registers.pc = pop_u16(cpu, mmu);
    4
}

/// RET cc - Conditional return
pub fn ret_nz(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    if !cpu.registers.flag_z() {
        cpu.registers.pc = pop_u16(cpu, mmu);
        5
    } else {
        2
    }
}

pub fn ret_z(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    if cpu.registers.flag_z() {
        cpu.registers.pc = pop_u16(cpu, mmu);
        5
    } else {
        2
    }
}

pub fn ret_nc(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    if !cpu.registers.flag_c() {
        cpu.registers.pc = pop_u16(cpu, mmu);
        5
    } else {
        2
    }
}

pub fn ret_c(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    if cpu.registers.flag_c() {
        cpu.registers.pc = pop_u16(cpu, mmu);
        5
    } else {
        2
    }
}

/// RETI - Return and enable interrupts
pub fn reti(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    cpu.registers.pc = pop_u16(cpu, mmu);
    cpu.ime = true;
    4
}

/// RST n - Call to fixed address
pub fn rst_00(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 { rst(cpu, mmu, 0x00); 4 }
pub fn rst_08(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 { rst(cpu, mmu, 0x08); 4 }
pub fn rst_10(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 { rst(cpu, mmu, 0x10); 4 }
pub fn rst_18(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 { rst(cpu, mmu, 0x18); 4 }
pub fn rst_20(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 { rst(cpu, mmu, 0x20); 4 }
pub fn rst_28(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 { rst(cpu, mmu, 0x28); 4 }
pub fn rst_30(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 { rst(cpu, mmu, 0x30); 4 }
pub fn rst_38(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 { rst(cpu, mmu, 0x38); 4 }

/// This helper implements RST operation (restart/call to fixed address)
fn rst(cpu: &mut Cpu, mmu: &mut Mmu, address: u8) {
    push_u16(cpu, mmu, cpu.registers.pc);
    cpu.registers.pc = address as u16;
}

// ===== Stack Instructions =====

/// PUSH rr - Push 16-bit register onto stack
pub fn push_bc(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 {
    push_u16(cpu, mmu, cpu.registers.bc());
    4
}

pub fn push_de(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 {
    push_u16(cpu, mmu, cpu.registers.de());
    4
}

pub fn push_hl(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 {
    push_u16(cpu, mmu, cpu.registers.hl());
    4
}

pub fn push_af(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 {
    push_u16(cpu, mmu, cpu.registers.af());
    4
}

/// POP rr - Pop 16-bit value from stack into register
pub fn pop_bc(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let value = pop_u16(cpu, mmu);
    cpu.registers.set_bc(value);
    3
}

pub fn pop_de(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let value = pop_u16(cpu, mmu);
    cpu.registers.set_de(value);
    3
}

pub fn pop_hl(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let value = pop_u16(cpu, mmu);
    cpu.registers.set_hl(value);
    3
}

pub fn pop_af(cpu: &mut Cpu, mmu: &Mmu) -> u8 {
    let value = pop_u16(cpu, mmu);
    cpu.registers.set_af(value);
    3
}

/// This helper pushes 16-bit value onto stack
fn push_u16(cpu: &mut Cpu, mmu: &mut Mmu, value: u16) {
    cpu.registers.sp = cpu.registers.sp.wrapping_sub(2);
    mmu.write_word(cpu.registers.sp, value);
}

/// This helper pops 16-bit value from stack
fn pop_u16(cpu: &mut Cpu, mmu: &Mmu) -> u16 {
    let value = mmu.read_word(cpu.registers.sp);
    cpu.registers.sp = cpu.registers.sp.wrapping_add(2);
    value
}

// ===== CB-Prefixed Instructions =====

/// This handles all CB-prefixed instructions (rotates, shifts, bit operations)
pub fn execute_cb(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 {
    let opcode = read_u8(cpu, mmu);
    
    // We extract the operation type from bits 6-7, register from bits 0-2
    let op = (opcode >> 6) & 0x03;
    let bit = (opcode >> 3) & 0x07;
    let reg = opcode & 0x07;
    
    match op {
        0 => execute_cb_rot_shift(cpu, mmu, bit, reg),
        1 => execute_cb_bit(cpu, mmu, bit, reg),
        2 => execute_cb_res(cpu, mmu, bit, reg),
        3 => execute_cb_set(cpu, mmu, bit, reg),
        _ => unreachable!(),
    }
}

/// This handles CB rotate and shift operations (RLC, RRC, RL, RR, SLA, SRA, SWAP, SRL)
fn execute_cb_rot_shift(cpu: &mut Cpu, mmu: &mut Mmu, op: u8, reg: u8) -> u8 {
    let (value, cycles) = if reg == 6 {
        // (HL) operations take 4 cycles
        (mmu.read_byte(cpu.registers.hl()), 4)
    } else {
        // Register operations take 2 cycles - use CB register encoding
        (get_reg_cb(cpu, reg), 2)
    };
    
    let result = match op {
        0 => rlc(cpu, value),   // RLC - Rotate left
        1 => rrc(cpu, value),   // RRC - Rotate right
        2 => rl(cpu, value),    // RL - Rotate left through carry
        3 => rr(cpu, value),    // RR - Rotate right through carry
        4 => sla(cpu, value),   // SLA - Shift left arithmetic
        5 => sra(cpu, value),   // SRA - Shift right arithmetic (preserve sign)
        6 => swap(cpu, value),  // SWAP - Swap nibbles
        7 => srl(cpu, value),   // SRL - Shift right logical
        _ => unreachable!(),
    };
    
    if reg == 6 {
        mmu.write_byte(cpu.registers.hl(), result);
    } else {
        // Use CB register encoding
        set_reg_cb(cpu, reg, result);
    }
    
    cycles
}

/// RLC - Rotate left, old bit 7 to carry and bit 0
fn rlc(cpu: &mut Cpu, value: u8) -> u8 {
    let carry = (value & 0x80) != 0;
    let result = (value << 1) | (if carry { 1 } else { 0 });
    cpu.registers.set_flag_z(result == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_c(carry);
    result
}

/// RRC - Rotate right, old bit 0 to carry and bit 7
fn rrc(cpu: &mut Cpu, value: u8) -> u8 {
    let carry = (value & 0x01) != 0;
    let result = (value >> 1) | (if carry { 0x80 } else { 0 });
    cpu.registers.set_flag_z(result == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_c(carry);
    result
}

/// RL - Rotate left through carry
fn rl(cpu: &mut Cpu, value: u8) -> u8 {
    let old_carry = if cpu.registers.flag_c() { 1 } else { 0 };
    let new_carry = (value & 0x80) != 0;
    let result = (value << 1) | old_carry;
    cpu.registers.set_flag_z(result == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_c(new_carry);
    result
}

/// RR - Rotate right through carry
fn rr(cpu: &mut Cpu, value: u8) -> u8 {
    let old_carry = if cpu.registers.flag_c() { 0x80 } else { 0 };
    let new_carry = (value & 0x01) != 0;
    let result = (value >> 1) | old_carry;
    cpu.registers.set_flag_z(result == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_c(new_carry);
    result
}

/// SLA - Shift left arithmetic (logical shift, bit 0 = 0)
fn sla(cpu: &mut Cpu, value: u8) -> u8 {
    let carry = (value & 0x80) != 0;
    let result = value << 1;
    cpu.registers.set_flag_z(result == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_c(carry);
    result
}

/// SRA - Shift right arithmetic (preserve sign bit)
fn sra(cpu: &mut Cpu, value: u8) -> u8 {
    let carry = (value & 0x01) != 0;
    let result = (value >> 1) | (value & 0x80);
    cpu.registers.set_flag_z(result == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_c(carry);
    result
}

/// SWAP - Swap upper and lower nibbles
fn swap(cpu: &mut Cpu, value: u8) -> u8 {
    let result = value.rotate_left(4);
    cpu.registers.set_flag_z(result == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_c(false);
    result
}

/// SRL - Shift right logical (bit 7 = 0)
fn srl(cpu: &mut Cpu, value: u8) -> u8 {
    let carry = (value & 0x01) != 0;
    let result = value >> 1;
    cpu.registers.set_flag_z(result == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(false);
    cpu.registers.set_flag_c(carry);
    result
}

/// BIT b,r - Test bit in register
fn execute_cb_bit(cpu: &mut Cpu, mmu: &Mmu, bit: u8, reg: u8) -> u8 {
    let value = if reg == 6 {
        mmu.read_byte(cpu.registers.hl())
    } else {
        get_reg_cb(cpu, reg)  // Use CB register encoding
    };
    
    let result = value & (1 << bit);
    cpu.registers.set_flag_z(result == 0);
    cpu.registers.set_flag_n(false);
    cpu.registers.set_flag_h(true);
    
    if reg == 6 { 3 } else { 2 }
}

/// RES b,r - Reset (clear) bit in register
fn execute_cb_res(cpu: &mut Cpu, mmu: &mut Mmu, bit: u8, reg: u8) -> u8 {
    let mask = !(1 << bit);
    
    if reg == 6 {
        let address = cpu.registers.hl();
        let value = mmu.read_byte(address);
        mmu.write_byte(address, value & mask);
        4
    } else {
        let value = get_reg_cb(cpu, reg);  // Use CB register encoding
        set_reg_cb(cpu, reg, value & mask);  // Use CB register encoding
        2
    }
}

/// SET b,r - Set bit in register
fn execute_cb_set(cpu: &mut Cpu, mmu: &mut Mmu, bit: u8, reg: u8) -> u8 {
    let mask = 1 << bit;
    
    if reg == 6 {
        let address = cpu.registers.hl();
        let value = mmu.read_byte(address);
        mmu.write_byte(address, value | mask);
        4
    } else {
        let value = get_reg_cb(cpu, reg);  // Use CB register encoding
        set_reg_cb(cpu, reg, value | mask);  // Use CB register encoding
        2
    }
}
