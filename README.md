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
  - **✅ Passes all 3 Blargg CPU test ROMs (1.26M+ instructions verified)**
- **PPU**: Complete pixel fetcher with FIFO and background tile rendering
  - 4-state machine (OAM Search, Pixel Transfer, HBlank, VBlank)
  - Tile map reading from VRAM
  - Scroll support (SCX, SCY)
  - Background palette (BGP) support
  - STAT register with PPU mode reporting
  - 160x144 resolution at 59.7 FPS
- **Memory**: Complete memory map including boot ROM, cartridge, VRAM, and I/O registers
  - MBC1 cartridge support (ROM/RAM banking, mode selection)
  - OAM DMA transfer (0xFF46 register)
- **Display**: SDL2-based rendering with authentic Game Boy color palette
- **Input**: Joypad support (D-pad, A, B, Start, Select)
- **Interrupts**: Full interrupt system with priority handling
  - VBlank, LCD STAT, Timer, Serial, and Joypad interrupts
  - IE/IF register support
  - Automatic handler dispatch
  - STAT interrupts for PPU mode changes (HBlank, VBlank, OAM, LY=LYC)
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

## Current Status

**Working:**

- ✅ **All 11 Blargg CPU instruction tests pass**:
  - 01-special.gb: 1,256,633 instructions verified against Gameboy Doctor
  - 02-interrupts.gb, 03-op sp,hl.gb, 04-op r,imm.gb, 05-op rp.gb
  - 06-ld r,r.gb, 07-jr,jp,call,ret,rst.gb, 08-misc instrs.gb
  - 09-op r,r.gb, 10-bit ops.gb, 11-op a,(hl).gb
- ✅ **Additional passing tests**:
  - halt_bug.gb: Displays test content correctly
  - instr_timing.gb: Instruction timing accurate
- ✅ CPU instruction accuracy verified with systematic debugging
- ✅ MBC1 memory bank controller implemented
- ✅ PPU rendering pipeline functional
- ✅ Timer system working correctly
- ✅ Interrupt timing accurate

**Recent Bug Fixes:**

1. **CB Register Encoding**: Fixed incorrect register mapping in CB-prefixed instructions (rotate, shift, BIT, RES, SET). CB instructions use different encoding than standard instructions.
2. **Interrupt Timing**: Moved interrupt check to after instruction execution, ensuring interrupts triggered during instructions are serviced immediately.
3. **Timer Frequencies**: Corrected timer frequencies from T-cycles to M-cycles for accurate timing.

**Known Issues:**

- interrupt_time.gb: Hangs at PC 0xC9C9 (timing precision issue)
- mem_timing.gb: Fails 3 tests (memory access timing not cycle-accurate)
- Some commercial ROMs may have compatibility issues

- Some commercial ROMs may still have compatibility issues
- Boot ROM not yet implemented
- APU (audio) not yet implemented

**Next Steps:**

- Test additional commercial ROMs with verified CPU accuracy
- Implement boot ROM support
- Add sprite rendering
- Optimize performance

## Current Limitations

- Limited MBC support (MBC1 only, others in progress)
- No audio (APU) implementation
- No save game support
- No Game Boy Color support
- Timing accuracy may not be sufficient for all commercial games

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
