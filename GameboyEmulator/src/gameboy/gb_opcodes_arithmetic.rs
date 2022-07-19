use super::{gb_opcodes_ld::inc16, GameBoy};

impl GameBoy {
    pub(in super::super) fn handle_arithmetic_instructions(&mut self, opcode: u8) -> bool {
        // Find the value to use on A
        let mut rh = 0x00;
        let mut found = false;

        // The big block between 0x80 and 0xBF
        if (0x80..=0xBF).contains(&opcode)
        {
            rh = self.load_reg(opcode & 0x07);
            found = true;
        }

        //The little strips after that
        if opcode & 0b11000110 == 0b11000110
        {
            rh = self.fetch_next_byte_from_pc();
            found = true;
        }

        // If we don't have a valid arithmetic opcode, we're done here, let another function try to parse this opcode
        if found == false
        {
            return false;
        }

        // Now let's see which function we're gonna do by filtering and shifting these bits: ..xxx...
        let function_index = (opcode & 0b00111000) >> 3;

        // Perform the operation
        match function_index {
            0 => self.reg_a = self.add_8_8(self.reg_a, rh),
            1 => self.reg_a = self.adc_8_8(self.reg_a, rh),
            2 => self.reg_a = self.sub_8_8(self.reg_a, rh),
            3 => self.reg_a = self.sbc_8_8(self.reg_a, rh),
            4 => self.reg_a = self.and_8_8(self.reg_a, rh),
            5 => self.reg_a = self.xor_8_8(self.reg_a, rh),
            6 => self.reg_a = self.or_8_8(self.reg_a, rh),
            7 => self.cp_8_8(self.reg_a, rh),
            _ => panic!(), // This should be impossible
        }
        return true;
    }

    pub(in super::super) fn handle_incdec_instructions(&mut self, opcode: u8) -> bool {
        //inc r8 / dec r8
        // Due to the fact that the opcodes are very nicely ordered in the opcode table, this AND pattern catches every inc r8 and dec r8 there is
        if opcode & 0b11000110 == 0b00000100 {
            // I love the layout of the opcode table; to get the register index this just *works*
            let index = (opcode & 0b00111000) >> 3;

            // The inc and dec instructions are right next to each other in the opcode table, so the LSB can be used to determine that
            let inc_or_dec = (opcode & 0b00000001);

            // Get the value
            let mut reg_value = self.load_reg(index);

            // Inc or Dec it, depending on that LSB
            if (inc_or_dec == 0) {
                reg_value = self.inc8(reg_value);
            } else {
                reg_value = self.dec8(reg_value);
            }

            // Store it back into the register
            self.store_reg(index, reg_value);
            return true;
        }

        // inc r16 / dec r16
        match opcode {
            0x03 => inc16(&mut self.reg_b, &mut self.reg_c),
            0x13 => inc16(&mut self.reg_d, &mut self.reg_e),
            0x23 => inc16(&mut self.reg_h, &mut self.reg_l),
            0x33 => {
                let mut sp_h = (self.sp >> 8) as u8;
                let mut sp_l = (self.sp & 0xFF) as u8;
                inc16(&mut sp_h, &mut sp_l);
                self.sp = (sp_h as u16) << 8 | (sp_l as u16);
            }
            _ => return false,
        }
        return true;
    }
}
