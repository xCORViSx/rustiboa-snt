// REMINDER: Read AGENTS.md file before continuing development
//
// CPU Module - Sharp LR35902 (modified Z80)
//
// This module implements the Game Boy's CPU, which is based on the Z80 but with
// some differences. We handle all the registers (A, B, C, D, E, F, H, L, PC, SP),
// the flags register, instruction fetching, decoding, and execution.

mod registers;
mod instructions;
mod opcodes;

pub use registers::Registers;

/// This struct represents the Game Boy's CPU state including all registers,
/// timing information, and execution state like whether interrupts are enabled
pub struct Cpu {
    /// All CPU registers (A, B, C, D, E, F, H, L, PC, SP)
    pub registers: Registers,
    
    /// The interrupt master enable flag (IME) which controls if interrupts work
    pub ime: bool,
    
    /// Whether we're currently halted (waiting for an interrupt)
    pub halted: bool,
    
    /// Machine cycles (M-cycles) spent on last instruction - each is 4 clock cycles
    pub last_m_cycles: u8,
}

impl Cpu {
    /// This creates a new CPU with all registers initialized to their power-on state.
    /// At power on, the Game Boy's CPU starts with specific values in registers.
    pub fn new() -> Self {
        Cpu {
            registers: Registers::new(),
            ime: false,
            halted: false,
            last_m_cycles: 0,
        }
    }
    
