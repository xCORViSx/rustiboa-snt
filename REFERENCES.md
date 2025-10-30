<!--REMINDER: Read AGENTS.md file before continuing development-->

# REFERENCES

## Pan Docs [https://gbdev.io/pandocs/]

- Fetched on [10/29/25] for: Comprehensive Game Boy hardware reference covering CPU opcodes, memory map, PPU operation, registers, timing, and all hardware specifications needed for accurate emulation

## Writing an Emulator: The First Pixel [https://blog.tigris.fr/2019/09/15/writing-an-emulator-the-first-pixel/]

- Fetched on [10/29/25] for: Detailed explanation of PPU implementation including state machine design (OAM search, pixel transfer, HBlank, VBlank), pixel fetcher architecture with FIFO, tile map reading, timing synchronization between CPU and PPU, and practical implementation guidance

## GameBoy Emulation in JavaScript: The CPU [https://imrannazar.com/GameBoy-Emulation-in-JavaScript:-The-CPU]

- Fetched on [10/29/25] for: CPU architecture overview covering Sharp LR35902 registers (A, B, C, D, E, F, H, L, PC, SP), flags register operation (Zero, Operation, Half-carry, Carry), instruction dispatch loop design, fetch-decode-execute cycle, and memory interface patterns

## DMG-01: How to Emulate a Game Boy [https://rylev.github.io/DMG-01/public/book/]

- Fetched on [10/29/25] for: Rust-specific emulator development guide providing project structure, implementation approach in Rust, and beginner-friendly explanations of Game Boy architecture suitable for this Rust implementation

## GameBoy Emulator Development Guide [https://emudev.de/gameboy-emulator/overview/]

- Fetched on [10/29/25] for: Overview of emulator development process, component breakdown (CPU, PPU, MMU, SPU/APU), system specifications (8KB RAM, 8KB VRAM, 4.194304 MHz clock), and practical development roadmap for building a complete emulator

## Pastraiser Game Boy Opcode Table [https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html]

- Fetched on [10/29/25] for: Complete LR35902 opcode reference table with all 256 base opcodes and 256 CB-prefixed opcodes, including instruction mnemonics, byte lengths, cycle timings, and flag effects for implementing the full CPU instruction set

## izik1's gbops - Accurate Game Boy Opcode Table [https://izik1.github.io/gbops/index.html]

- Fetched on [10/29/25] for: Highly accurate opcode reference with precise timing in T-cycles, conditional branch timings, and verified opcode behavior for cycle-accurate CPU emulation implementation

## Pan Docs - Rendering Overview [https://gbdev.io/pandocs/Rendering.html]

- Fetched on [10/29/25] for: Detailed PPU rendering process including mode timing (Mode 0 HBlank, Mode 1 VBlank, Mode 2 OAM scan, Mode 3 pixel transfer), scanline structure (154 scanlines total, 144 visible), dot timing (4.194 MHz, 456 dots per scanline), Mode 3 penalties from scrolling/window/sprites, and frame timing (70224 dots at 59.7 fps)

## Pan Docs - VRAM Tile Data [https://gbdev.io/pandocs/Tile_Data.html]

- Fetched on [10/29/25] for: Tile data format and storage details including 8x8 pixel tiles with 2-bit color depth (16 bytes per tile), VRAM layout at $8000-$97FF (384 tiles DMG, 768 tiles CGB), tile addressing methods ($8000 unsigned method vs $8800 signed method), 2-byte line format (LSB and MSB for pixel color IDs 0-3), and palette index translation

## Pan Docs - LCD Status Registers [https://gbdev.io/pandocs/STAT.html]

- Fetched on [10/29/25] for: LCD status register (STAT) implementation including LY register (current scanline 0-153), LYC register (scanline compare for interrupts), PPU mode flags in STAT, STAT interrupt conditions (LYC==LY, Mode 0/1/2), dot timing definition (1 T-cycle DMG, 2 T-cycles CGB double speed), and hardware quirks like spurious STAT interrupts

