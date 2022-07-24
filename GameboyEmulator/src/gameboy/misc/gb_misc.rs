use std::fs;

use crate::WIDTH;
use queues::queue;
use queues::Queue;
use rand::Rng;

use super::super::GameBoy;

impl GameBoy {
    pub(in crate) fn new() -> GameBoy {
        // Create the Game Boy object
        let mut new_game_boy = GameBoy {
            bios: [0xFF; 0x100],
            rom: Vec::new(),
            vram: [0xFF; 0x2000],
            wram: [0xFF; 0x2000],
            oam: [0xFF; 0xA0],
            io: [0xFF; 0x80],
            hram: [0xFF; 0x7F],
            ie: 0,
            ime: 0,
            pc: 0,
            sp: 0,
            reg_a: 0,
            reg_f: 0,
            reg_b: 0,
            reg_c: 0,
            reg_d: 0,
            reg_e: 0,
            reg_h: 0,
            reg_l: 0,
            curr_cycles_to_wait: 0,
            rom_chip_enabled: true,
            ppu_lx: 0,
            ppu_ly: 0,
            ppu_dots_into_curr_mode: 0,
            ppu_dots_into_curr_line: 0,
            ppu_mode: 1,
            ppu_fifo: queue![],
            ppu_tilemap_x: 0,
            ppu_tilemap_y: 0,
            ppu_pixels_to_discard: 0,
            framebuffer: vec![0; 160 * 144],
            times: [0xFF; 0x100],
            last_opcode: 0x00,
            curr_rom_bank: 1,
            cpu_cycle_counter: 0,
            last_opcode_cycles: 0,
            new_instruction_tick: false,
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
        let bios_bytes = match std::fs::read("../GameboyEmulator/bios/dmg_boot.bin") {
            Ok(bytes) => bytes,
            Err(e) => {
                println!("Unable to load BIOS file, error message: {e}");
                panic!();
            }
        };

        // Copy boot rom file to Game Boy
        for i in 0..256 {
            new_game_boy.bios[i] = bios_bytes[i];
        }

        // Init IO registers
        new_game_boy.init_io_registers();

        // Done!
        new_game_boy
    }

    pub(in crate) fn print_reg_state(&self) {
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
            "rIME: {:02X}, rIE:   {:02X}, rIF:  {:02X}, rSTAT:{:08b}",
            self.ime, self.ie, self.io[0x0F], self.io[0x41]
        );
    }

    pub(in crate) fn dump_memory(&mut self, file_path: &str, memory_start: u16, dump_length: u16) {
        // Get Vec<u8> of all the bytes in the range specified
        let mut bytes: Vec<u8> = Vec::new();
        bytes.reserve(dump_length as usize);
        for x in memory_start..(memory_start + dump_length) {
            bytes.push(self.fetch_byte_from_memory(x));
        }

        // Dump to file
        fs::write(file_path, bytes).unwrap();
    }

    pub(in crate) fn render_memory(
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
            buffer[x + (offset_y - 1) * WIDTH] = 0xFFFF00FF;
            buffer[x + (end_y + 1) * WIDTH] = 0xFFFF00FF;
        }
        for y in (offset_y - 1)..=(end_y + 1) {
            buffer[(offset_x - 1) + y * WIDTH] = 0xFFFF00FF;
            buffer[(end_x + 1) + y * WIDTH] = 0xFFFF00FF;
        }

        let ram_base = memory_start;
        // Loop over each tile
        for tile_y in 0..tile_h as usize {
            for tile_x in 0..tile_w as usize {
                // Loop over each pixel in a tile
                for pixel_y in 0..8 {
                    // Calculate the tile address for the current pixel
                    let tile_address =
                        ram_base + tile_y * (tile_w * 0x0010) + tile_x * 0x0010 + pixel_y * 2;
                    let tile_address = tile_address as u16;

                    // Get the 2 bytes for the pixel row, where row_1 is the LSB and row_2 the MSB
                    // This means if only row_1's bit is set, the colour is dark grey
                    let row_1 = self.fetch_byte_from_memory(tile_address + 0);
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
                                let buffer_index = buffer_x + buffer_y * WIDTH;

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
