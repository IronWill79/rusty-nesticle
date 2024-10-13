use crate::bus::Bus;
use crate::gpu::NUMBER_OF_PIXELS;
use crate::instruction_set::{ArithmeticTarget, Instruction};
use crate::memory::Memory;

#[derive(Debug, Default)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
}

#[derive(Debug, Default)]
pub struct CPU {
    pub bus: Bus,
    pub pc: u16,
    pub registers: Registers,
    pub sp: u8,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            bus: Bus::default(),
            pc: 0x0000,
            registers: Registers::default(),
            sp: 0x00,
        }
    }

    fn step(&mut self) {
        let mut byte = self.bus.read_byte(self.pc);
        let instruction = Instruction::from_byte(byte).expect("Invalid Instruction");
        let next_pc = self.execute(instruction);
        self.pc = next_pc;
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::ADD(ArithmeticTarget::C) => {
                let value = self.registers.c;
                let result = self.registers.a.wrapping_add(value);
                self.registers.a = result;
                self.pc + 1
            }
            _ => todo!(),
        }
    }

    pub fn run(&mut self, time_delta: u32) -> usize {
        todo!()
    }

    pub fn pixel_buffer(&mut self) -> Vec<u8> {
        todo!()
    }
}
