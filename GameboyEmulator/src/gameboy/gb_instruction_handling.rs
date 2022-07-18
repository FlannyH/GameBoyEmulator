use super::GameBoy;

impl GameBoy {
    pub(in super::super) fn process_next_instruction(&mut self) {
        // Read byte from PC
        let opcode = self.fetch_next_byte_from_pc();

        // Pass it to a bunch of functions, let them handle it. If none of them handle it, this is an invalid opcode, and we should hang.
        if self.handle_misc_instructions(&opcode) {
            return;
        }
        if self.handle_load_instructions(&opcode) {
            return;
        }
    }

    fn load_value_from_register(&mut self, index: u8) -> u8 {
        let a;
        match index & 0x07 {
            0 => a = self.reg_b,
            1 => a = self.reg_c,
            2 => a = self.reg_d,
            3 => a = self.reg_e,
            4 => a = self.reg_h,
            5 => a = self.reg_l,
            6 => {
                let address = (self.reg_h as u16) << 8 | self.reg_l as u16;
                a = self.fetch_byte_from_memory(address);
            }
            7 => a = self.reg_a,
            _ => panic!(),
        }
        return a;
    }
    fn store_value_to_register(&mut self, index: u8, value: u8) {
        match index & 0x07 {
            0 => self.reg_b = value,
            1 => self.reg_c = value,
            2 => self.reg_d = value,
            3 => self.reg_e = value,
            4 => self.reg_h = value,
            5 => self.reg_l = value,
            6 => {
                let address = (self.reg_h as u16) << 8 | self.reg_l as u16;
                self.store_byte_to_memory(address, value);
            }
            7 => self.reg_a = value,
            _ => panic!(),
        }
    }

    pub(in super::super) fn handle_misc_instructions(&mut self, opcode: &u8) -> bool {
        // These are all the special cases that don't fit neatly into one category
        match opcode {
            0x00 => return true, // NOP - no operation
            0x10 => todo!(),     // STOP
            0x76 => todo!(),     // HALT
            0xCB => {
                self.handle_prefixed_instructions(opcode);
            } // CB - prefixed instructions mostly for bit shifting, setting, and clearing
            0xF3 => {
                self.ime = 0;
                return true;
            }
            0xFB => {
                self.ime = 1;
                return true;
            }
            _ => return false,
        }
        return true;
    }
    pub(in super::super) fn handle_load_instructions(&mut self, opcode: &u8) -> bool {
        if (0x40..=0x7F).contains(opcode) {
            // Get the value from B
            let b = self.load_value_from_register(opcode & 0x07);
            self.store_value_to_register((opcode >> 3) & 0x07, b);

            return true;
        }
        return false;
    }

    pub(in super::super) fn handle_prefixed_instructions(&mut self, _opcode: &u8) -> bool {
        false
    }
}
