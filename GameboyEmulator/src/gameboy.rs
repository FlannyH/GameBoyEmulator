#![allow(dead_code)]

use std::collections::VecDeque;

use rodio::Sink;

mod apu;
mod cpu;
mod misc;
mod ppu;

pub enum FlagMask {
    Zero = 0x80,
    Neg = 0x40,
    Half = 0x20,
    Carry = 0x10,
}

#[derive(Clone)]
pub struct PpuFifoElement {
    pub color: u8,  //0, 1, 2, 3
    pub source: u8, //0: bg, 1: sprite 1, 2: sprite 2
}

struct OamEntry {
    posy: u8,
    posx: u8,
    tile: u8,
    attr: u8,
}

struct AudioSource {}

#[derive(Default, Debug)]
pub struct InputState {
    pub down: bool,
    pub up: bool,
    pub left: bool,
    pub right: bool,
    pub start: bool,
    pub select: bool,
    pub b: bool,
    pub a: bool,
}

pub struct GameBoy {
    // Memory Map
    bios: [u8; 0x100],
    rom: Vec<u8>,
    eram: Vec<u8>,
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
    ppu_fifo: VecDeque<PpuFifoElement>,
    ppu_tilemap_x: u8, //0..=31
    ppu_tilemap_y: u8, //0..=31
    ppu_pixels_to_discard: u8,
    ppu_sprite_buffer: Vec<OamEntry>,
    framebuffer: Vec<u32>,

    // APU
    apu_stream: rodio::OutputStream,
    //apu_stream_handle: rodio::OutputStreamHandle,
    apu_buffer: [[u16; 512]; 2],
    apu_buffer_to_use: usize,
    apu_buffer_write_index: usize,
    apu_buffer_read_index: usize,
    pub apu_sink: Sink,
    apu_sound_output: [u8; 4],
    apu_pulse1_freq_counter: u16,
    apu_pulse1_env_counter: u8,
    apu_pulse1_duty_step: usize,
    apu_pulse1_length_timer: u8,
    apu_pulse1_enabled: bool,
    apu_pulse1_curr_volume: u8,

    apu_pulse2_freq_counter: u16,
    apu_pulse2_env_counter: u8,
    apu_pulse2_duty_step: usize,
    apu_pulse2_length_timer: u8,
    apu_pulse2_enabled: bool,
    apu_pulse2_curr_volume: u8,

    apu_wave_freq_counter: u16,
    apu_wave_env_counter: u8,
    apu_wave_duty_step: usize,
    apu_wave_length_timer: u8,
    apu_wave_enabled: bool,

    apu_noise_freq_counter: u16,
    apu_noise_env_counter: u8,
    apu_noise_duty_step: usize,
    apu_noise_length_timer: u8,
    apu_noise_enabled: bool,
    apu_noise_curr_volume: u8,

    apu_pulse1_sweep_timer: u8,
    apu_pulse1_sweep_shadow_freq: u16,
    apu_pulse1_sweep_enable: bool,
    apu_clock_timer: u32,
    apu_clock: u32,

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
    eram_chip_enabled: bool,
    curr_rom_bank: u8,
    curr_eram_bank: u8,
    cpu_cycle_counter: u32,
    is_halted: bool,
    timer_div: u16,
    timer_overflow: bool,
    oam_dma_counter: u8,
    oam_dma_source: u16,
    joypad_state: u8,
    window_is_rendering: bool,
    save_path: String,

    // Debug
    debug_enabled: bool,
    debug_bios: bool,
    debug_require_input: bool,
}

impl GameBoy {
    pub(crate) fn render_screen(
        &self,
        buffer: &mut Vec<u32>,
        offset_x: usize,
        offset_y: usize,
        scale: usize,
        width: usize,
    ) {
        // Render outline
        let end_x = offset_x + 160 * scale;
        let end_y = offset_y + 144 * scale;
        draw_line_border(buffer, offset_x, end_x, offset_y, end_y, width);

        for y in 0..144 * scale {
            for x in 0..160 * scale {
                buffer[(offset_x + x + 1) + (offset_y + y + 1) * width] =
                    self.framebuffer[(x / scale) + (y / scale) * 160];
            }
        }
    }

    pub(crate) fn render_palettes(
        &self,
        buffer: &mut [u32],
        offset_x: usize,
        offset_y: usize,
        scale: usize,
        width: usize,
    ) {
        // Get palettes from IO
        // i was really about to type `ldh a, [$ff47]` wow
        let bgp = [
            self.io[0x47] & 0x03,
            self.io[0x47] >> 2 & 0x03,
            self.io[0x47] >> 4 & 0x03,
            self.io[0x47] >> 6 & 0x03,
        ];
        let obp0 = [
            self.io[0x48] & 0x03,
            self.io[0x48] >> 2 & 0x03,
            self.io[0x48] >> 4 & 0x03,
            self.io[0x48] >> 6 & 0x03,
        ];
        let obp1 = [
            self.io[0x49] & 0x03,
            self.io[0x49] >> 2 & 0x03,
            self.io[0x49] >> 4 & 0x03,
            self.io[0x49] >> 6 & 0x03,
        ];
        let palettes = [bgp, obp0, obp1];

        // Draw the palettes
        for (y, palette) in palettes.iter().enumerate() {
            for x in 0..4 {
                draw_rectangle(
                    buffer,
                    (((palette[x] ^ 0b11) as u32) * (235 / 3)) * 0x00010101,
                    offset_x + (scale) * x,
                    offset_x + (scale) * (x + 1),
                    offset_y + (scale) * y,
                    offset_y + (scale) * (y + 1),
                    width,
                );
                draw_line_border(
                    buffer,
                    offset_x + (scale) * x,
                    offset_x + (scale) * (x + 1),
                    offset_y + (scale) * y,
                    offset_y + (scale) * (y + 1),
                    width,
                );
            }
        }
    }

    pub(crate) fn save_game_if_possible(&self) {
        if !self.eram.is_empty() {
            std::fs::write(self.save_path.as_str(), self.eram.clone())
                .expect("Couldn't write save file");
        }
    }
}

fn draw_rectangle(
    buffer: &mut [u32],
    color: u32,
    x_1: usize,
    x_2: usize,
    y_1: usize,
    y_2: usize,
    width: usize,
) {
    for y in y_1..=y_2 {
        for x in x_1..=x_2 {
            buffer[x + y * width] = color;
        }
    }
}

fn draw_line_border(
    buffer: &mut [u32],
    x_1: usize,
    x_2: usize,
    y_1: usize,
    y_2: usize,
    buffer_width: usize,
) {
    for x in (x_1.wrapping_sub(1))..=(x_2 + 1) {
        buffer[x + (y_1 - 1) * buffer_width] = 0xFFFF00FF;
        buffer[x + (y_2 + 1) * buffer_width] = 0xFFFF00FF;
    }
    for y in (y_1.wrapping_sub(1))..=(y_2 + 1) {
        buffer[(x_1 - 1) + y * buffer_width] = 0xFFFF00FF;
        buffer[(x_2 + 1) + y * buffer_width] = 0xFFFF00FF;
    }
}
