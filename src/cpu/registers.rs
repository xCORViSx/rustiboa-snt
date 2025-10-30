// REMINDER: Read AGENTS.md file before continuing development
//
// CPU Registers - Sharp LR35902 Register Set
//
// This file defines all the CPU registers. The Game Boy has 8-bit registers
// (A, B, C, D, E, F, H, L) that can be paired into 16-bit registers (AF, BC, DE, HL),
// plus 16-bit registers for the program counter (PC) and stack pointer (SP).

/// This holds all the CPU registers and provides methods to access them individually
/// or as 16-bit pairs which is useful for memory addressing and 16-bit operations
pub struct Registers {
    /// Accumulator register - most arithmetic happens here
    pub a: u8,
    
    /// Flags register - stores result flags from operations
    /// Bit 7: Zero flag (Z) - set when result is zero
    /// Bit 6: Subtraction flag (N) - set when last operation was subtraction
    /// Bit 5: Half-carry flag (H) - set when lower nibble overflowed
    /// Bit 4: Carry flag (C) - set when result overflowed/underflowed
    /// Bits 0-3: Always zero
    pub f: u8,
    
    /// General purpose registers
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    
    /// Program counter - points to the next instruction to execute
    pub pc: u16,
    
    /// Stack pointer - points to the top of the stack in memory
    pub sp: u16,
}

// Flag bit positions in the F register
const FLAG_ZERO: u8 = 0b1000_0000;
const FLAG_SUBTRACT: u8 = 0b0100_0000;
const FLAG_HALF_CARRY: u8 = 0b0010_0000;
const FLAG_CARRY: u8 = 0b0001_0000;

impl Registers {
    /// This creates new registers with the power-on state that the Game Boy
    /// boot ROM expects after it finishes running
    pub fn new() -> Self {
        Registers {
            a: 0x01,  // After boot ROM, A = 0x01
            f: 0xB0,  // Flags: Z=1, N=0, H=1, C=1
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            pc: 0x0100,  // Execution starts at 0x0100 after boot ROM
            sp: 0xFFFE,  // Stack starts at top of high RAM
        }
    }
    
    // These methods get/set 16-bit register pairs which we need often
    
    /// This gets the AF register pair (A in high byte, F in low byte)
    pub fn af(&self) -> u16 {
        (self.a as u16) << 8 | (self.f as u16)
    }
    
    /// This sets the AF register pair from a 16-bit value
    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0xF0) as u8;  // Lower 4 bits always zero
    }
    
    /// This gets the BC register pair
    pub fn bc(&self) -> u16 {
        (self.b as u16) << 8 | (self.c as u16)
    }
    
    /// This sets the BC register pair
    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }
    
    /// This gets the DE register pair
    pub fn de(&self) -> u16 {
        (self.d as u16) << 8 | (self.e as u16)
    }
    
    /// This sets the DE register pair
    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }
    
    /// This gets the HL register pair (often used for memory addressing)
    pub fn hl(&self) -> u16 {
        (self.h as u16) << 8 | (self.l as u16)
    }
    
    /// This sets the HL register pair
    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
    
    // These methods check individual flags in the F register
    
    /// This checks if the Zero flag is set (result was zero)
    pub fn flag_z(&self) -> bool {
        self.f & FLAG_ZERO != 0
    }
    
    /// This sets or clears the Zero flag
    pub fn set_flag_z(&mut self, value: bool) {
        if value {
            self.f |= FLAG_ZERO;
        } else {
            self.f &= !FLAG_ZERO;
        }
    }
    
    /// This checks if the Subtract flag is set (last op was subtraction)
    pub fn flag_n(&self) -> bool {
        self.f & FLAG_SUBTRACT != 0
    }
    
    /// This sets or clears the Subtract flag
    pub fn set_flag_n(&mut self, value: bool) {
        if value {
            self.f |= FLAG_SUBTRACT;
        } else {
            self.f &= !FLAG_SUBTRACT;
        }
    }
    
    /// This checks if the Half-carry flag is set (lower nibble overflowed)
    pub fn flag_h(&self) -> bool {
        self.f & FLAG_HALF_CARRY != 0
    }
    
    /// This sets or clears the Half-carry flag
    pub fn set_flag_h(&mut self, value: bool) {
        if value {
            self.f |= FLAG_HALF_CARRY;
        } else {
            self.f &= !FLAG_HALF_CARRY;
        }
    }
    
    /// This checks if the Carry flag is set (result overflowed/underflowed)
    pub fn flag_c(&self) -> bool {
        self.f & FLAG_CARRY != 0
    }
    
    /// This sets or clears the Carry flag
    pub fn set_flag_c(&mut self, value: bool) {
        if value {
            self.f |= FLAG_CARRY;
        } else {
            self.f &= !FLAG_CARRY;
        }
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}
