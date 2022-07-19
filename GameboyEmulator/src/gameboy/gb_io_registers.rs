use super::GameBoy;

impl GameBoy {
    pub(in super::super) fn handle_io_register_write(&mut self, address: u16, value: u8) -> bool {
        // TODO: actually implement registers
        self.io[(address & 0x7F) as usize] = value;
        return true;
    }

    pub(in super::super) fn handle_io_register_read(&self, address: u16) -> u8 {
        return self.io[(address & 0x7F) as usize];
    }
}
