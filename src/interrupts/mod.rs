// REMINDER: Read AGENTS.md file before continuing development
//
// Interrupt System
//
// This module handles Game Boy interrupts. The GB has 5 interrupt sources:
// VBlank, LCD STAT, Timer, Serial, and Joypad. Each can be individually enabled
// via the IE register and their pending state is tracked in the IF register.
// When an interrupt fires, the CPU jumps to a specific handler address.

use crate::cpu::Cpu;
use crate::mmu::Mmu;

/// Interrupt bit positions in IE and IF registers
pub const INT_VBLANK: u8 = 0x01;  // Bit 0: VBlank interrupt
pub const INT_LCD_STAT: u8 = 0x02; // Bit 1: LCD STAT interrupt
pub const INT_TIMER: u8 = 0x04;    // Bit 2: Timer interrupt
pub const INT_SERIAL: u8 = 0x08;   // Bit 3: Serial interrupt
pub const INT_JOYPAD: u8 = 0x10;   // Bit 4: Joypad interrupt

/// Interrupt handler addresses in memory
const INT_VBLANK_ADDR: u16 = 0x0040;
const INT_LCD_STAT_ADDR: u16 = 0x0048;
const INT_TIMER_ADDR: u16 = 0x0050;
const INT_SERIAL_ADDR: u16 = 0x0058;
const INT_JOYPAD_ADDR: u16 = 0x0060;

/// This checks if any enabled interrupts are pending and services the highest priority one.
/// Returns the number of cycles taken (20 if interrupt serviced, 0 otherwise).
/// Priority order: VBlank > LCD STAT > Timer > Serial > Joypad
pub fn handle_interrupts(cpu: &mut Cpu, mmu: &mut Mmu) -> u8 {
    // We read the enabled interrupts (IE) and pending interrupts (IF)
    let ie = mmu.read_byte(0xFFFF); // Interrupt Enable register
    let if_reg = mmu.read_byte(0xFF0F); // Interrupt Flag register
    
    // We find which interrupts are both enabled and pending
    let triggered = ie & if_reg;
    
    // If the CPU is halted, any triggered interrupt wakes it up (even if IME is off)
    if cpu.halted && triggered != 0 {
        cpu.halted = false;
    }
    
    // We can only service interrupts if IME (Interrupt Master Enable) is set
    if !cpu.ime {
        return 0;
    }
    
    // If no interrupts are triggered, we return immediately
    if triggered == 0 {
        return 0;
    }
    
    // We disable IME so nested interrupts don't occur
    cpu.ime = false;
    
    // We check each interrupt in priority order and service the first one found
    let (int_bit, handler_addr) = if triggered & INT_VBLANK != 0 {
        (INT_VBLANK, INT_VBLANK_ADDR)
    } else if triggered & INT_LCD_STAT != 0 {
        (INT_LCD_STAT, INT_LCD_STAT_ADDR)
    } else if triggered & INT_TIMER != 0 {
        (INT_TIMER, INT_TIMER_ADDR)
    } else if triggered & INT_SERIAL != 0 {
        (INT_SERIAL, INT_SERIAL_ADDR)
    } else if triggered & INT_JOYPAD != 0 {
        (INT_JOYPAD, INT_JOYPAD_ADDR)
    } else {
        return 0;
    };
    
    // We clear this interrupt's pending flag
    mmu.write_byte(0xFF0F, if_reg & !int_bit);
    
    // We push the current PC onto the stack (like a CALL instruction)
    cpu.registers.sp = cpu.registers.sp.wrapping_sub(2);
    mmu.write_word(cpu.registers.sp, cpu.registers.pc);
    
    // We jump to the interrupt handler
    cpu.registers.pc = handler_addr;
    
    // Servicing an interrupt takes 20 cycles (5 M-cycles)
    20
}

/// This requests an interrupt by setting the corresponding bit in IF
pub fn request_interrupt(mmu: &mut Mmu, interrupt: u8) {
    let if_reg = mmu.read_byte(0xFF0F);
    mmu.write_byte(0xFF0F, if_reg | interrupt);
}
