// REMINDER: Read AGENTS.md file before continuing development
//
// Picture Processing Unit (PPU)
//
// This module implements the Game Boy's PPU which renders graphics to the screen.
// It operates as a state machine with four states: OAM Search, Pixel Transfer,
// HBlank, and VBlank. The PPU runs at 456 dots per scanline (154 scanlines per frame)
// generating the 160x144 pixel display using tiles from VRAM.

/// PPU state machine states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PpuState {
    /// Scanning OAM for sprites on this scanline (80 dots)
    OamSearch,
    /// Fetching and rendering pixels (172-289 dots depending on sprites/scrolling)
    PixelTransfer,
    /// Horizontal blank period at end of scanline (remaining dots to reach 456)
    HBlank,
    /// Vertical blank period after all 144 scanlines (4560 dots, 10 scanlines)
    VBlank,
}

/// This struct represents the PPU's state including timing, current scanline,
/// pixel FIFO, and the framebuffer that gets sent to the display
pub struct Ppu {
    /// Current PPU state
    state: PpuState,
    
    /// Dot counter within current scanline (0-455)
    dots: u16,
    
    /// Current scanline being drawn (LY register, 0-153)
    ly: u8,
    
    /// Current X position in scanline (0-159) - pixels pushed to screen
    x: u8,
    
    /// Fetcher state for background tiles
    fetcher_x: u8,
    fetcher_step: u8,
    
    /// Pixel FIFO for background pixels (holds color IDs 0-3)
    bg_fifo: Vec<u8>,
    
    /// Tile data being fetched
    tile_id: u8,
    tile_data_low: u8,
    tile_data_high: u8,
    
    /// Framebuffer holding pixel data (160x144 pixels, 4 shades of gray)
    pub framebuffer: [u8; 160 * 144],
    
    /// Frame complete flag
    frame_ready: bool,
}

impl Ppu {
    /// This creates a new PPU with everything initialized to power-on state
    pub fn new() -> Self {
        Ppu {
            state: PpuState::OamSearch,
            dots: 0,
            ly: 0,
            x: 0,
            fetcher_x: 0,
            fetcher_step: 0,
            bg_fifo: Vec::with_capacity(16),
            tile_id: 0,
            tile_data_low: 0,
            tile_data_high: 0,
            framebuffer: [0; 160 * 144],
            frame_ready: false,
        }
    }
    
    /// This advances the PPU by one dot (T-cycle), updating its state and potentially
    /// rendering pixels. Returns true when a frame is complete (VBlank starts).
    pub fn tick(&mut self, mmu: &mut crate::mmu::Mmu) -> bool {
        // Check if LCD is enabled (LCDC bit 7)
        let lcdc = mmu.read_byte(0xFF40);
        if (lcdc & 0x80) == 0 {
            // LCD is off - don't advance PPU
            return false;
        }
        
        self.dots += 1;
        
        // We handle each PPU mode based on current state
        match self.state {
            PpuState::OamSearch => {
                // Mode 2: We scan OAM for sprites overlapping this scanline
                if self.dots >= 80 {
                    self.state = PpuState::PixelTransfer;
                    self.x = 0;
                    self.fetcher_x = 0;
                    self.fetcher_step = 0;
                    self.bg_fifo.clear();
                }
            }
            
            PpuState::PixelTransfer => {
                // Mode 3: We fetch tiles and push pixels to the screen
                self.fetch_pixel(mmu);
                
                // We try to push a pixel from FIFO to screen if we have enough
                if !self.bg_fifo.is_empty() && self.x < 160 {
                    let color_id = self.bg_fifo.remove(0);
                    let color = self.get_color(color_id, mmu);
                    let index = (self.ly as usize * 160) + self.x as usize;
                    self.framebuffer[index] = color;
                    self.x += 1;
                }
                
                // When we've rendered all 160 pixels, we move to HBlank
                if self.x >= 160 {
                    self.state = PpuState::HBlank;
                }
            }
            
            PpuState::HBlank => {
                // Mode 0: We wait until the scanline completes (456 dots total)
                if self.dots >= 456 {
                    self.dots = 0;
                    self.ly += 1;
                    mmu.write_byte(0xFF44, self.ly);  // Update LY register
                    
                    // After scanline 143, we enter VBlank
                    if self.ly >= 144 {
                        self.state = PpuState::VBlank;
                        self.frame_ready = true;
                        // Request VBlank interrupt
                        crate::interrupts::request_interrupt(mmu, crate::interrupts::INT_VBLANK);
                    } else {
                        self.state = PpuState::OamSearch;
                    }
                }
            }
            
            PpuState::VBlank => {
                // Mode 1: We wait for remaining scanlines (144-153)
                if self.dots >= 456 {
                    self.dots = 0;
                    self.ly += 1;
                    mmu.write_byte(0xFF44, self.ly);  // Update LY register
                    
                    // After scanline 153, we restart from scanline 0
                    if self.ly > 153 {
                        self.ly = 0;
                        mmu.write_byte(0xFF44, 0);
                        self.state = PpuState::OamSearch;
                    }
                }
            }
        }
        
        // We return and clear the frame_ready flag
        let ready = self.frame_ready;
        self.frame_ready = false;
        ready
    }
    
