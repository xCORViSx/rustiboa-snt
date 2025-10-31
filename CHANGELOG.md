<!--REMINDER: Read AGENTS.md file before continuing development-->

# CHANGELOG

All notable changes to Rustiboa-SNT will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### New Features

- MBC1 memory bank controller with ROM/RAM banking and mode selection
- STAT register updates with current PPU mode (0-3) every tick
- STAT register initialized to 0x81 at boot (Mode 1)
- OAM DMA implementation (0xFF46 register, 160-byte transfer in 160 M-cycles)
- STAT interrupts for PPU mode changes (Mode 0/1/2 based on STAT enable bits)
- HALT wake-up on any enabled+pending interrupt (even if IME=0)
- DIV register (0xFF04) reset to 0 on any write
- PC tracking for infinite loop detection
- Extensive debug output for ROM testing (VRAM inspection, tile fetch logging, frame timing)

### Fixed

- PPU FIFO deadlock preventing pixel rendering (changed condition from len>8 to !empty)
- Missing LCDC initialization causing LCD to be disabled
- Missing LY register updates during PPU tick
- Missing STAT register mode updates
- HALT not waking up CPU properly on interrupts
- DIV register not resetting on writes
- LCDC register initialization (0x91 - LCD enabled, BG on)
- BGP register initialization (0xFC - palette)
- LCD enable checking in PPU tick
- halt_bug.gb test ROM now displays correctly

### Known Issues

- Link's Awakening shows blank screen - game fills tile map (all 0x7F) but never uploads tile graphics to VRAM address 0x87F0
- Game logic runs (not stuck in infinite loop) but appears waiting for specific hardware condition
- Issue likely related to:
  - CPU instruction implementation bugs (need test suite to verify all 512 instructions)
  - Missing official boot ROM (game may require specific boot ROM initialization)
  - Insufficient timing accuracy (game may need cycle-perfect PPU/CPU synchronization)
  - Missing or incorrectly implemented hardware features
- Requires CPU instruction test suite (Blargg's tests) or official boot ROM for further investigation
- No other commercial ROMs available for comparison testing

## [0.1.0] - 2025-01-30

### Added

- Initial project structure with Cargo.toml and source directories
- Essential documentation (README, REFERENCES, ROADMAP, CHANGELOG)
- Technical reference gathering from Pan Docs, emulator development guides
- Project planning and component breakdown
- CPU module with register definitions and complete instruction set (256 base + 256 CB-prefixed opcodes)
- Complete CPU instruction implementations:
  - Load operations (8-bit and 16-bit, including LD, LDH, LDI, LDD)
  - Arithmetic operations (ADD, ADC, SUB, SBC, INC, DEC)
  - Logic operations (AND, XOR, OR, CP)
  - Rotate/shift operations (RLCA, RRCA, RLA, RRA, RLC, RRC, RL, RR, SLA, SRA, SRL, SWAP)
  - Control flow (JP, JR, CALL, RET, RETI, RST with conditional variants)
  - Stack operations (PUSH, POP)
  - Bit operations (BIT, SET, RES)
  - Miscellaneous (NOP, STOP, HALT, DI, EI, DAA, CPL, SCF, CCF)
- MMU module with complete memory map implementation (boot ROM, cartridge, VRAM, WRAM, OAM, I/O, HRAM)
- PPU module with state machine structure (OAM Search, Pixel Transfer, HBlank, VBlank)
- Display module with SDL2 rendering pipeline (160x144 with 4x scaling, Game Boy palette)
- Input module with joypad handling framework
- Cartridge module with ROM loading and header parsing
- Successfully compiling project with all 512 CPU instructions
- Opcode reference documentation from Pastraiser and izik1's gbops
- Main emulation loop connecting CPU, MMU, PPU, display, and input
- SDL2 bundled dependency configured and built successfully
- Complete working build - Release binary (421KB) compiles cleanly with Rust edition 2024
- Complete interrupt system with handler dispatch for all 5 interrupt types (VBlank, LCD STAT, Timer, Serial, Joypad)
- Interrupt priority handling and IE/IF register support
- Timer system with DIV, TIMA, TMA, TAC registers
- 4 timer frequency modes (4096 Hz, 262144 Hz, 65536 Hz, 16384 Hz)
- Timer overflow handling with TMA reload and interrupt generation
- Feature-complete emulator ready for ROM testing
- VS Code tasks.json with 7 build/run/test configurations
- Comprehensive testing guide in refs/TESTING.md
- Clippy compliance (0 warnings) with idiomatic Rust patterns
- [GitHub repository](https://github.com/xCORViSx/rustiboa-snt) published
- v0.1.0 release tagged and published

### Planned

- Sharp LR35902 CPU with full instruction set
- Memory Management Unit with complete address space
- Picture Processing Unit with background rendering
- SDL2-based display output
- Joypad input handling
- Interrupt system
- Timer registers
- Cartridge ROM loading
- Boot ROM support
