// REMINDER: Read AGENTS.md file before continuing development
//
// Memory Management Unit (MMU)
//
// This module handles all memory access in the Game Boy. The memory map is:
// 0x0000-0x00FF: Boot ROM (disabled after boot)
// 0x0000-0x3FFF: ROM Bank 0 (from cartridge)
// 0x4000-0x7FFF: ROM Bank 1-N (switchable, from cartridge)
// 0x8000-0x9FFF: Video RAM (VRAM)
// 0xA000-0xBFFF: External RAM (from cartridge, if present)
// 0xC000-0xDFFF: Work RAM (WRAM)
// 0xE000-0xFDFF: Echo RAM (mirror of WRAM)
// 0xFE00-0xFE9F: Object Attribute Memory (OAM, sprite info)
// 0xFEA0-0xFEFF: Unusable
// 0xFF00-0xFF7F: I/O Registers
// 0xFF80-0xFFFE: High RAM (HRAM)
// 0xFFFF: Interrupt Enable register

/// This struct represents all memory in the Game Boy system and routes
/// read/write operations to the appropriate memory region
pub struct Mmu {
    /// Boot ROM (256 bytes) - runs first then gets disabled
    boot_rom: Option<Vec<u8>>,
    
    /// Whether boot ROM is currently enabled (starts true, switched off by writing to 0xFF50)
    boot_rom_enabled: bool,
    
    /// Cartridge ROM (loaded from .gb file) - TODO: implement banking
    rom: Vec<u8>,
    
    /// Video RAM - 8KB for tiles and tile maps
    vram: [u8; 0x2000],
    
    /// External cartridge RAM - 8KB (size varies by cartridge) - TODO: implement
    eram: [u8; 0x2000],
    
    /// Work RAM - 8KB for general program use
    wram: [u8; 0x2000],
    
    /// Object Attribute Memory - sprite data
    oam: [u8; 0xA0],
    
    /// I/O Registers - hardware control
    io_registers: [u8; 0x80],
    
    /// High RAM - fast 127-byte RAM
    hram: [u8; 0x7F],
    
    /// Interrupt Enable register
    ie: u8,
}

impl Mmu {
    /// This creates a new MMU with all memory regions initialized.
    /// The rom parameter is the cartridge data loaded from a .gb file.
    pub fn new(rom: Vec<u8>) -> Self {
        Mmu {
            boot_rom: None,  // TODO: optionally load boot ROM
            boot_rom_enabled: false,  // Start with boot ROM disabled for now
            rom,
            vram: [0; 0x2000],
            eram: [0; 0x2000],
            wram: [0; 0x2000],
            oam: [0; 0xA0],
            io_registers: [0; 0x80],
            hram: [0; 0x7F],
            ie: 0,
        }
    }
    
    /// This reads a byte from memory at the given address. We check which
    /// region the address falls into and return the appropriate byte.
    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            // Boot ROM or ROM Bank 0
            0x0000..=0x00FF => {
                if self.boot_rom_enabled && self.boot_rom.is_some() {
                    self.boot_rom.as_ref().unwrap()[address as usize]
                } else {
                    self.rom.get(address as usize).copied().unwrap_or(0xFF)
                }
            }
            0x0100..=0x3FFF => {
                self.rom.get(address as usize).copied().unwrap_or(0xFF)
            }
            // ROM Bank 1-N (TODO: implement banking)
            0x4000..=0x7FFF => {
                self.rom.get(address as usize).copied().unwrap_or(0xFF)
            }
            // Video RAM
            0x8000..=0x9FFF => {
                self.vram[(address - 0x8000) as usize]
            }
            // External RAM (TODO: implement properly with banking)
            0xA000..=0xBFFF => {
                self.eram[(address - 0xA000) as usize]
            }
            // Work RAM
            0xC000..=0xDFFF => {
                self.wram[(address - 0xC000) as usize]
            }
            // Echo RAM (mirror of WRAM)
            0xE000..=0xFDFF => {
                self.wram[(address - 0xE000) as usize]
            }
            // Object Attribute Memory
            0xFE00..=0xFE9F => {
                self.oam[(address - 0xFE00) as usize]
            }
            // Unusable memory
            0xFEA0..=0xFEFF => 0xFF,
            // I/O Registers
            0xFF00..=0xFF7F => {
                self.io_registers[(address - 0xFF00) as usize]
            }
            // High RAM
            0xFF80..=0xFFFE => {
                self.hram[(address - 0xFF80) as usize]
            }
            // Interrupt Enable register
            0xFFFF => self.ie,
        }
    }
    
    /// This writes a byte to memory at the given address. Some regions
    /// are read-only (like ROM) and writes to them may trigger special behavior.
    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            // ROM area - writes don't modify ROM but may control banking
            0x0000..=0x7FFF => {
                // TODO: Implement MBC (Memory Bank Controller) logic
            }
            // Video RAM
            0x8000..=0x9FFF => {
                self.vram[(address - 0x8000) as usize] = value;
            }
            // External RAM
            0xA000..=0xBFFF => {
                self.eram[(address - 0xA000) as usize] = value;
            }
            // Work RAM
            0xC000..=0xDFFF => {
                self.wram[(address - 0xC000) as usize] = value;
            }
            // Echo RAM (writes go to WRAM)
            0xE000..=0xFDFF => {
                self.wram[(address - 0xE000) as usize] = value;
            }
            // Object Attribute Memory
            0xFE00..=0xFE9F => {
                self.oam[(address - 0xFE00) as usize] = value;
            }
            // Unusable memory
            0xFEA0..=0xFEFF => {}
            // I/O Registers
            0xFF00..=0xFF7F => {
                // Special handling for certain registers
                if address == 0xFF50 && value != 0 {
                    // Writing to 0xFF50 disables boot ROM
                    self.boot_rom_enabled = false;
                }
                self.io_registers[(address - 0xFF00) as usize] = value;
            }
            // High RAM
            0xFF80..=0xFFFE => {
                self.hram[(address - 0xFF80) as usize] = value;
            }
            // Interrupt Enable register
            0xFFFF => {
                self.ie = value;
            }
        }
    }
    
    /// This reads a 16-bit word from memory (little-endian: low byte first)
    pub fn read_word(&self, address: u16) -> u16 {
        let low = self.read_byte(address) as u16;
        let high = self.read_byte(address.wrapping_add(1)) as u16;
        (high << 8) | low
    }
    
    /// This writes a 16-bit word to memory (little-endian: low byte first)
    pub fn write_word(&mut self, address: u16, value: u16) {
        self.write_byte(address, (value & 0xFF) as u8);
        self.write_byte(address.wrapping_add(1), (value >> 8) as u8);
    }
}
