use crate::gameboy::OamEntry;

use super::super::cpu::gb_interrupts::InterruptMasks;
use super::super::{GameBoy, PpuFifoElement};

enum LcdInterruptMasks {
    Hblank = 1 << 3,
    Vblank = 1 << 4,
    Oam = 1 << 5,
    Lyc = 1 << 6,
}

impl GameBoy {
    pub(crate) fn run_ppu_cycle(&mut self) {
        let ppu_y_prev = self.ppu_ly;
        if self.io[0x40] & 0x80 == 0 {
            self.ppu_dots_into_curr_line = 0;
            self.ppu_dots_into_curr_mode = 0;
            self.ppu_mode = 0;
            self.ppu_ly = 0;
            self.ppu_lx = 0;
            //self.io[0x40] = 0;
            self.io[0x41] = (self.io[0x41] & 0b01111000) | 0b10000111;
            //return;
        }

        let sprite_8_or_16: usize = match self.io[0x40] & (1 << 2) {
            0 => 8,
            _ => 16,
        };

        match self.ppu_mode {
            2 => {
                // OAM scanning
                // TODO: Fetch all the sprites and filter them
                if self.ppu_dots_into_curr_mode == 1 {
                    // LCD interrupt if OAM bit is set
                    if (self.io[0x41] & LcdInterruptMasks::Oam as u8) > 0 {
                        self.io[0x0F] |= InterruptMasks::Lcd as u8;
                    }

                    // Clear OAM fetch buffer
                    self.ppu_sprite_buffer.clear();

                    // Fetch sprites on this scanline
                    for sprite_base_address in (0x00..0xA0).step_by(4) {
                        // OAM entry order: Y, X, tile, attributes
                        if (self.ppu_ly..self.ppu_ly + sprite_8_or_16 as u8)
                            .contains(&(self.oam[sprite_base_address].wrapping_sub(9)))
                        {
                            self.ppu_sprite_buffer.push(OamEntry {
                                posy: self.oam[sprite_base_address],
                                posx: self.oam[sprite_base_address + 1],
                                tile: self.oam[sprite_base_address + 2],
                                attr: self.oam[sprite_base_address + 3],
                            });
                            if self.ppu_sprite_buffer.len() == 10 {
                                break;
                            }
                        }
                    }
                }

                // After this is all done, go into ppu mode 3
                if self.ppu_dots_into_curr_mode == 79 {
                    self.ppu_dots_into_curr_mode = 0;
                    self.ppu_mode = 3; // Go to pixel drawing
                }
            }
            3 => {
                // Pixel drawing
                // If this is the first cycle in this mode, set the PPU's tilemap location, + the amount of pixels to throw away, disable the window, and then clear the FIFO
                if self.ppu_dots_into_curr_mode == 1 {
                    self.ppu_lx = 0;
                    self.ppu_tilemap_x = self.io[0x43];
                    self.ppu_tilemap_y = self.io[0x42].wrapping_add(self.ppu_ly);
                    self.ppu_pixels_to_discard = self.io[0x43] & 0x07;
                    self.ppu_fifo.clear();
                    self.window_is_rendering = false;
                    //println!("LCDC: {:08b}, WX: {}, WY: {}, LX: {}, LY: {}", self.io[0x40], self.io[0x4B], self.io[0x4A], self.ppu_lx, self.ppu_ly)
                }

                // If the window is enabled, and the window triggers at this coordinate
                if (self.io[0x40] & (1 << 5) > 0)
                    && (self.io[0x4B] - 7 <= self.ppu_lx)
                    && (self.ppu_ly >= self.io[0x4A])
                    && !self.window_is_rendering
                {
                    // Set window enable flag in PPU
                    self.window_is_rendering = true;

                    // Clear the fifo; we don't need this data anymore
                    self.ppu_fifo.clear();

                    // Change the tilemap pixel locations to the top left of the tile map + the relative Y coordinate
                    if self.io[0x4B] < 8 {
                        self.ppu_pixels_to_discard = 7 - self.io[0x4B];
                    }
                    self.ppu_tilemap_x = 0;
                    self.ppu_tilemap_y = self.ppu_ly - self.io[0x4A];

                    //println!("Enabled window rendering at screen coord ({}, {}), and window coord ({}, {})", self.ppu_lx, self.ppu_ly, self.ppu_tilemap_x, self.ppu_tilemap_y);
                }

                // If the PPU FIFO is dry, fetch 8 pixels
                if self.ppu_fifo.len() <= 8 {
                    // Create address from tilemap x and y
                    let mut tile_index_sample_address: usize = 0x9800;

                    // If the window is rendering, use a different bit to check what tile map to use
                    if !self.window_is_rendering {
                        if (self.io[0x40] & (1 << 3)) > 0 {
                            tile_index_sample_address += 0x0400;
                        }
                    } else if (self.io[0x40] & (1 << 6)) > 0 {
                        tile_index_sample_address += 0x0400;
                    }

                    tile_index_sample_address += (self.ppu_tilemap_x as usize) >> 3;
                    tile_index_sample_address += 0x020 * ((self.ppu_tilemap_y as usize) >> 3);

                    // Get tile index from memory
                    let tile_index = self.vram[tile_index_sample_address & 0x1FFF];

                    // Create tile data index
                    let mut tile_data_sample_address: usize = 0x8000;
                    tile_data_sample_address += (tile_index as usize) << 4;
                    tile_data_sample_address += (self.ppu_tilemap_y as usize & 0x07) * 2;

                    if (tile_data_sample_address < 0x8800) && (self.io[0x40] & (1 << 4) == 0) {
                        tile_data_sample_address += 0x1000;
                    }

                    // Load 2 bytes (1 rows of pixels)
                    let row_low = self.vram[tile_data_sample_address & 0x1FFF];
                    let row_high = self.vram[(tile_data_sample_address + 1) & 0x1FFF];

                    // Parse them into color indices
                    for x in 0..8 {
                        let mut pixel = 0;
                        if row_low & (1 << (7 - x)) > 0 {
                            pixel += 1;
                        }
                        if row_high & (1 << (7 - x)) > 0 {
                            pixel += 2;
                        }
                        self.ppu_fifo.push_back(PpuFifoElement {
                            color: pixel,
                            source: 0,
                        });
                    }
                    self.ppu_tilemap_x = self.ppu_tilemap_x.wrapping_add(8);
                }

                // If sprites are enabled in LCD control
                if (self.io[0x40] & (1 << 1)) > 0 {
                    // Loop over each sprite in this scanline
                    for sprite in &self.ppu_sprite_buffer {
                        if sprite.posx == self.ppu_lx.wrapping_add(8)
                            || sprite.posx < 8 && self.ppu_lx == 0
                        {
                            // Get Y of the sprite tile we want
                            let sprite_tile_y = (sprite.posy - 9) - self.ppu_ly;

                            // Get the address of the tile we want to load
                            let mut tile_data_sample_address = 0x8000;
                            tile_data_sample_address += (sprite.tile as usize) << 4;
                            if sprite.attr & 0x40 == 0 {
                                tile_data_sample_address +=
                                    (sprite_8_or_16 - 1) * 2 - ((sprite_tile_y as usize) * 2);
                            } else {
                                tile_data_sample_address += (sprite_tile_y as usize) * 2;
                            }

                            // Load the row of pixels
                            let row_low = self.vram[tile_data_sample_address & 0x1FFF];
                            let row_high = self.vram[(tile_data_sample_address + 1) & 0x1FFF];

                            // Mix it into the queue
                            for x in 0..8 {
                                // Get color index
                                let mut color_sprite = (row_low & (1 << x)) >> x;
                                color_sprite += ((row_high & (1 << x)) >> x) * 2;

                                // Get palette index
                                let mut source_sprite = 1;
                                if sprite.attr & 0x10 != 0 {
                                    source_sprite = 2;
                                }

                                // Create the new fifo element
                                let new_fifo_element = PpuFifoElement {
                                    color: color_sprite,
                                    source: source_sprite,
                                };

                                // Handle flipping
                                let mut fifo_index: isize = match sprite.attr & 0x20 {
                                    0 => 7 - x as isize,
                                    _ => x as isize,
                                };

                                // Handle sprites with x < 8
                                if sprite.posx < 8 {
                                    fifo_index -= 8 - sprite.posx as isize;
                                }

                                if fifo_index < 0 {
                                    continue;
                                }

                                // Replace the tilemap fifo element if the color isn't 0
                                if (new_fifo_element.color != 0)
                                    && !((sprite.attr & 0x80 > 0)
                                        && (self.ppu_fifo[fifo_index as usize].source == 0
                                            && self.ppu_fifo[fifo_index as usize].color != 0))
                                {
                                    self.ppu_fifo[fifo_index as usize] = new_fifo_element;
                                }
                            }
                        }
                    }
                }

                // Push a pixel
                {
                    if self.ppu_pixels_to_discard > 0 {
                        self.ppu_pixels_to_discard -= 1;
                        self.ppu_fifo.pop_front();
                    } else {
                        let curr_pixel_index = self.ppu_fifo.pop_front().unwrap();
                        let curr_pixel_color = self.io[0x47 + curr_pixel_index.source as usize]
                            >> (curr_pixel_index.color * 2)
                            & 0x03;
                        let final_color: u32 =
                            (((curr_pixel_color ^ 0b11) as u32) * (235 / 3)) * 0x00010101;
                        if (self.io[0x40] & 0x80) == 0 {
                            self.framebuffer[self.ppu_lx as usize + self.ppu_ly as usize * 160] =
                                0xFFFFFFFF; // emulate the extra white from the screen being off
                        } else if (self.io[0x40] & 0x01) == 0 {
                            self.framebuffer[self.ppu_lx as usize + self.ppu_ly as usize * 160] =
                                0xFFEAEAEA;
                        } else {
                            self.framebuffer[self.ppu_lx as usize + self.ppu_ly as usize * 160] =
                                final_color;
                        }
                        self.ppu_lx += 1;
                    }
                }

                if self.ppu_lx == 160 {
                    // Clear FIFO
                    while !self.ppu_fifo.is_empty() {
                        let _ = self.ppu_fifo.pop_front().unwrap();
                    }
                    self.ppu_dots_into_curr_mode = 0;
                    self.ppu_mode = 0; // Go to H-blank
                    if (self.io[0x41] & LcdInterruptMasks::Hblank as u8) > 0 {
                        self.io[0x0F] |= InterruptMasks::Lcd as u8;
                    }
                }
            }
            0 => {
                // If end of scanline
                if self.ppu_dots_into_curr_line == 455 {
                    self.ppu_dots_into_curr_line = 0;
                    self.ppu_dots_into_curr_mode = 0;
                    self.ppu_lx = 0;
                    self.ppu_ly += 1;
                    self.ppu_tilemap_y = self.ppu_tilemap_y.wrapping_add(1);

                    // If end of frame
                    if self.ppu_ly == 144 {
                        self.ppu_mode = 1; // Go into V-blank

                        // Request Vblank and LCD interrupts
                        if (self.io[0x41] & LcdInterruptMasks::Vblank as u8) > 0 {
                            self.io[0x0F] |= InterruptMasks::Lcd as u8;
                        }
                        self.io[0x0F] |= InterruptMasks::Vblank as u8;
                        //println!("VBLANK interrupt requested");
                    } else {
                        self.ppu_mode = 2;
                    }
                }
            }
            1 => {
                // V-blank
                self.ppu_ly = 144 + (self.ppu_dots_into_curr_mode / 456) as u8;

                // If end of V-blank, go into OAM search
                if self.ppu_dots_into_curr_mode == 4559 {
                    self.ppu_dots_into_curr_line = 0;
                    self.ppu_dots_into_curr_mode = 0;
                    self.ppu_mode = 2;
                    self.ppu_ly = 0;
                    self.ppu_lx = 0;
                }
            }
            _ => panic!(),
        }
        // Tick timer
        self.ppu_dots_into_curr_mode += 1;
        self.ppu_dots_into_curr_line += 1;

        // Update IO registers
        self.io[0x41] &= 0b11111000;
        self.io[0x41] |= self.ppu_mode & 0x03;
        if self.ppu_ly == self.io[0x45] {
            self.io[0x41] |= 0x04;
        }
        self.io[0x41] |= self.ppu_mode & 0x03;
        self.io[0x44] = self.ppu_ly;

        // Request LCD interrupt if LY==LYC
        if self.ppu_ly == self.io[0x45] && self.ppu_ly > 0 && ppu_y_prev != self.ppu_ly {
            self.io[0x41] |= 1 << 2;
            if (self.io[0x41] & LcdInterruptMasks::Lyc as u8) > 0 {
                self.io[0x0F] |= InterruptMasks::Lcd as u8;
            }
        }

        //print!("PPU STATS: mode: {:>3}, curr_dots_mode: {:>3}, curr_dots_line {:>3}, lx: {:>3}, ly: {:>3}, tilemap_x: {:>3}, tilemap_y: {:>3}    \r", self.ppu_mode, self.ppu_dots_into_curr_mode, self.ppu_dots_into_curr_line, self.ppu_lx, self.ppu_ly, self.ppu_tilemap_x, self.ppu_tilemap_y);
    }
}