    /// This implements the pixel fetcher state machine that reads tiles from VRAM
    /// and pushes pixel data into the FIFO (8 pixels at a time from each tile)
    fn fetch_pixel(&mut self, mmu: &crate::mmu::Mmu) {
        // We run the fetcher every 2 dots (fetcher operates at half speed)
        if !self.dots.is_multiple_of(2) {
            return;
        }
        
        // The fetcher has 4 steps to fetch one tile (8 pixels):
        // 0: Get tile ID from tile map
        // 1: Get tile data low byte
        // 2: Get tile data high byte
        // 3: Push pixels to FIFO
        match self.fetcher_step {
            0 => {
                // Step 0: We read the tile ID from the background tile map
                let scx = mmu.read_byte(0xFF43); // Scroll X
                let scy = mmu.read_byte(0xFF42); // Scroll Y
                
                // Calculate tile map position including scroll
                let map_x = ((self.fetcher_x + (scx / 8)) % 32) as u16;
                let map_y = (((self.ly + scy) / 8) % 32) as u16;
                
                // Read from tile map (we use $9800 map for now, LCDC.3 selects map)
                let tile_map_addr = 0x9800 + (map_y * 32) + map_x;
                self.tile_id = mmu.read_byte(tile_map_addr);
                
                self.fetcher_step = 1;
            }
            
            1 => {
                // Step 1: We read the low byte of tile data
                let scy = mmu.read_byte(0xFF42);
                let tile_line = ((self.ly + scy) % 8) as u16; // Which line of the tile (0-7)
                
                // Calculate tile data address (we use $8000 addressing for now)
                let tile_data_addr = 0x8000 + (self.tile_id as u16 * 16) + (tile_line * 2);
                self.tile_data_low = mmu.read_byte(tile_data_addr);
                
                self.fetcher_step = 2;
            }
            
            2 => {
                // Step 2: We read the high byte of tile data
                let scy = mmu.read_byte(0xFF42);
                let tile_line = ((self.ly + scy) % 8) as u16;
                
                let tile_data_addr = 0x8000 + (self.tile_id as u16 * 16) + (tile_line * 2) + 1;
                self.tile_data_high = mmu.read_byte(tile_data_addr);
                
                self.fetcher_step = 3;
            }
            
            3 => {
                // Step 3: We push 8 pixels into the FIFO (only if FIFO is empty enough)
                if self.bg_fifo.len() <= 8 {
                    // We decode the 8 pixels from the two tile data bytes
                    for bit_pos in (0..8).rev() {
                        let low_bit = (self.tile_data_low >> bit_pos) & 1;
                        let high_bit = (self.tile_data_high >> bit_pos) & 1;
                        let color_id = (high_bit << 1) | low_bit;
                        self.bg_fifo.push(color_id);
                    }
                    
                    // Move to next tile
                    self.fetcher_x += 1;
                    self.fetcher_step = 0;
                }
            }
            
            _ => unreachable!(),
        }
    }
    
    /// This converts a color ID (0-3) to an actual color using the BGP palette
    fn get_color(&self, color_id: u8, mmu: &crate::mmu::Mmu) -> u8 {
        let bgp = mmu.read_byte(0xFF47); // Background palette register
        
        (bgp >> (color_id * 2)) & 0x03
    }
    
    /// This returns the current scanline (LY register value)
    pub fn ly(&self) -> u8 {
        self.ly
    }
    
    /// This returns the current PPU mode for the STAT register
    pub fn mode(&self) -> u8 {
        match self.state {
            PpuState::HBlank => 0,
            PpuState::VBlank => 1,
            PpuState::OamSearch => 2,
            PpuState::PixelTransfer => 3,
        }
    }
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}

