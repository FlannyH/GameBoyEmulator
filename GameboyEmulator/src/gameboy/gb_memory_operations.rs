pub mod gb_memory_operations {
    use crate::gameboy::GameBoy;

    impl GameBoy {
        pub(in super::super) fn fetch_byte_from_memory(&self, address: u16) -> u8 {
            match address {
                // ROM bank 0
                0x0000..=0x3FFF => {
                    // TODO: bootrom support
                    if self.rom_chip_enabled && address < 0x0100 {
                        return self.bios[address as usize];
                    }

                    if self.rom.len() > 0 {
                        self.rom[(address as usize) % self.rom.len()]
                    } else {
                        0xFF
                    }
                }
                // ROM bank 1 or higher
                0x4000..=0x7FFF => {
                    todo!();
                }
                // VRAM bank 0 or 1
                0x8000..=0x9FFF => {
                    // TODO: make sure this only returns the right value when PPU is unlocked, otherwise return 0xFF
                    self.vram[(address & 0x1FFF) as usize]
                }
                // External RAM
                0xA000..=0xBFFF => {
                    todo!();
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
                    // TODO: make sure this only returns the right value when PPU is unlocked, otherwise return 0xFF
                    self.oam[(address & 0xFF) as usize]
                }
                // Not usable
                0xFEA0..=0xFEFF => 0xFF,
                // I/O registers
                0xFF00..=0xFF7F => {
                    // TODO: IO behaves differently per value
                    todo!();
                }
                // HRAM
                0xFF80..=0xFFFE => self.hram[(address & 0x7F) as usize],
                // Interrupts Enable Register
                0xFFFF => self.ie,
            }
        }

        pub(in super::super) fn store_byte_to_memory(&mut self, address: u16, value: u8) {
            match address {
                // ROM bank 0
                0x0000..=0x3FFF => {
                    // TODO: implement mapper stuff
                    ()
                }
                // ROM bank 1 or higher
                0x4000..=0x7FFF => {
                    // TODO: implement mapper stuff
                    ()
                }
                // VRAM bank 0 or 1
                0x8000..=0x9FFF => {
                    // TODO: make sure this only returns the right value when PPU is unlocked, otherwise return 0xFF
                    self.vram[(address & 0x1FFF) as usize] = value;
                }
                // External RAM
                0xA000..=0xBFFF => {
                    todo!();
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
                    // TODO: make sure this only returns the right value when PPU is unlocked, otherwise return 0xFF
                    self.oam[(address & 0xFF) as usize] = value;
                }
                // Not usable
                0xFEA0..=0xFEFF => (),
                // I/O registers
                0xFF00..=0xFF7F => {
                    // TODO: IO behaves differently per value
                    if self.handle_io_register_write(address, value) == false {
                        println!("--Tried to access memory address ${:04X}, which is an IO register that isn't yet implemented!", address);
                        todo!();
                    }
                }
                // HRAM
                0xFF80..=0xFFFE => self.hram[(address & 0x7F) as usize] = value,
                // Interrupts Enable Register
                0xFFFF => self.ie = value,
            }
        }

        pub(in super::super) fn fetch_next_byte_from_pc(&mut self) -> u8 {
            let byte = self.fetch_byte_from_memory(self.pc);
            self.pc += 1;
            return byte;
        }
        pub(in super::super) fn fetch_next_short_from_pc(&mut self) -> u16 {
            let byte1 = self.fetch_byte_from_memory(self.pc) as u16;
            self.pc += 1;
            let byte2 = self.fetch_byte_from_memory(self.pc) as u16;
            self.pc += 1;
            byte1 + (byte2 << 8)
        }
        pub(in super::super) fn fetch_short_from_memory(&self, address: u16) -> u16 {
            let byte1 = self.fetch_byte_from_memory(address) as u16;
            let address = address + 1;
            let byte2 = self.fetch_byte_from_memory(address) as u16;
            return byte1 + byte2 << 8;
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
}
