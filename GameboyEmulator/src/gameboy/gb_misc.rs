use crate::{PIXEL_SCALE, WIDTH};
use rand::Rng;

use super::GameBoy;

impl GameBoy {
    pub(in crate) fn new() -> GameBoy {
        // Create the Game Boy object
        let mut new_game_boy = GameBoy {
            vram: [0xFF; 0x2000],
            wram: [0xFF; 0x2000],
            oam: [0xFF; 0xA0],
            io: [0xFF; 0x80],
            hram: [0xFF; 0x7F],
            ie: 0,
            pc: 0,
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

        // Done!
        new_game_boy
    }

    pub(in crate) fn render_debug(&self, buffer: &mut Vec<u32>) {
        // Debug rendering
        let VRAM_BASE = 0x8000;
        // Loop over each tile
        for tile_y in 0..24 as usize {
            for tile_x in 0..16 as usize {
                // Loop over each pixel in a tile
                for pixel_y in 0..8 {
                    // Calculate the tile address for the current pixel
                    let tile_address = VRAM_BASE + tile_y * 0x0100 + tile_x * 0x010 + pixel_y * 2;
                    let tile_address = tile_address as u16;

                    // Get the 2 bytes for the pixel row, where row_1 is the LSB and row_2 the MSB
                    // This means if only row_1's bit is set, the colour is dark grey
                    let row_1 = self.fetch_byte_from_memory(tile_address + 0);
                    let row_2 = self.fetch_byte_from_memory(tile_address + 1);

                    for pixel_x in 0..8 {
                        // Calculate pixel brightness from 0 to 3
                        let mut brightness = 0;
                        if ((row_1 >> pixel_x) & 0x01) > 0 {
                            brightness += 1;
                        }
                        if ((row_2 >> pixel_x) & 0x01) > 0 {
                            brightness += 2;
                        }

                        // Brighten that up so it's actually visible on screen
                        brightness *= 255 / 3;
                        brightness = brightness | brightness << 16;
                        brightness = brightness | brightness << 8;
                        brightness |= 0xFF000000;

                        for y in 0..PIXEL_SCALE {
                            for x in 0..PIXEL_SCALE {
                                // Calculate buffer index for this pixel
                                let buffer_x = (tile_x * 8 + pixel_x) * PIXEL_SCALE + x;
                                let buffer_y = (tile_y * 8 + pixel_y) * PIXEL_SCALE + y;
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
