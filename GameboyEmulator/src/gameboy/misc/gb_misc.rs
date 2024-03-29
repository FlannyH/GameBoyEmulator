use std::collections::VecDeque;
use std::{env, fs};

use crate::DEBUG_WIDTH;
use rand::Rng;
use rodio::{OutputStream, Sink};

use super::super::GameBoy;

impl GameBoy {
    pub(crate) fn new() -> GameBoy {
        // Create the Game Boy object
        let (stream, stream_device) = OutputStream::try_default().unwrap();
        let mut new_game_boy = GameBoy {
            bios: [0xFF; 0x100],
            rom: Vec::new(),
            eram: Vec::new(),
            vram: [0xFF; 0x2000],
            wram: [0xFF; 0x2000],
            oam: [0xFF; 0xA0],
            io: [0xFF; 0x80],
            hram: [0xFF; 0x7F],
            ie: 0,
            ime: 0,
            pc: 0,
            sp: 0,
            ppu_lx: 0,
            ppu_ly: 0,
            ppu_mode: 1,
            ppu_dots_into_curr_mode: 0,
            ppu_dots_into_curr_line: 0,
            ppu_fifo: VecDeque::new(),
            ppu_tilemap_x: 0,
            ppu_tilemap_y: 0,
            ppu_pixels_to_discard: 0,
            ppu_sprite_buffer: Vec::new(),
            framebuffer: vec![0; 160 * 144],
            apu_stream: stream,
            //apu_stream_handle: stream_device,
            apu_buffer: [[0; 512]; 2],
            apu_buffer_to_use: 0,
            apu_buffer_write_index: 0,
            apu_buffer_read_index: 0,
            apu_sound_output: [0, 0, 0, 0],
            apu_pulse1_freq_counter: 0,
            apu_pulse1_env_counter: 0,
            apu_pulse1_duty_step: 0,
            apu_pulse1_length_timer: 0,
            apu_pulse1_enabled: false,
            apu_pulse1_curr_volume: 0,
            apu_pulse2_freq_counter: 0,
            apu_pulse2_env_counter: 0,
            apu_pulse2_duty_step: 0,
            apu_pulse2_length_timer: 0,
            apu_pulse2_enabled: false,
            apu_pulse2_curr_volume: 0,
            apu_noise_freq_counter: 0,
            apu_noise_env_counter: 0,
            apu_noise_duty_step: 0,
            apu_noise_length_timer: 0,
            apu_noise_enabled: false,
            apu_noise_curr_volume: 0,
            apu_clock_timer: 0,
            apu_clock: 0,
            reg_a: 0,
            reg_f: 0,
            reg_b: 0,
            reg_c: 0,
            reg_d: 0,
            reg_e: 0,
            reg_h: 0,
            reg_l: 0,
            times: [0xFF; 0x100],
            curr_cycles_to_wait: 0,
            last_opcode: 0x00,
            last_opcode_cycles: 0,
            new_instruction_tick: false,
            rom_chip_enabled: true,
            eram_chip_enabled: false,
            curr_rom_bank: 1,
            curr_eram_bank: 0,
            cpu_cycle_counter: 0,
            is_halted: false,
            timer_div: 0,
            timer_overflow: false,
            oam_dma_counter: 0,
            oam_dma_source: 0,
            joypad_state: 0xFF,
            window_is_rendering: false,
            save_path: "".to_string(),
            debug_enabled: false,
            debug_bios: false,
            debug_require_input: false,
            apu_pulse1_sweep_timer: 0,
            apu_pulse1_sweep_shadow_freq: 0,
            apu_pulse1_sweep_enable: false,
            apu_wave_freq_counter: 0,
            apu_wave_env_counter: 0,
            apu_wave_duty_step: 0,
            apu_wave_length_timer: 0,
            apu_wave_enabled: false,
            apu_sink: Sink::try_new(&stream_device).unwrap(),
        };

        // Init RNG
        let mut rng = rand::thread_rng();

        // Randomize VRAM
        for value in &mut new_game_boy.vram {
            *value = rng.gen_range(0..=255) as u8;
        }

        // Randomize WRAM
        for value in &mut new_game_boy.wram {
            *value = rng.gen_range(0..=255) as u8;
        }

        // Randomize OAM
        for value in &mut new_game_boy.oam {
            *value = rng.gen_range(0..=255) as u8;
        }

        // Randomize HRAM
        for value in &mut new_game_boy.hram {
            *value = rng.gen_range(0..=255) as u8;
        }

        // Load boot rom file
        let mut dmg_boot_path = env::current_exe().unwrap();
        dmg_boot_path.pop();
        dmg_boot_path.push("bios");
        dmg_boot_path.push("dmg_boot.bin");

        let bios_bytes = match std::fs::read(dmg_boot_path) {
            Ok(bytes) => bytes,
            Err(_e) => Vec::from(FLAN_BOOT_ROM),
        };

        // Copy boot rom file to Game Boy
        new_game_boy.bios[..256].copy_from_slice(&bios_bytes[..256]);

        // Init IO registers
        new_game_boy.init_io_registers();

        // Done!
        new_game_boy
    }

