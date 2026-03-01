//! COR24 CPU state

use serde::{Deserialize, Serialize};

/// Memory size: 64KB for emulation (addresses 0x000000-0x00FFFF)
pub const MEMORY_SIZE: usize = 65536;

/// Default reset address (embedded block RAM start)
pub const RESET_ADDRESS: u32 = 0x000000;

/// Stack pointer initial value
pub const INITIAL_SP: u32 = 0x00FC00;

/// COR24 CPU state
#[derive(Clone, Serialize, Deserialize)]
pub struct CpuState {
    /// Program counter (24-bit)
    pub pc: u32,
    /// Register file (8 x 24-bit registers)
    pub registers: [u32; 8],
    /// Condition flag
    pub c: bool,
    /// Memory (byte-addressable)
    pub memory: Vec<u8>,
    /// Halted flag
    pub halted: bool,
    /// Cycle count
    pub cycles: u64,
    /// Instruction count
    pub instructions: u64,
}

impl Default for CpuState {
    fn default() -> Self {
        Self::new()
    }
}

impl CpuState {
    /// Create a new CPU state with default values
    pub fn new() -> Self {
        let mut state = Self {
            pc: RESET_ADDRESS,
            registers: [0; 8],
            c: false,
            memory: vec![0; MEMORY_SIZE],
            halted: false,
            cycles: 0,
            instructions: 0,
        };
        // Initialize stack pointer
        state.registers[4] = INITIAL_SP;
        state
    }

    /// Reset CPU to initial state (preserves memory)
    pub fn reset(&mut self) {
        self.pc = RESET_ADDRESS;
        self.registers = [0; 8];
        self.registers[4] = INITIAL_SP;
        self.c = false;
        self.halted = false;
        self.cycles = 0;
        self.instructions = 0;
    }

    /// Hard reset (clears memory too)
    pub fn hard_reset(&mut self) {
        self.reset();
        self.memory.fill(0);
    }

    /// Read a byte from memory
    pub fn read_byte(&self, addr: u32) -> u8 {
        let addr = (addr as usize) % MEMORY_SIZE;
        self.memory[addr]
    }

    /// Write a byte to memory
    pub fn write_byte(&mut self, addr: u32, value: u8) {
        let addr = (addr as usize) % MEMORY_SIZE;
        self.memory[addr] = value;
    }

    /// Read a 24-bit word from memory (little-endian)
    pub fn read_word(&self, addr: u32) -> u32 {
        let b0 = self.read_byte(addr) as u32;
        let b1 = self.read_byte(addr.wrapping_add(1)) as u32;
        let b2 = self.read_byte(addr.wrapping_add(2)) as u32;
        b0 | (b1 << 8) | (b2 << 16)
    }

    /// Write a 24-bit word to memory (little-endian)
    pub fn write_word(&mut self, addr: u32, value: u32) {
        self.write_byte(addr, (value & 0xFF) as u8);
        self.write_byte(addr.wrapping_add(1), ((value >> 8) & 0xFF) as u8);
        self.write_byte(addr.wrapping_add(2), ((value >> 16) & 0xFF) as u8);
    }

    /// Get register value (masked to 24 bits)
    pub fn get_reg(&self, reg: u8) -> u32 {
        self.registers[(reg & 0x07) as usize] & 0xFFFFFF
    }

    /// Set register value (masked to 24 bits)
    pub fn set_reg(&mut self, reg: u8, value: u32) {
        self.registers[(reg & 0x07) as usize] = value & 0xFFFFFF;
    }

    /// Sign extend 8-bit to 24-bit
    pub fn sign_extend_8(value: u8) -> u32 {
        if value & 0x80 != 0 {
            0xFFFF00 | (value as u32)
        } else {
            value as u32
        }
    }

    /// Sign extend 24-bit result
    pub fn mask_24(value: u32) -> u32 {
        value & 0xFFFFFF
    }

    /// Load program into memory at given address
    pub fn load_program(&mut self, start_addr: u32, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            self.write_byte(start_addr + i as u32, byte);
        }
    }
}

/// Instruction decode ROM
/// Maps 8-bit instruction bytes to 12-bit decoded values: [opcode(5):ra(3):rb(3)]
/// Uses the const DECODE_ROM extracted from dis_rom.v
#[derive(Clone)]
pub struct DecodeRom;

impl Default for DecodeRom {
    fn default() -> Self {
        Self::new()
    }
}

impl DecodeRom {
    /// Create decode ROM (uses static const array)
    pub fn new() -> Self {
        Self
    }

    /// Decode an instruction byte
    pub fn decode(&self, byte: u8) -> u16 {
        crate::cpu::decode_rom::DECODE_ROM[byte as usize]
    }

    /// Check if an instruction byte is valid
    pub fn is_valid(&self, byte: u8) -> bool {
        crate::cpu::decode_rom::DECODE_ROM[byte as usize] != 0xFFF
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_state_new() {
        let cpu = CpuState::new();
        assert_eq!(cpu.pc, RESET_ADDRESS);
        assert_eq!(cpu.registers[4], INITIAL_SP);
        assert!(!cpu.halted);
    }

    #[test]
    fn test_memory_operations() {
        let mut cpu = CpuState::new();

        cpu.write_byte(0x100, 0x42);
        assert_eq!(cpu.read_byte(0x100), 0x42);

        cpu.write_word(0x200, 0x123456);
        assert_eq!(cpu.read_word(0x200), 0x123456);
    }

    #[test]
    fn test_sign_extend() {
        assert_eq!(CpuState::sign_extend_8(0x7F), 0x00007F);
        assert_eq!(CpuState::sign_extend_8(0x80), 0xFFFF80);
        assert_eq!(CpuState::sign_extend_8(0xFF), 0xFFFFFF);
    }
}
