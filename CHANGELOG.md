<!--REMINDER: Read AGENTS.md file before continuing development-->

# CHANGELOG

All notable changes to Rustiboa-SNT will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
- **Complete working build** - Release binary (421KB) compiles cleanly with Rust edition 2024
- Complete interrupt system with handler dispatch for all 5 interrupt types (VBlank, LCD STAT, Timer, Serial, Joypad)
- Interrupt priority handling and IE/IF register support
- Timer system with DIV, TIMA, TMA, TAC registers
- 4 timer frequency modes (4096 Hz, 262144 Hz, 65536 Hz, 16384 Hz)
- Timer overflow handling with TMA reload and interrupt generation
- **Feature-complete emulator** ready for ROM testing
- VS Code tasks.json with 7 build/run/test configurations
- Comprehensive testing guide in refs/TESTING.md
- Clippy compliance (0 warnings) with idiomatic Rust patterns

### In Progress

- Boot ROM testing and verification
- Performance optimization## [0.1.0] - TBD

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
