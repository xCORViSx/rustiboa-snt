// REMINDER: Read AGENTS.md file before continuing development
//
// Display Module - SDL2 rendering
//
// This module handles creating an SDL2 window and rendering the Game Boy's
// framebuffer to it. The Game Boy screen is 160x144 pixels with 4 shades of gray.

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;

const SCREEN_WIDTH: u32 = 160;
const SCREEN_HEIGHT: u32 = 144;
const SCALE_FACTOR: u32 = 4; // Scale up for visibility

/// Game Boy color palette (4 shades of gray/green)
const PALETTE: [u32; 4] = [
    0xE0F8D0, // Lightest (white/off-white)
    0x88C070, // Light gray/green
    0x346856, // Dark gray/green
    0x081820, // Darkest (black/dark blue)
];

/// This struct manages the SDL2 display system including the window,
/// canvas for drawing, and texture that holds the Game Boy's framebuffer
pub struct Display<'a> {
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    texture: Texture<'a>,
}

impl<'a> Display<'a> {
    /// This creates a new SDL2 window and initializes the rendering pipeline.
    /// The window is scaled up from 160x144 to make it more visible.
    pub fn new(sdl_context: &Sdl) -> Result<Self, String> {
        let video_subsystem = sdl_context.video()?;
        
        let window = video_subsystem
            .window(
                "Rustiboa-SNT - Game Boy Emulator",
                SCREEN_WIDTH * SCALE_FACTOR,
                SCREEN_HEIGHT * SCALE_FACTOR,
            )
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        
        let mut canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())?;
        
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0xE0, 0xF8, 0xD0));
        canvas.clear();
        canvas.present();
        
        let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();
        let texture = unsafe {
            // SAFETY: We're storing the texture_creator in the struct so the texture lifetime is valid
            std::mem::transmute::<Texture<'_>, Texture<'_>>(
                texture_creator
                    .create_texture_streaming(PixelFormatEnum::RGB24, SCREEN_WIDTH, SCREEN_HEIGHT)
                    .map_err(|e| e.to_string())?
            )
        };
        
        Ok(Display {
            canvas,
            texture_creator,
            texture,
        })
    }
    
    /// This renders the Game Boy's framebuffer to the SDL2 window.
    /// Each pixel in the framebuffer is a value 0-3 representing one of four gray shades.
    pub fn render(&mut self, framebuffer: &[u8; 160 * 144]) -> Result<(), String> {
        // We update the texture with pixel data from the framebuffer
        self.texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..SCREEN_HEIGHT as usize {
                for x in 0..SCREEN_WIDTH as usize {
                    let fb_index = y * SCREEN_WIDTH as usize + x;
                    let color_index = framebuffer[fb_index] & 0x03; // Mask to 0-3
                    let color = PALETTE[color_index as usize];
                    
                    let offset = y * pitch + x * 3;
                    buffer[offset] = ((color >> 16) & 0xFF) as u8;     // R
                    buffer[offset + 1] = ((color >> 8) & 0xFF) as u8;  // G
                    buffer[offset + 2] = (color & 0xFF) as u8;          // B
                }
            }
        })?;
        
        // We clear the canvas and draw the texture scaled up
        self.canvas.clear();
        self.canvas.copy(
            &self.texture,
            None,
            Some(Rect::new(
                0,
                0,
                SCREEN_WIDTH * SCALE_FACTOR,
                SCREEN_HEIGHT * SCALE_FACTOR,
            )),
        )?;
        self.canvas.present();
        
        Ok(())
    }
}
