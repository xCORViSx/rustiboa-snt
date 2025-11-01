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

/// This struct represents the Game Boy's Memory Management Unit which maps all
/// memory addresses to their corresponding regions (ROM, RAM, VRAM, I/O, etc.)
pub struct Mmu {
    /// Optional boot ROM (256 bytes at 0x0000-0x00FF)
    boot_rom: Option<Vec<u8>>,
    
    /// Whether the boot ROM is currently mapped at 0x0000-0x00FF
    pub boot_rom_enabled: bool,
    
    /// Cartridge ROM (16KB+ depending on MBC)
    rom: Vec<u8>,
    
    /// Video RAM (8KB at 0x8000-0x9FFF)
    vram: [u8; 0x2000],
    
    /// External/Cartridge RAM (8KB+ depending on MBC, at 0xA000-0xBFFF)
    eram: [u8; 0x2000],
    
    /// Work RAM (8KB at 0xC000-0xDFFF)
    wram: [u8; 0x2000],
    
    /// Object Attribute Memory (160 bytes at 0xFE00-0xFE9F)
    oam: [u8; 0xA0],
    
    /// I/O Registers (128 bytes at 0xFF00-0xFF7F)
    io_registers: [u8; 0x80],
    
    /// High RAM (127 bytes at 0xFF80-0xFFFE)
    hram: [u8; 0x7F],
    
    /// Interrupt Enable register (at 0xFFFF)
    ie: u8,
    
    // MBC1 banking state
    /// Whether RAM is enabled for read/write
    ram_enabled: bool,
    /// Currently selected ROM bank (1-31)
    rom_bank: u8,
    /// Currently selected RAM bank or upper ROM bits (0-3)
    ram_bank: u8,
    /// Banking mode: false = ROM mode, true = RAM mode
    banking_mode: bool,
    
    // OAM DMA state
    /// Whether a DMA transfer is currently active
    dma_active: bool,
    /// Source address for DMA (high byte from 0xFF46)
    dma_source: u16,
    /// Current progress in the DMA transfer (0-160 bytes)
    dma_progress: u8,
    
    // Serial port output for test ROM results
    /// Accumulated serial port output (test ROMs print results here)
    pub serial_output: String,
    
    /// Gameboy Doctor mode: always return 0x90 for LY register
    pub doctor_mode: bool,
}impl Mmu {
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
            // OAM DMA starts inactive
            dma_active: false,
            dma_source: 0,
            dma_progress: 0,
            // Serial port output starts empty
            serial_output: String::new(),
            // Gameboy Doctor mode starts disabled
            doctor_mode: false,
        };
        
        // Initialize I/O registers to post-boot state
        mmu.write_byte(0xFF40, 0x91);  // LCDC: LCD on, BG on, BG tile map 9800
        mmu.write_byte(0xFF41, 0x81);  // STAT: Mode 1 (as per DMG boot state)
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
                // Special handling for LY register in Gameboy Doctor mode
                if self.doctor_mode && address == 0xFF44 {
                    0x90
                } else {
                    self.io_registers[(address - 0xFF00) as usize]
                }
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
                if address == 0xFF01 {
                    // Serial Data (SB) - Blargg tests write ASCII characters here
                    // We accumulate them in serial_output for test result reading
                    self.io_registers[0x01] = value;
                    if value >= 0x20 && value <= 0x7E {
                        // Only accumulate printable ASCII characters
                        self.serial_output.push(value as char);
                    }
                } else if address == 0xFF02 {
                    // Serial Control (SC) - writing 0x81 triggers a transfer
                    // For test ROMs, we just acknowledge the write
                    self.io_registers[0x02] = value;
                    // Clear transfer flag after "transfer" completes instantly
                    if value & 0x80 != 0 {
                        self.io_registers[0x02] = value & 0x7F;
                    }
                } else if address == 0xFF04 {
                    // Writing ANY value to DIV (0xFF04) resets it to 0
                    self.io_registers[(address - 0xFF00) as usize] = 0;
                } else if address == 0xFF46 {
                    // Writing to 0xFF46 (DMA register) starts OAM DMA transfer
                    // The value written is the source address divided by 0x100
                    // Transfer copies 160 bytes from source to OAM (0xFE00-0xFE9F)
                    self.dma_source = (value as u16) << 8;  // Convert to full address
                    self.dma_active = true;
                    self.dma_progress = 0;
                    self.io_registers[(address - 0xFF00) as usize] = value;
                } else if address == 0xFF50 && value != 0 {
                    // Writing to 0xFF50 disables boot ROM
                    self.boot_rom_enabled = false;
                    self.io_registers[(address - 0xFF00) as usize] = value;
                } else {
                    self.io_registers[(address - 0xFF00) as usize] = value;
                }
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
    
    /// This advances OAM DMA by one M-cycle if a transfer is active.
    /// OAM DMA transfers one byte per M-cycle from source to OAM.
    /// The transfer takes 160 M-cycles total (160 bytes: 0xFE00-0xFE9F).
    pub fn tick_dma(&mut self) {
        // We check if DMA transfer is currently active
        if !self.dma_active {
            return;
        }
        
        // We calculate the source and destination addresses for this byte
        let source_addr = ((self.dma_source as u16) << 8) | (self.dma_progress as u16);
        
        // We read from source and write to OAM
        // Note: We need to read directly from memory regions to avoid recursion
        let byte = match source_addr {
            0x0000..=0x7FFF => self.rom.get(source_addr as usize).copied().unwrap_or(0xFF),
            0x8000..=0x9FFF => self.vram[(source_addr - 0x8000) as usize],
            0xA000..=0xBFFF => self.eram[(source_addr - 0xA000) as usize],
            0xC000..=0xDFFF => self.wram[(source_addr - 0xC000) as usize],
            0xE000..=0xFDFF => self.wram[(source_addr - 0xE000) as usize],
            _ => 0xFF,
        };
        
        // We write to OAM memory
        self.oam[self.dma_progress as usize] = byte;
        
        // We advance the progress counter
        self.dma_progress += 1;
        
        // When we've transferred all 160 bytes, DMA is complete
        if self.dma_progress >= 160 {
            self.dma_active = false;
        }
    }
    
    /// This increments the DIV register directly without triggering the reset logic.
    /// Used by the timer to update DIV every 256 CPU cycles.
    pub fn increment_div(&mut self) {
        // DIV is at 0xFF04, which maps to io_registers[0x04]
        self.io_registers[0x04] = self.io_registers[0x04].wrapping_add(1);
    }
}
