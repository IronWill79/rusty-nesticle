use crate::{
    gpu::{GPU, VRAM_BEGIN, VRAM_END},
    memory::Memory,
};

#[derive(Debug)]
pub struct Bus {
    pub memory: [u8; 0xffff],
    pub gpu: GPU,
}

impl Default for Bus {
    fn default() -> Self {
        Self {
            memory: [0; 0xffff],
            gpu: GPU::default(),
        }
    }
}

impl Memory for Bus {
    fn read_byte(&self, address: u16) -> u8 {
        if address >= VRAM_BEGIN as u16 || address < VRAM_END as u16 {
            self.gpu.read_byte(address)
        } else {
            self.memory[address as usize]
        }
    }

    fn write_byte(&mut self, address: u16, byte: u8) {
        todo!()
    }
}
