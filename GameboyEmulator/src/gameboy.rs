#![allow(dead_code)]

mod gb_instruction_handling;
mod gb_arithmetic_operations;
mod gb_memory_operations;
mod gb_misc;

pub enum FlagMask
{
    ZERO = 0x80,
    NEG = 0x40,
    HALF = 0x20,
    CARRY = 0x10,
}

pub struct GameBoy {
    // Memory Map
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
}