    pub(crate) fn print_reg_state(&self) {
        println!("AF: {:02X} {:02X}", self.reg_a, self.reg_f);
        println!("BC: {:02X} {:02X}", self.reg_b, self.reg_c);
        println!("DE: {:02X} {:02X}", self.reg_d, self.reg_e);
        println!("HL: {:02X} {:02X}", self.reg_h, self.reg_l);
        println!("PC: {:02X} {:02X}", self.pc >> 8, self.pc & 0xFF);
        println!("SP: {:02X} {:02X}", self.sp >> 8, self.sp & 0xFF);
        println!(
            "Previous instruction cycle count: {} M / {} T",
            self.last_opcode_cycles,
            self.last_opcode_cycles * 4,
        );
        println!("CPU cycle counter: {}", self.cpu_cycle_counter);
        println!(
            "rDIV: {:02X}, rTIMA: {:02X}, rTMA: {:02X}, rTAC: {:08b}",
            self.io[0x04], self.io[0x05], self.io[0x06], self.io[0x07]
        );
        println!(
            "rIME: {:02X}, rIE:   {:02X}, rIF:  {:02X}, rSTAT:{:08b}, rLY:  {}",
            self.ime, self.ie, self.io[0x0F], self.io[0x41], self.io[0x44]
        );
    }

    pub(crate) fn dump_memory(&mut self, file_path: &str, memory_start: u16, dump_length: u16) {
        // Get Vec<u8> of all the bytes in the range specified
        let mut bytes: Vec<u8> = Vec::new();
        bytes.reserve(dump_length as usize);
        for x in memory_start..(memory_start + dump_length) {
            bytes.push(self.fetch_byte_from_memory(x));
        }

        // Dump to file
        fs::write(file_path, bytes).unwrap();
    }

    pub(crate) fn render_memory(
        &mut self,
        buffer: &mut Vec<u32>,
        memory_start: usize,
        tile_w: usize,
        tile_h: usize,
        offset_x: usize,
        offset_y: usize,
        pixel_scale: usize,
    ) {
        // Render outline
        let end_x = offset_x + tile_w * 8 * pixel_scale;
        let end_y = offset_y + tile_h * 8 * pixel_scale;
        for x in (offset_x - 1)..=(end_x + 1) {
            buffer[x + (offset_y - 1) * DEBUG_WIDTH] = 0xFFFF00FF;
            buffer[x + (end_y + 1) * DEBUG_WIDTH] = 0xFFFF00FF;
        }
        for y in (offset_y - 1)..=(end_y + 1) {
            buffer[(offset_x - 1) + y * DEBUG_WIDTH] = 0xFFFF00FF;
            buffer[(end_x + 1) + y * DEBUG_WIDTH] = 0xFFFF00FF;
        }

        let ram_base = memory_start;
        // Loop over each tile
        for tile_y in 0..tile_h {
            for tile_x in 0..tile_w {
                // Loop over each pixel in a tile
                for pixel_y in 0..8 {
                    // Calculate the tile address for the current pixel
                    let tile_address =
                        ram_base + tile_y * (tile_w * 0x0010) + tile_x * 0x0010 + pixel_y * 2;
                    let tile_address = tile_address as u16;

                    // Get the 2 bytes for the pixel row, where row_1 is the LSB and row_2 the MSB
                    // This means if only row_1's bit is set, the colour is dark grey
                    let row_1 = self.fetch_byte_from_memory(tile_address);
                    let row_2 = self.fetch_byte_from_memory(tile_address + 1);
                    self.curr_cycles_to_wait -= 2;

                    for pixel_x in 0..8 {
                        // Calculate pixel brightness from 0 to 3
                        let mut brightness = 0;
                        if ((row_1 >> (7 - pixel_x)) & 0x01) > 0 {
                            brightness += 1;
                        }
                        if ((row_2 >> (7 - pixel_x)) & 0x01) > 0 {
                            brightness += 2;
                        }

                        // Brighten that up so it's actually visible on screen
                        brightness *= 255 / 3;
                        brightness = brightness | brightness << 16;
                        brightness = brightness | brightness << 8;
                        brightness |= 0xFF000000;

                        for y in 0..pixel_scale {
                            for x in 0..pixel_scale {
                                // Calculate buffer index for this pixel
                                let buffer_x = (tile_x * 8 + pixel_x) * pixel_scale + x + offset_x;
                                let buffer_y = (tile_y * 8 + pixel_y) * pixel_scale + y + offset_y;
                                let buffer_index = buffer_x + buffer_y * DEBUG_WIDTH;

                                // Set the pixel in the buffer
                                buffer[buffer_index] = brightness;
                            }
                        }
                    }
                }
            }
        }
    }
}

