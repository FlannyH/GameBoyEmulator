use super::GameBoy;

impl GameBoy {
    pub(in super::super) fn handle_io_register_write(&mut self, address: u16, value: u8) -> bool {
        if self.sound_control_registers(address, value) {
            return true;
        }
        if self.sound_channel_registers(address, value) {
            return true;
        }
        return false;
    }

    fn sound_control_registers(&mut self, address: u16, value: u8) -> bool {
        match address {
            0xFF24 => self.io[(address & 0x7F) as usize] = value,
            0xFF25 => self.io[(address & 0x7F) as usize] = value,
            0xFF26 => self.io[(address & 0x7F) as usize] = value & 0xF0,
            _ => return false,
        }
        return true;
    }

    fn sound_channel_registers(&mut self, address: u16, _value: u8) -> bool {
        if (0xFF10..=0xFF3F).contains(&address) {
            // TODO: actually implement audio channel registers
            return true;
        }
        return false;
    }
}
