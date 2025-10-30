<!--REMINDER: Read AGENTS.md file before continuing development-->

# Rustiboa-SNT

A DMG (original Game Boy) emulator written in Rust.

## Overview

Rustiboa-SNT is a Game Boy emulator implementing the Sharp LR35902 CPU (modified Z80) and associated hardware components including the PPU (Picture Processing Unit), memory management, timers, and input handling. The emulator aims for cycle-accurate emulation to properly run Game Boy games and the boot ROM.

## Features

**Core Implementation Complete:**

- **CPU**: Sharp LR35902 with complete instruction set (256 base + 256 CB-prefixed opcodes)
  - All load operations (8-bit/16-bit)
  - Arithmetic and logic operations with proper flag handling
  - Control flow instructions (jumps, calls, returns)
  - Rotate, shift, and bit manipulation operations
  - Cycle-accurate timing
- **PPU**: Complete pixel fetcher with FIFO and background tile rendering
  - 4-state machine (OAM Search, Pixel Transfer, HBlank, VBlank)
  - Tile map reading from VRAM
  - Scroll support (SCX, SCY)
  - Background palette (BGP) support
  - 160x144 resolution at 59.7 FPS
- **Memory**: Complete memory map including boot ROM, cartridge, VRAM, and I/O registers
- **Display**: SDL2-based rendering with authentic Game Boy color palette
- **Input**: Joypad support (D-pad, A, B, Start, Select)
- **Interrupts**: Full interrupt system with priority handling
  - VBlank, LCD STAT, Timer, Serial, and Joypad interrupts
  - IE/IF register support
  - Automatic handler dispatch
- **Timer**: Complete timer system
  - DIV register (16384 Hz)
  - TIMA/TMA/TAC registers
  - 4 programmable frequencies
  - Timer interrupt on overflow
- **Cartridge**: ROM loading with header parsing

## System Specifications

- **Main RAM**: 8KB
- **Video RAM**: 8KB
- **Resolution**: 160x144 pixels (20x18 tiles)
- **Max Sprites**: 40 total (10 per scanline)
- **Clock Speed**: 4.194304 MHz
- **CPU**: Sharp LR35902 (8-bit modified Z80)

## Building

Ensure you have Rust installed (edition 2024). You'll also need SDL2 development libraries:

**macOS:**

```bash
brew install sdl2
```

**Ubuntu/Debian:**

```bash
sudo apt-get install libsdl2-dev
```

**Windows:**

Follow the [SDL2 installation guide](https://github.com/Rust-SDL2/rust-sdl2#windows-msvc) for MSVC.

Build the project with:

```bash
cargo build --release
```

## Running

```bash
cargo run --release -- <path-to-rom.gb>
```

### Controls

- **D-Pad**: Arrow keys
- **A/B**: Z and X keys
- **Start/Select**: Enter and Shift keys

### Testing

To test the emulator, you'll need Game Boy ROM files:

1. **Boot ROM** (`dmg_boot.bin`, 256 bytes) - Tests basic CPU/PPU operation with Nintendo logo animation
2. **Test ROMs** - Homebrew test suites like Blargg's CPU tests or dmg-acid2
3. **Game ROMs** - Commercial games (use only legally owned ROMs)

See [TESTING.md](refs/TESTING.md) for detailed testing instructions.

### Build Tasks

VS Code tasks available (`Cmd+Shift+P` → Run Task):

- Build Release (default: `Cmd+Shift+B`)
- Build Debug
- Run with ROM
- Clean
- Check
- Clippy

## Current Limitations

- No MBC (Memory Bank Controller) support yet - only ROM-only cartridges work
- No audio (APU) implementation
- No save game support
- No Game Boy Color support

## Dependencies

- `sdl2`: Graphics and input handling
- Standard Rust libraries

## Project Structure

```text
rustiboa-snt/
├── src/
│   ├── main.rs           # Entry point
│   ├── cpu/              # CPU implementation
│   │   ├── mod.rs        # CPU state and execution
│   │   ├── registers.rs  # Register system and flags
│   │   ├── instructions.rs  # All 512 instruction implementations
│   │   └── opcodes.rs    # Opcode mapping (stub)
│   ├── mmu/              # Memory management
│   ├── ppu/              # Picture processing unit
│   ├── cartridge/        # ROM loading
│   ├── display/          # SDL2 rendering
│   └── input/            # Input handling
├── Cargo.toml
├── AGENTS.md             # AI agent instructions
├── REFERENCES.md         # Technical references
├── ROADMAP.md            # Development roadmap
└── CHANGELOG.md          # Version history
```

## References

See [REFERENCES.md](REFERENCES.md) for technical documentation and resources used in development.

## License

This project is open source. See LICENSE for details.
