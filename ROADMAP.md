<!--REMINDER: Read AGENTS.md file before continuing development-->

# ROADMAP

## Project Goal

Build a cycle-accurate DMG (original Game Boy) emulator in Rust capable of running the boot ROM and commercial Game Boy games.

## Phase 1: Foundation ✓

- [x] Create project structure and essential documentation
- [x] Gather technical references for Game Boy hardware
- [x] Initialize Cargo project with Rust edition 2024
- [x] Set up module structure (cpu, mmu, ppu, display, input, cartridge)
- [x] Implement basic register system
- [x] Implement complete memory map in MMU
- [x] Create SDL2 display framework
- [x] Verify project compiles successfully

## Phase 2: Core Components ✓

- [x] **CPU Implementation** - Complete!
  - [x] Sharp LR35902 registers and flags
  - [x] Instruction decode and dispatch
  - [x] All opcodes (256 base + 256 CB-prefixed)
  - [x] Cycle-accurate timing
  
- [x] **Memory Management Unit (MMU)**
  - [x] Memory map implementation
  - [x] Boot ROM handling (0x0000-0x00FF)
  - [x] Cartridge ROM mapping
  - [x] VRAM (0x8000-0x9FFF)
  - [x] Working RAM (0xC000-0xDFFF)
  - [x] OAM (0xFE00-0xFE9F)
  - [x] I/O registers (0xFF00-0xFF7F)
  - [x] High RAM (0xFF80-0xFFFE)

## Phase 3: Graphics ✓

- [x] **Picture Processing Unit (PPU)**
  - [x] PPU state machine framework
  - [x] Framebuffer allocation
  - [x] Pixel fetcher with FIFO
  - [x] Background tile rendering
  - [x] Tile map and tile data handling
  - [x] LY register and scanline tracking
  - [ ] Sprite rendering (40 sprites, 10 per line)
  
- [x] **Display System**
  - [x] SDL2 window setup (160x144)
  - [x] Framebuffer management
  - [x] Game Boy color palette
  - [x] VBlank synchronization

## Phase 4: Input & Interrupts ✓

- [x] **Input Handling**
  - [x] Joypad register (0xFF00)
  - [x] D-pad mapping
  - [x] Button mapping (A, B, Start, Select)
  - [x] SDL2 event handling
  
- [x] **Interrupt System**
  - [x] Interrupt Enable register (IE)
  - [x] Interrupt Flag register (IF)
  - [x] VBlank interrupt
  - [x] LCD STAT interrupt
  - [x] Timer interrupt
  - [x] Serial interrupt
  - [x] Joypad interrupt

## Phase 5: Timing & Cartridge ✓

- [x] **Timer System**
  - [x] DIV register (0xFF04)
  - [x] TIMA register (0xFF05)
  - [x] TMA register (0xFF06)
  - [x] TAC register (0xFF07)
  
- [x] **Cartridge Support**
  - [x] ROM file loading (.gb)
  - [x] Cartridge header parsing
  - [x] MBC1 support (ROM/RAM banking, mode selection)

## Phase 6: Testing & Polish (In Progress)

- [ ] Boot ROM verification
- [x] Test with test ROMs (halt_bug.gb ✓)
- [ ] Debug commercial ROM compatibility (Link's Awakening investigation ongoing)
- [ ] Improve timing accuracy for commercial games
- [x] Clippy and rustfmt compliance (0 warnings)
- [ ] Performance optimization
- [x] Build configuration (tasks.json)
- [x] Final documentation update

## Phase 7: Future Enhancements

- [ ] Game Boy Color (CGB) support
- [ ] Additional MBC types (MBC2, MBC3, MBC5)
- [ ] Save state functionality
- [ ] Audio Processing Unit (APU/Sound)
- [ ] Debugger with breakpoints
- [ ] ROM information display
- [ ] Configuration file support
- [ ] Controller support beyond keyboard

## Current Status

**Active Phase**: Phase 6 (Testing & Polish) - Debugging commercial ROM compatibility

**Next Milestone**: Achieve stable rendering for commercial games (Link's Awakening investigation)

**Current Investigation**: Link's Awakening shows blank screen - game fills tile map but never uploads tile graphics. Appears stuck in initialization waiting for specific hardware behavior. Debug output active for continued investigation. Issue likely requires:

- CPU instruction test suite to verify all 512 instructions work correctly
- Official boot ROM for proper hardware initialization
- More accurate PPU/CPU timing
- Additional hardware features not yet identified

**Recent Completion**:

- All 512 CPU instructions (256 base + 256 CB-prefixed) with proper flag handling and cycle-accurate timing
- PPU framework with complete pixel fetcher, FIFO, background tile rendering, and VBlank interrupt
- Complete interrupt system with priority handling for all 5 interrupt types
- STAT interrupts for PPU mode changes (HBlank, VBlank, OAM Search)
- Timer system with DIV, TIMA, TMA, TAC registers and 4 frequency modes
- Main emulation loop implemented and integrated
- SDL2 bundled build completed (2m 29s compilation time)
- **Project successfully compiles and links** - 421KB release binary ready
- **Emulator is feature-complete** and ready for testing with ROMs
- VS Code tasks.json with build/run/test configurations
- Comprehensive testing documentation in refs/TESTING.md
- **Clippy clean** (0 warnings) with all auto-fixes applied
- **MBC1 memory bank controller** implemented with ROM/RAM banking
- **PPU FIFO deadlock fixed** - changed condition from len>8 to !empty
- **LCDC/BGP/STAT registers initialized** - LCD enabled, background on, STAT mode tracking
- **Test ROM verified** - halt_bug.gb displays text correctly
- **OAM DMA implemented** - 0xFF46 register triggers 160-byte transfer in 160 M-cycles
- **HALT wake-up fixed** - CPU wakes on any enabled+pending interrupt even if IME=0
- **DIV register reset** - writing to 0xFF04 properly resets it to 0
- **STAT interrupts implemented** - Mode 0/1/2 interrupts trigger based on STAT enable bits
- Published v0.1.0 to GitHub
- All debugging features committed and pushed

