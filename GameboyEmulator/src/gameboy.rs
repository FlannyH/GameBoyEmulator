#![allow(dead_code)]

mod gb_arithmetic_operations;
mod gb_branch_operations;
mod gb_cartridge;
mod gb_instruction_handling;
mod gb_io_registers;
mod gb_memory_operations;
mod gb_misc;
mod gb_opcodes_arithmetic;
mod gb_opcodes_branch;
mod gb_opcodes_ld;
mod gb_opcodes_prefixed;
mod gb_prefixed_operations;

pub enum FlagMask {
    ZERO = 0x80,
    NEG = 0x40,
    HALF = 0x20,
    CARRY = 0x10,
}

pub struct GameBoy {
    // Memory Map
    pub(self) bios: [u8; 0x100],
    pub(self) rom: Vec<u8>,
    pub(self) vram: [u8; 0x2000],
    pub(self) wram: [u8; 0x2000],
    pub(self) oam: [u8; 0xA0],
    pub(self) io: [u8; 0x80],
    pub(self) hram: [u8; 0x7F],
    pub(self) ie: u8,
    pub(self) ime: u8,
    pub(self) pc: u16,
    pub(self) sp: u16,

    // Registers
    pub(self) reg_a: u8,
    pub(self) reg_f: u8,
    pub(self) reg_b: u8,
    pub(self) reg_c: u8,
    pub(self) reg_d: u8,
    pub(self) reg_e: u8,
    pub(self) reg_h: u8,
    pub(self) reg_l: u8,

    // Misc emulation
    pub(self) curr_cycles_to_wait: u32,
    pub(self) rom_chip_enabled: bool,
}