const FLAN_BOOT_ROM: [u8; 256] = [
    0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x11, 0x9E, 0x00, 0x21,
    0x10, 0x80, 0x1A, 0x22, 0x22, 0x1C, 0x20, 0xFA, 0x21, 0xE8, 0x98, 0x01, 0x1C, 0x00, 0x3E, 0x01,
    0x22, 0x3C, 0x22, 0x3C, 0x22, 0x3C, 0x22, 0x3C, 0x09, 0x22, 0x3C, 0x22, 0x3C, 0x22, 0x3C, 0x22,
    0x3C, 0x09, 0x22, 0x3C, 0x22, 0x3C, 0x22, 0x3C, 0x22, 0x3C, 0x09, 0x21, 0x47, 0xFF, 0x3E, 0x00,
    0x77, 0x3E, 0x91, 0xE0, 0x40, 0x06, 0x0A, 0xCD, 0x8E, 0x00, 0x0E, 0x03, 0x06, 0x06, 0xCD, 0x8E,
    0x00, 0x3E, 0x54, 0x86, 0x77, 0x0D, 0x20, 0xF4, 0x06, 0x32, 0xCD, 0x8E, 0x00, 0x0E, 0x03, 0x06,
    0x06, 0xCD, 0x8E, 0x00, 0x3E, 0xAC, 0x86, 0x77, 0x0D, 0x20, 0xF4, 0xFA, 0x04, 0x01, 0xFE, 0xCE,
    0x20, 0xFE, 0x01, 0x13, 0x00, 0x11, 0xD8, 0x00, 0x21, 0x4D, 0x01, 0x31, 0xFE, 0xFF, 0x3E, 0xE1,
    0xE0, 0x0F, 0xAF, 0xE0, 0xFF, 0xF3, 0x3E, 0xFF, 0xC6, 0x01, 0x3E, 0x01, 0x18, 0x70, 0xF0, 0x44,
    0xFE, 0x90, 0x20, 0xFA, 0x16, 0x00, 0x14, 0x20, 0xFD, 0x05, 0x20, 0xF2, 0xC9, 0xFF, 0x00, 0x00,
    0x01, 0x01, 0x01, 0x02, 0x02, 0x02, 0x3F, 0xD5, 0xAA, 0x55, 0xAA, 0xD5, 0x7F, 0x00, 0xFC, 0x57,
    0xAA, 0x55, 0xAA, 0x55, 0xFE, 0x00, 0x00, 0x00, 0x80, 0x80, 0x80, 0x40, 0x40, 0x40, 0x02, 0x04,
    0x04, 0x04, 0x04, 0x04, 0x04, 0x02, 0x04, 0x04, 0x04, 0x00, 0x10, 0x08, 0x07, 0x00, 0x20, 0x20,
    0x20, 0x00, 0x08, 0x10, 0xE0, 0x00, 0x40, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x40, 0x01, 0x00,
    0x00, 0x7A, 0x42, 0x72, 0x42, 0x43, 0x80, 0x7F, 0x00, 0x19, 0x25, 0x3D, 0x25, 0xA5, 0x01, 0xFE,
    0x00, 0x20, 0xAD, 0x61, 0x21, 0x20, 0x80, 0x00, 0x00, 0xCC, 0x0A, 0x6C, 0x2A, 0xCC, 0xE0, 0x50,
];
