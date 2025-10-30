# Testing Guide

This document provides instructions for testing the Rustiboa-SNT emulator.

## Quick Start

### Running the Emulator

```bash
cargo build --release
./target/release/rustiboa-snt <path-to-rom.gb>
```

Or use VS Code tasks:
- `Cmd+Shift+B` → Build Release
- `Cmd+Shift+P` → Run Task → "Run with ROM"

### Controls

- **D-Pad**: Arrow keys (↑ ↓ ← →)
- **A Button**: Z key
- **B Button**: X key
- **Start**: Enter/Return
- **Select**: Shift

## Testing with Boot ROM

The DMG boot ROM is the official Game Boy startup sequence that displays the scrolling Nintendo logo.

### Obtaining the Boot ROM

The DMG boot ROM (`dmg_boot.bin`, 256 bytes) can be:
1. Extracted from a real Game Boy using hardware tools
2. Found online (search "dmg_boot.bin" - ensure it's the authentic version)

**Note**: The boot ROM is copyrighted by Nintendo. Use responsibly.

### Expected Behavior

When running with the boot ROM:
1. Screen starts blank
2. Nintendo logo scrolls down from top
3. Logo bounces and locks into place
4. "ding" sound effect (when audio is implemented)
5. Logo fades and game starts (if cartridge ROM is also loaded)

### Running Boot ROM Only

```bash
./target/release/rustiboa-snt dmg_boot.bin
```

The emulator should display the Nintendo logo animation. This verifies:
- ✓ CPU instruction execution
- ✓ Memory management
- ✓ PPU rendering
- ✓ Timer operation
- ✓ Interrupt handling

## Testing with Commercial ROMs

### Public Domain Test ROMs

Test with homebrew and public domain ROMs first:
- **Blargg's Test ROMs**: CPU instruction tests
- **dmg-acid2**: PPU rendering test
- **Mooneye GB**: Comprehensive hardware tests

Available at: https://github.com/retrio/gb-test-roms

### Running Commercial Games

```bash
./target/release/rustiboa-snt game.gb
```

**Note**: Only use ROMs you legally own. Commercial ROMs are copyrighted.

### Known Limitations

Current implementation supports:
- ✓ ROM-only cartridges
- ✓ Basic memory mapping
- ✗ MBC (Memory Bank Controllers) - not yet implemented
- ✗ Save games - not yet implemented
- ✗ Audio (APU) - not yet implemented

## Troubleshooting

### Emulator doesn't start
- Check ROM file exists and is readable
- Verify ROM file is valid .gb format
- Try with a different ROM

### Display issues
- Ensure SDL2 is properly installed
- Check `target/Frameworks/libSDL2-2.0.0.dylib` exists
- Try rebuilding: `cargo clean && cargo build --release`

### Performance issues
- Use release build (not debug): `--release` flag
- Close other applications
- Check CPU usage is reasonable

### Graphical glitches
- Some games may use MBC or advanced features not yet implemented
- Try simpler ROMs first (boot ROM, test ROMs)
- Check REFERENCES.md for PPU implementation details

## Debugging

### Enable verbose logging
Add debug prints in `src/main.rs`:
```rust
println!("PC: {:04X}, Opcode: {:02X}", cpu.registers.pc, opcode);
```

### Check CPU state
Print register values in the main loop to debug execution.

### Verify memory
Check MMU reads/writes are hitting correct addresses.

## Performance Metrics

Expected performance:
- **Frame Rate**: ~60 FPS (59.7 FPS accurate)
- **CPU Usage**: <10% on modern hardware
- **Memory**: <50 MB RAM usage

## Next Steps After Testing

1. **MBC Support**: Implement MBC1 for larger games
2. **Audio**: Add APU (sound) implementation
3. **Save States**: Add state save/load functionality
4. **Game Boy Color**: Extend to support CGB mode
5. **Debugger**: Add step-through debugging UI

## Resources

- Pan Docs: https://gbdev.io/pandocs/
- Game Boy test ROMs: https://github.com/retrio/gb-test-roms
- GB Dev Community: https://gbdev.io/

## Reporting Issues

If you encounter bugs:
1. Note the ROM being used
2. Describe expected vs actual behavior
3. Include any error messages
4. Check ROADMAP.md for known limitations
