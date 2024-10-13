use crate::memory::Memory;

const VRAM_SIZE: usize = 0x3fff;
pub const VRAM_BEGIN: usize = 0x8000;
pub const VRAM_END: usize = 0xffff;
pub const NUMBER_OF_PIXELS: usize = 320 * 240;
pub const ONE_FRAME_IN_CYCLES: usize = 320;

#[derive(Clone, Copy, Debug, Default)]
struct Tile {}

#[derive(Debug)]
pub struct GPU {
    pub tile_set: [Tile; 384],
    pub video_ram: [u8; VRAM_SIZE],
    pub canvas_buffer: [u8; NUMBER_OF_PIXELS],
}

impl Default for GPU {
    fn default() -> Self {
        Self {
            tile_set: [Tile::default(); 384],
            video_ram: [0x00; VRAM_SIZE],
            canvas_buffer: [0x00; NUMBER_OF_PIXELS],
        }
    }
}

impl Memory for GPU {
    fn read_byte(&self, address: u16) -> u8 {
        todo!()
    }

    fn write_byte(&mut self, address: u16, byte: u8) {
        todo!()
    }
}
