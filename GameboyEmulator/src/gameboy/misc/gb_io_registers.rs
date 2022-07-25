use super::super::GameBoy;

impl GameBoy {
    pub(in super::super) fn init_io_registers(&mut self) {
        let initial_io_state = [
            0xCF, 0x00, 0x7E, 0xFF, 0x00, 0x00, 0x00, 0xF8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xE1, 0x80, 0x3F, 0x00, 0xFF, 0xBF, 0xFF, 0x3F, 0x00, 0xFF, 0xBF, 0x7F, 0xFF,
            0x9F, 0xFF, 0xBF, 0xFF, 0xFF, 0x00, 0x00, 0xBF, 0x00, 0x00, 0x70, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x50, 0xBD, 0x02, 0xF9, 0x29, 0xF3, 0x48, 0x6E,
            0x03, 0xB2, 0xDC, 0x37, 0x16, 0xF6, 0x9D, 0x9F, 0x00, 0x84, 0x00, 0x00, 0x00, 0x00,
            0xFF, 0xFC, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF,
        ];

        for x in 0..128 {
            self.io[x] = initial_io_state[x];
        }
    }

    pub(in super::super) fn handle_io_register_write(&mut self, address: u16, value: u8) -> bool {
        // TODO: actually implement registers
        match address {
            0xFF00 => self.io[0x00] = (value & 0b00110000) | (self.io[0x00] & 0b11001111),
            0xFF04 => self.timer_div = 0x0000,
            0xFF46 => {
                self.oam_dma_counter = 160;
                self.oam_dma_source = (value as u16) << 8;
            }
            0xFF50 => self.rom_chip_enabled = false,
            _ => self.io[(address & 0x7F) as usize] = value,
        }
        return true;
    }

    pub(in super::super) fn handle_io_register_read(&self, address: u16) -> u8 {
        match address {
            0xFF00 => (self.io[0x00] & 0xF0) | (0x0F), // TODO: actually implement input
            _ => self.io[(address & 0x7F) as usize],
        }
    }
}
