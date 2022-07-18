use crate::GameBoy;

impl GameBoy {
    pub(in super::super) fn handle_load_instructions(&mut self, opcode: u8) -> bool {
        // Handle all the register load instructions
        if (0x40..=0x7F).contains(&opcode) {
            // Get the value from B
            let b = self.load_value_from_register(opcode & 0x07);
            self.store_value_to_register((opcode >> 3) & 0x07, b);

            return true;
        }

        //Handle all the other LD instructions
        match opcode {
            // LD [r16], A
            0x02 => self.store8_to_pointer16(self.reg_b, self.reg_c, self.reg_a),
            0x12 => self.store8_to_pointer16(self.reg_d, self.reg_e, self.reg_a),
            0x22 => {
                self.store8_to_pointer16(self.reg_h, self.reg_l, self.reg_a);
                inc16(&mut self.reg_h, &mut self.reg_l)
            }
            0x32 => {
                self.store8_to_pointer16(self.reg_h, self.reg_l, self.reg_a);
                dec16(&mut self.reg_h, &mut self.reg_l)
            }
            // LD A, [r16]
            0x0A => self.reg_a = self.load8_from_pointer16(self.reg_b, self.reg_c),
            0x1A => self.reg_a = self.load8_from_pointer16(self.reg_d, self.reg_e),
            0x2A => {
                self.reg_a = self.load8_from_pointer16(self.reg_h, self.reg_l);
                inc16(&mut self.reg_h, &mut self.reg_l)
            }
            0x3A => {
                self.reg_a = self.load8_from_pointer16(self.reg_h, self.reg_l);
                dec16(&mut self.reg_h, &mut self.reg_l)
            }

            // LD r8, d8
            0x06 => self.reg_b = self.fetch_next_byte_from_pc(),
            0x16 => self.reg_d = self.fetch_next_byte_from_pc(),
            0x26 => self.reg_h = self.fetch_next_byte_from_pc(),
            0x36 => {
                let fetch = self.fetch_next_byte_from_pc();
                self.store8_to_pointer16(self.reg_h, self.reg_l, fetch)
            }
            0x0E => self.reg_c = self.fetch_next_byte_from_pc(),
            0x1E => self.reg_e = self.fetch_next_byte_from_pc(),
            0x2E => self.reg_l = self.fetch_next_byte_from_pc(),
            0x3E => self.reg_a = self.fetch_next_byte_from_pc(),

            // LDH (a8), A
            0xE0 => {
                let fetch = self.fetch_next_byte_from_pc();
                self.store8_to_pointer16(0xFF, fetch, self.reg_a);
            }
            0xF0 => {
                let fetch = self.fetch_next_byte_from_pc();
                self.reg_a = self.load8_from_pointer16(0xFF, fetch);
            }

            // LD (a16), A
            0xEA => {
                let address = self.fetch_next_short_from_pc();
                self.store8_to_pointer16((address >> 8) as u8, (address & 0xFF) as u8, self.reg_a);
            }
            0xFA => {
                let address = self.fetch_next_short_from_pc();
                self.reg_a =
                    self.load8_from_pointer16((address >> 8) as u8, (address & 0xFF) as u8);
            }

            // LD [C], A
            0xE2 => self.store8_to_pointer16(0xFF, self.reg_c, self.reg_a),
            0xF2 => self.reg_a = self.load8_from_pointer16(0xFF, self.reg_c),
            // LD r16, d16
            0x01 => {
                let fetch = self.fetch_next_short_from_pc();
                self.reg_b = (fetch >> 8) as u8;
                self.reg_c = (fetch & 0xFF) as u8;
            }
            0x11 => {
                let fetch = self.fetch_next_short_from_pc();
                self.reg_d = (fetch >> 8) as u8;
                self.reg_e = (fetch & 0xFF) as u8;
            }
            0x21 => {
                let fetch = self.fetch_next_short_from_pc();
                self.reg_h = (fetch >> 8) as u8;
                self.reg_l = (fetch & 0xFF) as u8;
            }
            0x31 => self.sp = self.fetch_next_short_from_pc(),

            // LD [a16], SP
            0x08 => {
                let target_address = self.fetch_next_short_from_pc();
                self.store_byte_to_memory(target_address + 0, (self.sp & 0xFF) as u8);
                self.store_byte_to_memory(target_address + 1, (self.sp >> 8) as u8);
            }

            // LD HL, SP + r8
            0xF8 => {
                // SP + r8
                let mut sp_h = (self.sp >> 8) as u8;
                let mut sp_l = (self.sp & 0xFF) as u8;
                let rel_offset = self.fetch_next_byte_from_pc();
                (sp_h, sp_l) = self.add_16_s8(sp_h, sp_l, rel_offset);

                // store to HL
                self.reg_h = sp_h;
                self.reg_l = sp_l;
            }

            // LD SP, HL
            0xF9 => {
                self.sp = (self.reg_h as u16) << 8 | (self.reg_l as u16);
            }

            _ => return false,
        }
        return true;
    }

    pub (in super::super) fn load_value_from_register(&mut self, index: u8) -> u8 {
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
}

pub(in super::super) fn inc16(h: &mut u8, l: &mut u8) {
    if *l == 0xFF {
        if *h == 0xFF {
            *h = 0x00;
        } else {
            *h += 1;
        }
        *l = 0x00;
    } else {
        *l += 1;
    }
}

pub(in super::super) fn dec16(h: &mut u8, l: &mut u8) {
    if *l == 0x00 {
        if *h == 0x00 {
            *h = 0xFF;
        } else {
            *h -= 1;
        }
        *l = 0xFF;
    } else {
        *l -= 1;
    }
}
