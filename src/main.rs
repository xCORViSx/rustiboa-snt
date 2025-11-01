// REMINDER: Read AGENTS.md file before continuing development
//
// Rustiboa-SNT - A DMG (original Game Boy) emulator
//
// This is the main entry point for the emulator. We create and initialize all
// major components (CPU, MMU, PPU, display, input), load the ROM or boot ROM,
// and then enter the main emulation loop where we run the CPU and PPU in sync.

// Allow dead code during development as we're building the framework
#![allow(dead_code)]

mod cpu;
mod mmu;
mod ppu;
mod display;
mod cartridge;
mod input;
mod interrupts;
mod timer;

use std::env;
use std::process;
use std::fs::File;
use std::io::Write;

use cpu::Cpu;
use mmu::Mmu;
use ppu::Ppu;
use display::Display;
use input::Input;
use cartridge::Cartridge;
use timer::Timer;

fn main() {
    // We parse command line arguments to get the ROM file path and optional log file
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <rom-file.gb> [--log <logfile>]", args[0]);
        eprintln!("\nRustiboa-SNT - A DMG (original Game Boy) emulator");
        eprintln!("Provide a .gb ROM file to run");
        eprintln!("Optional: --log <logfile> to enable CPU state logging for Gameboy Doctor");
        process::exit(1);
    }
    
    let rom_path = &args[1];
    
    // Check for --log flag to enable CPU state logging for Gameboy Doctor
    let mut log_file: Option<File> = None;
    if args.len() >= 4 && args[2] == "--log" {
        match File::create(&args[3]) {
            Ok(file) => {
                log_file = Some(file);
                eprintln!("CPU logging enabled: {}", args[3]);
            }
            Err(e) => {
                eprintln!("Failed to create log file: {}", e);
                process::exit(1);
            }
        }
    }
    
    println!("Rustiboa-SNT - Game Boy Emulator");
    println!("Loading ROM: {}", rom_path);
    
    // We load the cartridge ROM from the file
    let cartridge = match Cartridge::load(rom_path) {
        Ok(cart) => cart,
        Err(e) => {
            eprintln!("Failed to load ROM: {}", e);
            process::exit(1);
        }
    };
    
    println!("Cartridge loaded: {}", cartridge.title);
    println!("ROM size: {} bytes", cartridge.rom.len());
    
    // We initialize all emulator components
    let mut mmu = Mmu::new(cartridge.rom.clone());
    let mut cpu = Cpu::new();
    let mut ppu = Ppu::new();
    let mut input = Input::new();
    let mut timer = Timer::new();
    
    // For Gameboy Doctor compatibility: initialize CPU state as if boot ROM finished
    if log_file.is_some() {
        mmu.doctor_mode = true;  // Enable special LY register handling
        cpu.registers.a = 0x01;
        cpu.registers.f = 0xB0;
        cpu.registers.b = 0x00;
        cpu.registers.c = 0x13;
        cpu.registers.d = 0x00;
        cpu.registers.e = 0xD8;
        cpu.registers.h = 0x01;
        cpu.registers.l = 0x4D;
        cpu.registers.sp = 0xFFFE;
        cpu.registers.pc = 0x0100;
    }
    
    // We initialize SDL2 for display and input handling
    let sdl = sdl2::init().unwrap();
    let mut display = Display::new(&sdl).expect("Failed to create display");
    let mut event_pump = sdl.event_pump().unwrap();
    
    println!("Emulator initialized!");
    println!("Controls: Arrow keys = D-pad, Z = A, X = B, Enter = Start, Shift = Select");
    
    let mut vram_write_count = 0u64;
    let start_time = std::time::Instant::now();
    let mut last_pc = 0u16;
    let mut pc_stuck_count = 0u32;
    
    // Main emulation loop: we run CPU cycles and PPU in sync
    'running: loop {
        // Handle input events
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                Event::Quit {..} => break 'running,
                Event::KeyDown { keycode: Some(key), .. } => {
                    input.key_down(key);
                }
                Event::KeyUp { keycode: Some(key), .. } => {
                    input.key_up(key);
                }
                _ => {}
            }
        }
        
        // Log CPU state for Gameboy Doctor (before executing next instruction)
        // Format: A:00 F:11 B:22 C:33 D:44 E:55 H:66 L:77 SP:8888 PC:9999 PCMEM:AA,BB,CC,DD
        if let Some(ref mut file) = log_file {
            if !cpu.halted {
                let pc = cpu.registers.pc;
                let pcmem0 = mmu.read_byte(pc);
                let pcmem1 = mmu.read_byte(pc.wrapping_add(1));
                let pcmem2 = mmu.read_byte(pc.wrapping_add(2));
                let pcmem3 = mmu.read_byte(pc.wrapping_add(3));
                
                writeln!(file, "A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}",
                    cpu.registers.a, cpu.registers.f,
                    cpu.registers.b, cpu.registers.c,
                    cpu.registers.d, cpu.registers.e,
                    cpu.registers.h, cpu.registers.l,
                    cpu.registers.sp, pc,
                    pcmem0, pcmem1, pcmem2, pcmem3
                ).unwrap();
            }
        }
        
        // Track if PC is stuck in a loop
        let current_pc = cpu.registers.pc;
        if current_pc == last_pc {
            pc_stuck_count += 1;
            if pc_stuck_count % 1000000 == 0 {
                eprintln!("Warning: PC stuck at 0x{:04X} for {} iterations", current_pc, pc_stuck_count);
            }
        } else {
            if pc_stuck_count > 10000 {
                eprintln!("PC was stuck at 0x{:04X} for {} iterations, now at 0x{:04X}", last_pc, pc_stuck_count, current_pc);
            }
            pc_stuck_count = 0;
            last_pc = current_pc;
        }
        
        // Run one CPU instruction (this returns M-cycles used)
        let m_cycles = cpu.tick(&mut mmu);
        
        // Check and handle any pending interrupts AFTER instruction execution
        // This ensures instructions that modify IF get their interrupts serviced immediately
        let int_cycles = interrupts::handle_interrupts(&mut cpu, &mut mmu);
        let total_cycles = m_cycles + int_cycles;
        
        // Update timer based on cycles executed
        timer.tick(total_cycles, &mut mmu);
        
        // Run OAM DMA for each M-cycle if active
        for _ in 0..total_cycles {
            mmu.tick_dma();
        }
        
        // Run PPU for corresponding T-cycles (4 T-cycles = 1 M-cycle)
        // Each M-cycle from CPU = 4 PPU dots
        for _ in 0..(total_cycles * 4) {
            let frame_ready = ppu.tick(&mut mmu);
            
            // When a frame is complete, we render it to the screen
            if frame_ready {
                // Check VRAM and framebuffer content
                vram_write_count += 1;
                
                // Print serial output if any (Blargg test results)
                if !mmu.serial_output.is_empty() {
                    println!("{}", mmu.serial_output);
                    // Clear to avoid reprinting
                    mmu.serial_output.clear();
                }
                
                // if vram_write_count <= 10 || vram_write_count % 60 == 0 {
                //     let elapsed = start_time.elapsed().as_secs_f32();
                //     let vram_has_data = mmu.read_byte(0x8000) != 0 || mmu.read_byte(0x9800) != 0;
                //     let fb_has_data = ppu.framebuffer.iter().any(|&p| p != 0);
                //     // Check tile 0x7F data (at 0x87F0)
                //     let tile_7f_data = mmu.read_byte(0x87F0);
                //     eprintln!("[{:.1}s] Frame {}, VRAM[0x8000]={:02X}, VRAM[0x9800]={:02X}, Tile 0x7F={:02X}, FB has data: {}", 
                //              elapsed, vram_write_count, mmu.read_byte(0x8000), mmu.read_byte(0x9800), tile_7f_data, fb_has_data);
                // }
                if let Err(e) = display.render(&ppu.framebuffer) {
                    eprintln!("Render error: {}", e);
                }
            }
        }
        
        // Update joypad state in MMU (write to 0xFF00 register)
        let joypad_state = input.read_joypad();
        mmu.write_byte(0xFF00, joypad_state);
        
        // Small delay to prevent running at unlimited speed (temporary)
        // TODO: Implement proper frame timing with VSync
        std::thread::sleep(std::time::Duration::from_micros(1));
    }
    
    println!("\nEmulator stopped");
}

