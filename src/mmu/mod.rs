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
    
    /// Cartridge ROM (loaded from .gb file)
    rom: Vec<u8>,
    
    /// Video RAM - 8KB for tiles and tile maps
    vram: [u8; 0x2000],
    
    /// External cartridge RAM - 8KB (size varies by cartridge)
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
    
    // MBC1 state
    /// Whether RAM is enabled (0x0A in 0x0000-0x1FFF enables it)
    ram_enabled: bool,
    
    /// ROM bank number (5 bits, 0x01-0x1F, bank 0 not selectable for 0x4000-0x7FFF)
    rom_bank: u8,
    
    /// RAM bank number (2 bits, 0x00-0x03) or upper bits of ROM bank in ROM banking mode
    ram_bank: u8,
    
    /// Banking mode: false = ROM banking (default), true = RAM banking
    banking_mode: bool,
}

impl Mmu {
    /// This creates a new MMU with all memory regions initialized.
    /// The rom parameter is the cartridge data loaded from a .gb file.
    pub fn new(rom: Vec<u8>) -> Self {
        let mut mmu = Mmu {
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
            // MBC1 starts with ROM bank 1 selected for 0x4000-0x7FFF
            ram_enabled: false,
            rom_bank: 1,
            ram_bank: 0,
            banking_mode: false,
        };
        
        // Initialize I/O registers to post-boot state
        mmu.write_byte(0xFF40, 0x91);  // LCDC: LCD on, BG on, BG tile map 9800
        mmu.write_byte(0xFF47, 0xFC);  // BGP: Background palette
        
        mmu
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
                // ROM Bank 0 (or higher banks in RAM banking mode)
                let bank = if self.banking_mode {
                    // In RAM banking mode, upper 2 bits can be applied to bank 0 access
                    (self.ram_bank << 5) as usize
                } else {
                    0
                };
                let addr = (bank * 0x4000) + (address as usize);
                self.rom.get(addr).copied().unwrap_or(0xFF)
            }
            // ROM Bank 1-N (switchable via MBC1)
            0x4000..=0x7FFF => {
                // Combine 5-bit ROM bank with 2-bit RAM bank (used as upper ROM bits)
                let bank = (self.rom_bank | (self.ram_bank << 5)) as usize;
                // Bank 0 is not allowed for this region, treat as bank 1
                let effective_bank = if bank == 0 { 1 } else { bank };
                let addr = (effective_bank * 0x4000) + ((address - 0x4000) as usize);
                self.rom.get(addr).copied().unwrap_or(0xFF)
            }
            // Video RAM
            0x8000..=0x9FFF => {
                self.vram[(address - 0x8000) as usize]
            }
            // External RAM (MBC1 controlled)
            0xA000..=0xBFFF => {
                if !self.ram_enabled {
                    return 0xFF;
                }
                let bank = if self.banking_mode { self.ram_bank } else { 0 };
                let addr = ((bank as usize) * 0x2000) + ((address - 0xA000) as usize);
                // Clamp to available RAM
                if addr < self.eram.len() {
                    self.eram[addr]
                } else {
                    0xFF
                }
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
            // MBC1: RAM Enable (0x0000-0x1FFF)
            0x0000..=0x1FFF => {
                // Writing 0x0A to this range enables RAM, anything else disables it
                self.ram_enabled = (value & 0x0F) == 0x0A;
            }
            // MBC1: ROM Bank Number (0x2000-0x3FFF)
            0x2000..=0x3FFF => {
                // Lower 5 bits select ROM bank (1-31)
                let bank = value & 0x1F;
                // Bank 0 is treated as bank 1
                self.rom_bank = if bank == 0 { 1 } else { bank };
            }
            // MBC1: RAM Bank Number or Upper ROM Bank bits (0x4000-0x5FFF)
            0x4000..=0x5FFF => {
                // Lower 2 bits - used as RAM bank or upper ROM bank bits
                self.ram_bank = value & 0x03;
            }
            // MBC1: Banking Mode Select (0x6000-0x7FFF)
            0x6000..=0x7FFF => {
                // 0 = ROM banking mode (default), 1 = RAM banking mode
                self.banking_mode = (value & 0x01) == 0x01;
            }
            // Video RAM
            0x8000..=0x9FFF => {
                self.vram[(address - 0x8000) as usize] = value;
            }
            // External RAM (MBC1 controlled)
            0xA000..=0xBFFF => {
                if !self.ram_enabled {
                    return;
                }
                let bank = if self.banking_mode { self.ram_bank } else { 0 };
                let addr = ((bank as usize) * 0x2000) + ((address - 0xA000) as usize);
                // Only write if within RAM bounds
                if addr < self.eram.len() {
                    self.eram[addr] = value;
                }
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
