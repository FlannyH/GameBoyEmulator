use super::GameBoy;

impl GameBoy {
    pub(in super::super) fn handle_arithmetic_instructions(&mut self, opcode: u8) -> bool {
        // Handle 0x80..=0xBF
        // Get right hand value, left hand value is always register A
        let rh = self.load_value_from_register(opcode & 0x07);

        // Perform the operation
        match opcode {
            0x80..=0x87 => self.reg_a = self.add_8_8(self.reg_a, rh),
            0x88..=0x8F => self.reg_a = self.adc_8_8(self.reg_a, rh),
            0x90..=0x97 => self.reg_a = self.sub_8_8(self.reg_a, rh),
            0x98..=0x9F => self.reg_a = self.sbc_8_8(self.reg_a, rh),
            0xA0..=0xA7 => self.reg_a = self.and_8_8(self.reg_a, rh),
            0xA8..=0xAF => self.reg_a = self.xor_8_8(self.reg_a, rh),
            0xB0..=0xB7 => self.reg_a = self.or_8_8(self.reg_a, rh),
            0xB8..=0xBF => self.cp_8_8(self.reg_a, rh),
            _ => (),
        }
        return true;
    }
}
