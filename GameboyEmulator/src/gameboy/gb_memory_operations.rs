pub mod gb_memory_operations {
    use crate::gameboy::GameBoy;

    impl GameBoy {
        pub(in super::super) fn fetch_byte_from_memory(&self, address: u16) -> u8 {
            match address {
                // ROM bank 0
                0x0000..=0x3FFF => {
                    todo!();
                }
                // ROM bank 1 or higher
                0x4000..=0x7FFF => {
                    todo!();
                }
                // VRAM bank 0 or 1
                0x8000..=0x9FFF => {
                    // TODO: make sure this only returns the right value when PPU is unlocked, otherwise return 0xFF
                    !self.vram[(address & 0x1FFF) as usize]
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
            return byte1 + byte2 << 8;
        }
        pub(in super::super) fn fetch_short_from_memory(&self, address: u16) -> u16 {
            let byte1 = self.fetch_byte_from_memory(address) as u16;
            let address = address + 1;
            let byte2 = self.fetch_byte_from_memory(address) as u16;
            return byte1 + byte2 << 8;
        }
    }
}
