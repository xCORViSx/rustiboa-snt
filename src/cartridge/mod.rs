// REMINDER: Read AGENTS.md file before continuing development
//
// Cartridge Module - ROM loading and parsing
//
// This module handles loading Game Boy ROM files (.gb) and parsing the
// cartridge header which contains info about the game, cartridge type,
// ROM/RAM sizes, and which Memory Bank Controller (MBC) is used.

use std::fs::File;
use std::io::Read;
use std::path::Path;

/// This struct represents a loaded cartridge with its ROM data and metadata
pub struct Cartridge {
    /// The full ROM data loaded from the .gb file
    pub rom: Vec<u8>,
    
    /// Game title from the cartridge header
    pub title: String,
    
    /// Cartridge type (which MBC is used)
    pub cartridge_type: u8,
    
    /// ROM size in bytes
    pub rom_size: usize,
    
    /// RAM size in bytes (if cartridge has RAM)
    pub ram_size: usize,
}

impl Cartridge {
    /// This loads a Game Boy ROM file from disk and parses its header.
    /// The header is at addresses 0x0100-0x014F in the ROM.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let mut file = File::open(&path).map_err(|e| format!("Failed to open ROM: {}", e))?;
        
        let mut rom = Vec::new();
        file.read_to_end(&mut rom)
            .map_err(|e| format!("Failed to read ROM: {}", e))?;
        
        if rom.len() < 0x150 {
            return Err("ROM too small, invalid cartridge".to_string());
        }
        
        // We extract the game title from bytes 0x0134-0x0143
        let title_bytes = &rom[0x0134..=0x0143];
        let title = String::from_utf8_lossy(title_bytes)
            .trim_end_matches('\0')
            .to_string();
        
        // We read the cartridge type byte which tells us the MBC type
        let cartridge_type = rom[0x0147];
        
        // We calculate ROM size from the size code at 0x0148
        let rom_size_code = rom[0x0148];
        let rom_size = (32 * 1024) << rom_size_code; // 32KB << code
        
        // We calculate RAM size from the size code at 0x0149
        let ram_size_code = rom[0x0149];
        let ram_size = match ram_size_code {
            0x00 => 0,
            0x01 => 2 * 1024,    // 2KB
            0x02 => 8 * 1024,    // 8KB
            0x03 => 32 * 1024,   // 32KB (4 banks of 8KB)
            0x04 => 128 * 1024,  // 128KB (16 banks of 8KB)
            0x05 => 64 * 1024,   // 64KB (8 banks of 8KB)
            _ => 0,
        };
        
        Ok(Cartridge {
            rom,
            title,
            cartridge_type,
            rom_size,
            ram_size,
        })
    }
    
    /// This returns a string describing the cartridge type
    pub fn cartridge_type_name(&self) -> &str {
        match self.cartridge_type {
            0x00 => "ROM ONLY",
            0x01 => "MBC1",
            0x02 => "MBC1+RAM",
            0x03 => "MBC1+RAM+BATTERY",
            0x05 => "MBC2",
            0x06 => "MBC2+BATTERY",
            0x08 => "ROM+RAM",
            0x09 => "ROM+RAM+BATTERY",
            0x0F => "MBC3+TIMER+BATTERY",
            0x10 => "MBC3+TIMER+RAM+BATTERY",
            0x11 => "MBC3",
            0x12 => "MBC3+RAM",
            0x13 => "MBC3+RAM+BATTERY",
            0x19 => "MBC5",
            0x1A => "MBC5+RAM",
            0x1B => "MBC5+RAM+BATTERY",
            _ => "UNKNOWN",
        }
    }
}
