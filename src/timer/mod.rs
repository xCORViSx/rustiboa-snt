// REMINDER: Read AGENTS.md file before continuing development
//
// Timer System
//
// This module implements the Game Boy's timer registers. The timer has a divider
// register (DIV) that increments at 16384 Hz, and a programmable timer (TIMA)
// that can run at 4 different frequencies selected by TAC. When TIMA overflows,
// it loads the value from TMA and requests a timer interrupt.

use crate::mmu::Mmu;
use crate::interrupts;

/// Timer frequencies in CPU clocks (4.194304 MHz = 4194304 Hz)
/// These are the number of CPU cycles between TIMA increments
const TIMER_FREQ_4096: u16 = 1024;   // 4194304 / 4096 = 1024 cycles
const TIMER_FREQ_262144: u16 = 16;    // 4194304 / 262144 = 16 cycles
const TIMER_FREQ_65536: u16 = 64;     // 4194304 / 65536 = 64 cycles
const TIMER_FREQ_16384: u16 = 256;    // 4194304 / 16384 = 256 cycles

/// This struct tracks the internal timer state including cycle counters
pub struct Timer {
    /// Divider counter (increments every 256 cycles to update DIV)
    div_counter: u16,
    
    /// Timer counter (increments based on TAC frequency to update TIMA)
    tima_counter: u16,
}

impl Timer {
    /// This creates a new timer with everything at zero
    pub fn new() -> Self {
        Timer {
            div_counter: 0,
            tima_counter: 0,
        }
    }
    
    /// This advances the timer by the specified number of CPU cycles,
    /// updating DIV and TIMA registers and requesting timer interrupt on overflow
    pub fn tick(&mut self, cycles: u8, mmu: &mut Mmu) {
        // Update DIV register (increments at 16384 Hz = every 256 CPU cycles)
        self.div_counter += cycles as u16;
        if self.div_counter >= 256 {
            self.div_counter -= 256;
            let div = mmu.read_byte(0xFF04);
            mmu.write_byte(0xFF04, div.wrapping_add(1));
        }
        
        // Check if timer is enabled (bit 2 of TAC)
        let tac = mmu.read_byte(0xFF07);
        if tac & 0x04 == 0 {
            return; // Timer disabled
        }
        
        // Get timer frequency from TAC bits 0-1
        let frequency = match tac & 0x03 {
            0 => TIMER_FREQ_4096,
            1 => TIMER_FREQ_262144,
            2 => TIMER_FREQ_65536,
            3 => TIMER_FREQ_16384,
            _ => unreachable!(),
        };
        
        // Update TIMA based on selected frequency
        self.tima_counter += cycles as u16;
        while self.tima_counter >= frequency {
            self.tima_counter -= frequency;
            
            let tima = mmu.read_byte(0xFF05);
            if tima == 0xFF {
                // TIMA overflow: load TMA value and request timer interrupt
                let tma = mmu.read_byte(0xFF06);
                mmu.write_byte(0xFF05, tma);
                interrupts::request_interrupt(mmu, interrupts::INT_TIMER);
            } else {
                // Normal increment
                mmu.write_byte(0xFF05, tima + 1);
            }
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