    /// This method executes one instruction - it fetches the opcode from memory,
    /// decodes what instruction it is, executes it, and returns how many cycles it took.
    pub fn tick(&mut self, mmu: &mut crate::mmu::Mmu) -> u8 {
        // If we're halted, we just wait and don't execute anything
        if self.halted {
            return 1; // Return 1 M-cycle for waiting
        }
        
        // We fetch the next instruction byte from where PC points
        let opcode = mmu.read_byte(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        
        // We execute the instruction and get back how many cycles it took
        let cycles = self.execute(opcode, mmu);
        self.last_m_cycles = cycles;
        
        cycles
    }
    
    /// This executes a single instruction based on the opcode we fetched.
    /// Each opcode maps to a specific instruction the CPU can perform.
    fn execute(&mut self, opcode: u8, mmu: &mut crate::mmu::Mmu) -> u8 {
        // We use a match statement to dispatch to the correct instruction implementation
        // based on the opcode value. Each instruction returns the number of M-cycles it took.
        use instructions::*;
        
        match opcode {
            // 0x0X - Misc/control
            0x00 => nop(self),
            0x01 => ld_bc_u16(self, mmu),
            0x02 => ld_bc_a(self, mmu),
            0x03 => inc_bc(self),
            0x04 => inc_b(self),
            0x05 => dec_b(self),
            0x06 => ld_b_u8(self, mmu),
            0x07 => rlca(self),
            0x08 => ld_u16_sp(self, mmu),
            0x09 => add_hl_bc(self),
            0x0A => ld_a_bc(self, mmu),
            0x0B => dec_bc(self),
            0x0C => inc_c(self),
            0x0D => dec_c(self),
            0x0E => ld_c_u8(self, mmu),
            0x0F => rrca(self),
            
            // 0x1X
            0x10 => stop(self),
            0x11 => ld_de_u16(self, mmu),
            0x12 => ld_de_a(self, mmu),
            0x13 => inc_de(self),
            0x14 => inc_d(self),
            0x15 => dec_d(self),
            0x16 => ld_d_u8(self, mmu),
            0x17 => rla(self),
            0x18 => jr_i8(self, mmu),
            0x19 => add_hl_de(self),
            0x1A => ld_a_de(self, mmu),
            0x1B => dec_de(self),
            0x1C => inc_e(self),
            0x1D => dec_e(self),
            0x1E => ld_e_u8(self, mmu),
            0x1F => rra(self),
            
            // 0x2X
            0x20 => jr_nz_i8(self, mmu),
            0x21 => ld_hl_u16(self, mmu),
            0x22 => ld_hli_a(self, mmu),
            0x23 => inc_hl(self),
            0x24 => inc_h(self),
            0x25 => dec_h(self),
            0x26 => ld_h_u8(self, mmu),
            0x27 => daa(self),
            0x28 => jr_z_i8(self, mmu),
            0x29 => add_hl_hl(self),
            0x2A => ld_a_hli(self, mmu),
            0x2B => dec_hl(self),
            0x2C => inc_l(self),
            0x2D => dec_l(self),
            0x2E => ld_l_u8(self, mmu),
            0x2F => cpl(self),
            
            // 0x3X
            0x30 => jr_nc_i8(self, mmu),
            0x31 => ld_sp_u16(self, mmu),
            0x32 => ld_hld_a(self, mmu),
            0x33 => inc_sp(self),
            0x34 => inc_hl_mem(self, mmu),
            0x35 => dec_hl_mem(self, mmu),
            0x36 => ld_hl_u8(self, mmu),
            0x37 => scf(self),
            0x38 => jr_c_i8(self, mmu),
            0x39 => add_hl_sp(self),
            0x3A => ld_a_hld(self, mmu),
            0x3B => dec_sp(self),
            0x3C => inc_a(self),
            0x3D => dec_a(self),
            0x3E => ld_a_u8(self, mmu),
            0x3F => ccf(self),
            
            // 0x4X - LD r,r instructions (register to register loads)
            0x40 => ld_r_r(self, REG_B, REG_B),
            0x41 => ld_r_r(self, REG_B, REG_C),
            0x42 => ld_r_r(self, REG_B, REG_D),
            0x43 => ld_r_r(self, REG_B, REG_E),
            0x44 => ld_r_r(self, REG_B, REG_H),
            0x45 => ld_r_r(self, REG_B, REG_L),
            0x46 => ld_b_hl(self, mmu),
            0x47 => ld_r_r(self, REG_B, REG_A),
            0x48 => ld_r_r(self, REG_C, REG_B),
            0x49 => ld_r_r(self, REG_C, REG_C),
            0x4A => ld_r_r(self, REG_C, REG_D),
            0x4B => ld_r_r(self, REG_C, REG_E),
            0x4C => ld_r_r(self, REG_C, REG_H),
            0x4D => ld_r_r(self, REG_C, REG_L),
            0x4E => ld_c_hl(self, mmu),
            0x4F => ld_r_r(self, REG_C, REG_A),
            
            // 0x5X
            0x50 => ld_r_r(self, REG_D, REG_B),
            0x51 => ld_r_r(self, REG_D, REG_C),
            0x52 => ld_r_r(self, REG_D, REG_D),
            0x53 => ld_r_r(self, REG_D, REG_E),
            0x54 => ld_r_r(self, REG_D, REG_H),
            0x55 => ld_r_r(self, REG_D, REG_L),
            0x56 => ld_d_hl(self, mmu),
            0x57 => ld_r_r(self, REG_D, REG_A),
            0x58 => ld_r_r(self, REG_E, REG_B),
            0x59 => ld_r_r(self, REG_E, REG_C),
            0x5A => ld_r_r(self, REG_E, REG_D),
            0x5B => ld_r_r(self, REG_E, REG_E),
            0x5C => ld_r_r(self, REG_E, REG_H),
            0x5D => ld_r_r(self, REG_E, REG_L),
            0x5E => ld_e_hl(self, mmu),
            0x5F => ld_r_r(self, REG_E, REG_A),
            
            // 0x6X
            0x60 => ld_r_r(self, REG_H, REG_B),
            0x61 => ld_r_r(self, REG_H, REG_C),
            0x62 => ld_r_r(self, REG_H, REG_D),
            0x63 => ld_r_r(self, REG_H, REG_E),
            0x64 => ld_r_r(self, REG_H, REG_H),
            0x65 => ld_r_r(self, REG_H, REG_L),
            0x66 => ld_h_hl(self, mmu),
            0x67 => ld_r_r(self, REG_H, REG_A),
            0x68 => ld_r_r(self, REG_L, REG_B),
            0x69 => ld_r_r(self, REG_L, REG_C),
            0x6A => ld_r_r(self, REG_L, REG_D),
            0x6B => ld_r_r(self, REG_L, REG_E),
            0x6C => ld_r_r(self, REG_L, REG_H),
            0x6D => ld_r_r(self, REG_L, REG_L),
            0x6E => ld_l_hl(self, mmu),
            0x6F => ld_r_r(self, REG_L, REG_A),
            
            // 0x7X
            0x70 => ld_hl_b(self, mmu),
            0x71 => ld_hl_c(self, mmu),
            0x72 => ld_hl_d(self, mmu),
            0x73 => ld_hl_e(self, mmu),
            0x74 => ld_hl_h(self, mmu),
            0x75 => ld_hl_l(self, mmu),
            0x76 => halt(self),
            0x77 => ld_hl_a(self, mmu),
            0x78 => ld_r_r(self, REG_A, REG_B),
            0x79 => ld_r_r(self, REG_A, REG_C),
            0x7A => ld_r_r(self, REG_A, REG_D),
            0x7B => ld_r_r(self, REG_A, REG_E),
            0x7C => ld_r_r(self, REG_A, REG_H),
            0x7D => ld_r_r(self, REG_A, REG_L),
            0x7E => ld_a_hl(self, mmu),
            0x7F => ld_r_r(self, REG_A, REG_A),
            
            // 0x8X - ADD/ADC instructions
            0x80 => add_a_r(self, REG_B),
            0x81 => add_a_r(self, REG_C),
            0x82 => add_a_r(self, REG_D),
            0x83 => add_a_r(self, REG_E),
            0x84 => add_a_r(self, REG_H),
            0x85 => add_a_r(self, REG_L),
            0x86 => add_a_hl(self, mmu),
            0x87 => add_a_r(self, REG_A),
            0x88 => adc_a_r(self, REG_B),
            0x89 => adc_a_r(self, REG_C),
            0x8A => adc_a_r(self, REG_D),
            0x8B => adc_a_r(self, REG_E),
            0x8C => adc_a_r(self, REG_H),
            0x8D => adc_a_r(self, REG_L),
            0x8E => adc_a_hl(self, mmu),
            0x8F => adc_a_r(self, REG_A),
            
            // 0x9X - SUB/SBC instructions
            0x90 => sub_a_r(self, REG_B),
            0x91 => sub_a_r(self, REG_C),
            0x92 => sub_a_r(self, REG_D),
            0x93 => sub_a_r(self, REG_E),
            0x94 => sub_a_r(self, REG_H),
            0x95 => sub_a_r(self, REG_L),
            0x96 => sub_a_hl(self, mmu),
            0x97 => sub_a_r(self, REG_A),
            0x98 => sbc_a_r(self, REG_B),
            0x99 => sbc_a_r(self, REG_C),
            0x9A => sbc_a_r(self, REG_D),
            0x9B => sbc_a_r(self, REG_E),
            0x9C => sbc_a_r(self, REG_H),
            0x9D => sbc_a_r(self, REG_L),
            0x9E => sbc_a_hl(self, mmu),
            0x9F => sbc_a_r(self, REG_A),
            
            // 0xAX - AND/XOR instructions
            0xA0 => and_a_r(self, REG_B),
            0xA1 => and_a_r(self, REG_C),
            0xA2 => and_a_r(self, REG_D),
            0xA3 => and_a_r(self, REG_E),
            0xA4 => and_a_r(self, REG_H),
            0xA5 => and_a_r(self, REG_L),
            0xA6 => and_a_hl(self, mmu),
            0xA7 => and_a_r(self, REG_A),
            0xA8 => xor_a_r(self, REG_B),
            0xA9 => xor_a_r(self, REG_C),
            0xAA => xor_a_r(self, REG_D),
            0xAB => xor_a_r(self, REG_E),
            0xAC => xor_a_r(self, REG_H),
            0xAD => xor_a_r(self, REG_L),
            0xAE => xor_a_hl(self, mmu),
            0xAF => xor_a_r(self, REG_A),
            
            // 0xBX - OR/CP instructions
            0xB0 => or_a_r(self, REG_B),
            0xB1 => or_a_r(self, REG_C),
            0xB2 => or_a_r(self, REG_D),
            0xB3 => or_a_r(self, REG_E),
            0xB4 => or_a_r(self, REG_H),
            0xB5 => or_a_r(self, REG_L),
            0xB6 => or_a_hl(self, mmu),
            0xB7 => or_a_r(self, REG_A),
            0xB8 => cp_a_r(self, REG_B),
            0xB9 => cp_a_r(self, REG_C),
            0xBA => cp_a_r(self, REG_D),
            0xBB => cp_a_r(self, REG_E),
            0xBC => cp_a_r(self, REG_H),
            0xBD => cp_a_r(self, REG_L),
            0xBE => cp_a_hl(self, mmu),
            0xBF => cp_a_r(self, REG_A),
            
            // 0xCX - Control flow and stack operations
            0xC0 => ret_nz(self, mmu),
            0xC1 => pop_bc(self, mmu),
            0xC2 => jp_nz_u16(self, mmu),
            0xC3 => jp_u16(self, mmu),
            0xC4 => call_nz_u16(self, mmu),
            0xC5 => push_bc(self, mmu),
            0xC6 => add_a_u8(self, mmu),
            0xC7 => rst_00(self, mmu),
            0xC8 => ret_z(self, mmu),
            0xC9 => ret(self, mmu),
            0xCA => jp_z_u16(self, mmu),
            0xCB => execute_cb(self, mmu), // CB-prefixed instructions
            0xCC => call_z_u16(self, mmu),
            0xCD => call_u16(self, mmu),
            0xCE => adc_a_u8(self, mmu),
            0xCF => rst_08(self, mmu),
            
            // 0xDX
            0xD0 => ret_nc(self, mmu),
            0xD1 => pop_de(self, mmu),
            0xD2 => jp_nc_u16(self, mmu),
            0xD3 => illegal_opcode(opcode),
            0xD4 => call_nc_u16(self, mmu),
            0xD5 => push_de(self, mmu),
            0xD6 => sub_a_u8(self, mmu),
            0xD7 => rst_10(self, mmu),
            0xD8 => ret_c(self, mmu),
            0xD9 => reti(self, mmu),
            0xDA => jp_c_u16(self, mmu),
            0xDB => illegal_opcode(opcode),
            0xDC => call_c_u16(self, mmu),
            0xDD => illegal_opcode(opcode),
            0xDE => sbc_a_u8(self, mmu),
            0xDF => rst_18(self, mmu),
            
            // 0xEX
            0xE0 => ldh_u8_a(self, mmu),
            0xE1 => pop_hl(self, mmu),
            0xE2 => ldh_c_a(self, mmu),
            0xE3 => illegal_opcode(opcode),
            0xE4 => illegal_opcode(opcode),
            0xE5 => push_hl(self, mmu),
            0xE6 => and_a_u8(self, mmu),
            0xE7 => rst_20(self, mmu),
            0xE8 => add_sp_i8(self, mmu),
            0xE9 => jp_hl(self),
            0xEA => ld_u16_a(self, mmu),
            0xEB => illegal_opcode(opcode),
            0xEC => illegal_opcode(opcode),
            0xED => illegal_opcode(opcode),
            0xEE => xor_a_u8(self, mmu),
            0xEF => rst_28(self, mmu),
            
            // 0xFX
            0xF0 => ldh_a_u8(self, mmu),
            0xF1 => pop_af(self, mmu),
            0xF2 => ldh_a_c(self, mmu),
            0xF3 => di(self),
            0xF4 => illegal_opcode(opcode),
            0xF5 => push_af(self, mmu),
            0xF6 => or_a_u8(self, mmu),
            0xF7 => rst_30(self, mmu),
            0xF8 => ld_hl_sp_i8(self, mmu),
            0xF9 => ld_sp_hl(self),
            0xFA => ld_a_u16(self, mmu),
            0xFB => ei(self),
            0xFC => illegal_opcode(opcode),
            0xFD => illegal_opcode(opcode),
            0xFE => cp_a_u8(self, mmu),
            0xFF => rst_38(self, mmu),
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}
