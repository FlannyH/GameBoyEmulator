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
    pub(self) bios: [u8; 0x100],
    pub(self) rom: Vec<u8>,
    pub(self) vram: [u8; 0x2000],
    pub(self) wram: [u8; 0x2000],
    pub(self) oam: [u8; 0xA0],
    pub(self) io: [u8; 0x80],
    pub(self) hram: [u8; 0x7F],

    // State
    pub(self) ie: u8,
    pub(self) ime: u8,
    pub(self) pc: u16,
    pub(self) sp: u16,

    // PPU
    pub(self) ppu_lx: u8,
    pub(self) ppu_ly: u8,
    pub(self) ppu_mode: u8,
    pub(self) ppu_dots_into_curr_mode: u16,
    pub(self) ppu_dots_into_curr_line: u16,
    pub(self) ppu_fifo: Queue<PpuFifoElement>,
    pub(self) ppu_tilemap_x: u8, //0..=31
    pub(self) ppu_tilemap_y: u8, //0..=31
    pub(self) ppu_pixels_to_discard: u8,
    pub(self) framebuffer: Vec<u32>,

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
    pub(in crate) times: [u8; 256],
    pub(self) curr_cycles_to_wait: u32,
    pub(self) last_opcode: u8,
    pub(self) rom_chip_enabled: bool,
    pub(self) curr_rom_bank: u8,
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
