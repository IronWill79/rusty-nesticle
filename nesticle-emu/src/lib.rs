mod bus;
mod cpu;
mod gpu;
mod instruction_set;
mod memory;

#[cfg(test)]
mod tests {
    use crate::cpu::CPU;

    #[test]
    fn can_create_cpu() {
        let cpu = CPU::new();
        assert_eq!(cpu.pc, 0x0000);
    }
}
