use crate::gameboy::GameBoy;

impl GameBoy {
    pub(in super::super) fn fetch_byte_from_memory(&mut self, address: u16) -> u8 {
        self.curr_cycles_to_wait += 1;
        match address {
            // ROM bank 0
            0x0000..=0x3FFF => {
                if self.rom_chip_enabled && address < 0x0100 {
                    return self.bios[address as usize];
                }

                if !self.rom.is_empty() {
                    self.rom[(address as usize) % self.rom.len()]
                } else {
                    0xFF
                }
            }
            // ROM bank 1 or higher
            0x4000..=0x7FFF => {
                if !self.rom.is_empty() {
                    self.rom[(0x4000 * (self.curr_rom_bank as usize)
                        + ((address & 0x3FFF) as usize))
                        % self.rom.len()]
                } else {
                    0xFF
                }
            }
            // VRAM bank 0 or 1
            0x8000..=0x9FFF => {
                // Make sure this only returns the right value when PPU is unlocked, otherwise return 0xFF
                if self.ppu_mode != 3 {
                    self.vram[(address & 0x1FFF) as usize]
                } else {
                    0xFF
                }
            }
            // External RAM
            0xA000..=0xBFFF => {
                if self.eram_chip_enabled {
                    self.eram[(address as usize & 0x1FFF) + 0x2000 * self.curr_eram_bank as usize]
                } else {
                    0xFF
                }
            }
            // WRAM bank 0
            0xC000..=0xCFFF => self.wram[(address & 0x1FFF) as usize],
            // WRAM bank 1 or higher
            0xD000..=0xDFFF => {
                // TODO: implement bank switching
                self.wram[(address & 0x1FFF) as usize]
            }
            // WRAM bank 0 (mirror)
            0xE000..=0xEFFF => self.wram[(address & 0x1FFF) as usize],
            // WRAM bank 1 or higher (mirror)
            0xF000..=0xFDFF => {
                // TODO: implement bank switching
                self.wram[(address & 0x1FFF) as usize]
            }
            // OAM sprite attribute table
            0xFE00..=0xFE9F => {
                // Only return the byte if the PPU is not accessing this memory
                match self.ppu_mode {
                    0 => self.oam[(address & 0xFF) as usize],
                    1 => self.oam[(address & 0xFF) as usize],
                    _ => 0xFF,
                }
            }
            // Not usable
            0xFEA0..=0xFEFF => 0xFF,
            // I/O registers
            0xFF00..=0xFF7F => self.handle_io_register_read(address),

            // HRAM
            0xFF80..=0xFFFE => self.hram[(address & 0x7F) as usize],
            // Interrupts Enable Register
            0xFFFF => self.ie,
        }
    }

    pub(in super::super) fn store_byte_to_memory(&mut self, address: u16, value: u8) {
        self.curr_cycles_to_wait += 1;
        match address {
            // ROM bank 0
            0x0000..=0x3FFF => {
                // TODO: implement mapper stuff
                if (0x0000..=0x1FFF).contains(&address) {
                    self.eram_chip_enabled = value & 0x0F == 0x0A;
                }
                if (0x2000..=0x3FFF).contains(&address) {
                    self.curr_rom_bank = value;
                    if self.curr_rom_bank == 0 {
                        self.curr_rom_bank = 1;
                    }
                }
            }
            // ROM bank 1 or higher
            0x4000..=0x7FFF => {
                if (0x4000..=0x5FFF).contains(&address) {
                    self.curr_eram_bank = (value as usize % (self.eram.len() / 8192)) as u8;
                }
            }
            // VRAM bank 0 or 1
            0x8000..=0x9FFF => {
                // Only store the value if the PPU is not reading from VRAM
                if self.ppu_mode != 3 {
                    self.vram[(address & 0x1FFF) as usize] = value;
                }
            }
            // External RAM
            0xA000..=0xBFFF => {
                if self.eram_chip_enabled {
                    self.eram
                        [(address as usize & 0x1FFF) + 0x2000 * self.curr_eram_bank as usize] =
                        value;
                } //else ignore
            }
            // WRAM bank 0
            0xC000..=0xCFFF => self.wram[(address & 0x1FFF) as usize] = value,
            // WRAM bank 1 or higher
            0xD000..=0xDFFF => {
                // TODO: implement bank switching
                self.wram[(address & 0x1FFF) as usize] = value;
            }
            // WRAM bank 0 (mirror)
            0xE000..=0xEFFF => self.wram[(address & 0x1FFF) as usize] = value,
            // WRAM bank 1 or higher (mirror)
            0xF000..=0xFDFF => {
                // TODO: implement bank switching
                self.wram[(address & 0x1FFF) as usize] = value;
            }
            // OAM sprite attribute table
            0xFE00..=0xFE9F => {
                // Only return the byte if the PPU is not accessing this memory
                match self.ppu_mode {
                    0 => self.oam[(address & 0xFF) as usize] = value,
                    1 => self.oam[(address & 0xFF) as usize] = value,
                    _ => (),
                }
            }
            // Not usable
            0xFEA0..=0xFEFF => (),
            // I/O registers
            0xFF00..=0xFF7F => {
                // TODO: IO behaves differently per value
                if !self.handle_io_register_write(address, value) {
                    println!("--Tried to access memory address ${:04X}, which is an IO register that isn't yet implemented!", address);
                    todo!();
                }
            }
            // HRAM
            0xFF80..=0xFFFE => self.hram[(address & 0x7F) as usize] = value,
            // Interrupts Enable Register
            0xFFFF => {
                self.ie = value;
            }
        }
    }

    pub(in super::super) fn fetch_next_byte_from_pc(&mut self) -> u8 {
        let byte = self.fetch_byte_from_memory(self.pc);
        self.pc += 1;
        byte
    }
    pub(in super::super) fn fetch_next_short_from_pc(&mut self) -> u16 {
        let byte1 = self.fetch_byte_from_memory(self.pc) as u16;
        self.pc += 1;
        let byte2 = self.fetch_byte_from_memory(self.pc) as u16;
        self.pc += 1;
        byte1 + (byte2 << 8)
    }
    pub(in super::super) fn fetch_short_from_memory(&mut self, address: u16) -> u16 {
        let byte1 = self.fetch_byte_from_memory(address) as u16;
        let address = address + 1;
        let byte2 = self.fetch_byte_from_memory(address) as u16;
        byte1 + (byte2 << 8)
    }

    pub(in super::super) fn store8_to_pointer16(&mut self, h: u8, l: u8, value: u8) {
        let address = (h as u16) << 8 | (l as u16);
        self.store_byte_to_memory(address, value);
    }

    pub(in super::super) fn load8_from_pointer16(&mut self, h: u8, l: u8) -> u8 {
        let address = (h as u16) << 8 | (l as u16);
        self.fetch_byte_from_memory(address)
    }
}
