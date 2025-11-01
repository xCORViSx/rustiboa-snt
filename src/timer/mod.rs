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

/// Timer frequencies in M-cycles (CPU clock / 4)
/// These are the number of M-cycles between TIMA increments
const TIMER_FREQ_4096: u16 = 256;     // TAC=00: 4096 Hz = 256 M-cycles
const TIMER_FREQ_262144: u16 = 4;     // TAC=01: 262144 Hz = 4 M-cycles
const TIMER_FREQ_65536: u16 = 16;     // TAC=10: 65536 Hz = 16 M-cycles
const TIMER_FREQ_16384: u16 = 64;     // TAC=11: 16384 Hz = 64 M-cycles

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
    
    /// This advances the timer by the specified number of M-cycles,
    /// updating DIV and TIMA registers and requesting timer interrupt on overflow
    pub fn tick(&mut self, cycles: u8, mmu: &mut Mmu) {
        // Update DIV register (increments at 16384 Hz = every 64 M-cycles)
        self.div_counter += cycles as u16;
        if self.div_counter >= 64 {
            self.div_counter -= 64;
            mmu.increment_div();
        }
        
        // Check if timer is enabled (bit 2 of TAC)
        let tac = mmu.read_byte(0xFF07);
        if tac & 0x04 == 0 {
            // Timer disabled - reset counter when disabled
            self.tima_counter = 0;
            return;
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
