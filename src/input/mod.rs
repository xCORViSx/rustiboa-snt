// REMINDER: Read AGENTS.md file before continuing development
//
// Input Module - Joypad handling
//
// This module handles Game Boy joypad input. The joypad has 8 buttons mapped
// to I/O register 0xFF00: D-pad (Up, Down, Left, Right) and buttons (A, B, Start, Select).
// The register uses a matrix system where you select button or d-pad mode.

use sdl2::keyboard::Keycode;
use std::collections::HashSet;

/// This struct tracks which buttons are currently pressed and manages
/// the joypad state register that the Game Boy reads
pub struct Input {
    /// Keys currently pressed (from SDL2)
    keys_pressed: HashSet<Keycode>,
    
    /// Joypad register state (0xFF00)
    joypad_state: u8,
}

impl Input {
    /// This creates a new input handler with no keys pressed
    pub fn new() -> Self {
        Input {
            keys_pressed: HashSet::new(),
            joypad_state: 0xFF, // All bits high = no buttons pressed
        }
    }
    
    /// This handles an SDL2 key press event
    pub fn key_down(&mut self, keycode: Keycode) {
        self.keys_pressed.insert(keycode);
        self.update_joypad_state();
    }
    
    /// This handles an SDL2 key release event
    pub fn key_up(&mut self, keycode: Keycode) {
        self.keys_pressed.remove(&keycode);
        self.update_joypad_state();
    }
    
    /// This updates the internal joypad state based on currently pressed keys.
    /// The Game Boy joypad register uses active-low logic (0 = pressed).
    fn update_joypad_state(&mut self) {
        // TODO: Implement proper joypad matrix and register handling
        // For now we just store basic state
        self.joypad_state = 0xFF;
        
        // Map SDL keys to Game Boy buttons
        // Arrow keys = D-pad, Z/X = A/B, Enter/Shift = Start/Select
        // When a button is pressed, clear its bit (active low)
        
        if self.keys_pressed.contains(&Keycode::Right) {
            self.joypad_state &= !0x01;
        }
        if self.keys_pressed.contains(&Keycode::Left) {
            self.joypad_state &= !0x02;
        }
        if self.keys_pressed.contains(&Keycode::Up) {
            self.joypad_state &= !0x04;
        }
        if self.keys_pressed.contains(&Keycode::Down) {
            self.joypad_state &= !0x08;
        }
        if self.keys_pressed.contains(&Keycode::Z) {
            // A button
            self.joypad_state &= !0x10;
        }
        if self.keys_pressed.contains(&Keycode::X) {
            // B button
            self.joypad_state &= !0x20;
        }
        if self.keys_pressed.contains(&Keycode::Return) {
            // Start
            self.joypad_state &= !0x40;
        }
        if self.keys_pressed.contains(&Keycode::RShift) {
            // Select
            self.joypad_state &= !0x80;
        }
    }
    
    /// This returns the current joypad register value for the MMU to read
    pub fn read_joypad(&self) -> u8 {
        self.joypad_state
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}
