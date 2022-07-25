#![allow(dead_code)]

use queues::Queue;

use crate::WIDTH;

mod cpu;
mod misc;
mod ppu;

pub enum FlagMask {
    ZERO = 0x80,
    NEG = 0x40,
    HALF = 0x20,
    CARRY = 0x10,
}

#[derive(Clone)]
pub struct PpuFifoElement {
    pub color: u8,  //0, 1, 2, 3
    pub source: u8, //0: bg, 1: sprite 1, 2: sprite 2
}

pub struct GameBoy {
    // Memory Map
    bios: [u8; 0x100],
    rom: Vec<u8>,
    vram: [u8; 0x2000],
    wram: [u8; 0x2000],
    oam: [u8; 0xA0],
    io: [u8; 0x80],
    hram: [u8; 0x7F],

    // State
    ie: u8,
    ime: u8,
    pc: u16,
    sp: u16,

    // PPU
    ppu_lx: u8,
    ppu_ly: u8,
    ppu_mode: u8,
    ppu_dots_into_curr_mode: u16,
    ppu_dots_into_curr_line: u16,
    ppu_fifo: Queue<PpuFifoElement>,
    ppu_tilemap_x: u8, //0..=31
    ppu_tilemap_y: u8, //0..=31
    ppu_pixels_to_discard: u8,
    framebuffer: Vec<u32>,

    // Registers
    reg_a: u8,
    reg_f: u8,
    reg_b: u8,
    reg_c: u8,
    reg_d: u8,
    reg_e: u8,
    reg_h: u8,
    reg_l: u8,

    // Misc emulation
    pub times: [u8; 256],
    curr_cycles_to_wait: u32,
    last_opcode: u8,
    last_opcode_cycles: u32,
    new_instruction_tick: bool,
    rom_chip_enabled: bool,
    curr_rom_bank: u8,
    cpu_cycle_counter: u32,
    is_halted: bool,

    // Debug
    DEBUG_ENABLED: bool,
    DEBUG_BIOS: bool,
    DEBUG_REQUIRE_INPUT: bool,
}

impl GameBoy {
    pub(crate) fn render_screen(&self, buffer: &mut Vec<u32>, offset_x: usize, offset_y: usize) {
        // Render outline
        let end_x = offset_x + 160 * 2;
        let end_y = offset_y + 144 * 2;
        for x in (offset_x - 1)..=(end_x + 1) {
            buffer[x + (offset_y - 1) * WIDTH] = 0xFFFF00FF;
            buffer[x + (end_y + 1) * WIDTH] = 0xFFFF00FF;
        }
        for y in (offset_y - 1)..=(end_y + 1) {
            buffer[(offset_x - 1) + y * WIDTH] = 0xFFFF00FF;
            buffer[(end_x + 1) + y * WIDTH] = 0xFFFF00FF;
        }

        for y in 0..144 * 2 {
            for x in 0..160 * 2 {
                buffer[(offset_x + x + 1) + (offset_y + y + 1) * WIDTH] =
                    self.framebuffer[(x / 2) + (y / 2) * 160];
            }
        }
    }
}
