use super::GameBoy;

impl GameBoy {
    pub(in super::super) fn jump_absolute(&mut self, address: u16) {
        self.pc = address;
    }

    pub(in super::super) fn jump_relative(&mut self, offset: u8) {
        match offset {
            0x00..=0x7F => self.pc += offset as u16,
            0x80..=0xFF => self.pc -= ((!offset) + 1) as u16,
        }
    }

    pub(in super::super) fn push_stack(&mut self, value: u16) {
        self.sp -= 1;
        self.store_byte_to_memory(self.sp, (value >> 8) as u8);
        self.sp -= 1;
        self.store_byte_to_memory(self.sp, (value & 0xFF) as u8);
    }

    pub(in super::super) fn pop_stack(&mut self) -> u16 {
        let mut popped_value = 0x00;
        popped_value |= self.fetch_byte_from_memory(self.sp) as u16;
        self.sp -= 1;
        popped_value |= (self.fetch_byte_from_memory(self.sp) as u16) << 8;
        self.sp -= 1;
        return popped_value;
    }
}
