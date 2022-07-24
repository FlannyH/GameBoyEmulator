use super::super::{FlagMask, GameBoy};

impl GameBoy {
    pub(in super::super) fn handle_branch_instructions(&mut self, opcode: u8) -> bool {
        match opcode {
            0x20 => {
                //jr nz, r8
                // Get relative
                let offset = self.fetch_next_byte_from_pc();

                // If the zero flag is not set, branch
                if self.reg_f & (FlagMask::ZERO as u8) == 0 {
                    self.jump_relative(offset);
                }
            }
            0x30 => {
                //jr nc, r8
                // Get relative
                let offset = self.fetch_next_byte_from_pc();

                // If the carry flag is not set, branch
                if self.reg_f & (FlagMask::CARRY as u8) == 0 {
                    self.jump_relative(offset);
                }
            }
            0x18 => {
                // Get relative
                let offset = self.fetch_next_byte_from_pc();
                self.jump_relative(offset);
            }
            0x28 => {
                //jr z, r8
                // Get relative
                let offset = self.fetch_next_byte_from_pc();

                // If the zero flag is set, branch
                if self.reg_f & (FlagMask::ZERO as u8) != 0 {
                    self.jump_relative(offset);
                }
            }
            0x38 => {
                //jr c, r8
                // Get relative
                let offset = self.fetch_next_byte_from_pc();

                // If the carry flag is set, branch
                if self.reg_f & (FlagMask::CARRY as u8) != 0 {
                    self.jump_relative(offset);
                }
            }
            0xC2 => {
                // jp nz, a16
                let target = self.fetch_next_short_from_pc();
                if self.reg_f & (FlagMask::ZERO as u8) == 0 {
                    self.jump_absolute(target);
                }
            }
            0xD2 => {
                // jp nc, a16
                let target = self.fetch_next_short_from_pc();
                if self.reg_f & (FlagMask::CARRY as u8) == 0 {
                    self.jump_absolute(target);
                }
            }
            0xCA => {
                // jp z, a16
                let target = self.fetch_next_short_from_pc();
                if self.reg_f & (FlagMask::ZERO as u8) != 0 {
                    self.jump_absolute(target);
                }
            }
            0xDA => {
                // jp c, a16
                let target = self.fetch_next_short_from_pc();
                if self.reg_f & (FlagMask::CARRY as u8) != 0 {
                    self.jump_absolute(target);
                }
            }
            0xC3 => {
                // jp a16
                let target = self.fetch_next_short_from_pc();
                self.jump_absolute(target);
            }
            0xE9 => {
                // jp hl
                let target = ((self.reg_h as u16) << 8) | (self.reg_l as u16);
                self.jump_absolute(target);
            }
            0xC4 => {
                // call nz, a16
                let target = self.fetch_next_short_from_pc();
                if self.reg_f & (FlagMask::ZERO as u8) == 0 {
                    self.push_stack(self.pc);
                    self.jump_absolute(target);
                }
            }
            0xD4 => {
                // call nc, a16
                let target = self.fetch_next_short_from_pc();
                if self.reg_f & (FlagMask::CARRY as u8) == 0 {
                    self.push_stack(self.pc);
                    self.jump_absolute(target);
                }
            }
            0xCC => {
                // call z, a16
                let target = self.fetch_next_short_from_pc();
                if self.reg_f & (FlagMask::ZERO as u8) != 0 {
                    self.push_stack(self.pc);
                    self.jump_absolute(target);
                }
            }
            0xDC => {
                // call c, a16
                let target = self.fetch_next_short_from_pc();
                if self.reg_f & (FlagMask::CARRY as u8) != 0 {
                    self.push_stack(self.pc);
                    self.jump_absolute(target);
                }
            }
            0xCD => {
                // call a16
                let target = self.fetch_next_short_from_pc();
                self.push_stack(self.pc);
                self.jump_absolute(target);
            }
            0xC0 => {
                // ret nz
                if self.reg_f & (FlagMask::ZERO as u8) == 0 {
                    let target = self.pop_stack();
                    self.jump_absolute(target);
                }
            }
            0xD0 => {
                // ret nc
                if self.reg_f & (FlagMask::CARRY as u8) == 0 {
                    let target = self.pop_stack();
                    self.jump_absolute(target);
                }
            }
            0xC8 => {
                // ret z
                if self.reg_f & (FlagMask::ZERO as u8) != 0 {
                    let target = self.pop_stack();
                    self.jump_absolute(target);
                }
            }
            0xD8 => {
                // ret c
                if self.reg_f & (FlagMask::CARRY as u8) != 0 {
                    let target = self.pop_stack();
                    self.jump_absolute(target);
                }
            }
            0xC9 => {
                // ret
                let target = self.pop_stack();
                self.jump_absolute(target);
            }
            0xD9 => {
                // reti
                self.ime = 1;
                let target = self.pop_stack();
                self.jump_absolute(target);
            }
            0xC7 => {
                // rst $00
                self.push_stack(self.pc);
                self.jump_absolute(0x00);
            }
            0xD7 => {
                // rst $10
                self.push_stack(self.pc);
                self.jump_absolute(0x10);
            }
            0xE7 => {
                // rst $20
                self.push_stack(self.pc);
                self.jump_absolute(0x20);
            }
            0xF7 => {
                // rst $30
                self.push_stack(self.pc);
                self.jump_absolute(0x30);
            }
            0xCF => {
                // rst $08
                self.push_stack(self.pc);
                self.jump_absolute(0x08);
            }
            0xDF => {
                // rst $18
                self.push_stack(self.pc);
                self.jump_absolute(0x18);
            }
            0xEF => {
                // rst $28
                self.push_stack(self.pc);
                self.jump_absolute(0x28);
            }
            0xFF => {
                // rst $38
                self.push_stack(self.pc);
                self.jump_absolute(0x38);
            }
            0xC5 => self.push_stack((self.reg_b as u16) << 8 | (self.reg_c as u16)),
            0xD5 => self.push_stack((self.reg_d as u16) << 8 | (self.reg_e as u16)),
            0xE5 => self.push_stack((self.reg_h as u16) << 8 | (self.reg_l as u16)),
            0xF5 => self.push_stack((self.reg_a as u16) << 8 | (self.reg_f as u16)),
            0xC1 => {
                let popped = self.pop_stack();
                self.reg_b = (popped >> 8) as u8;
                self.reg_c = (popped & 0xFF) as u8;
            }
            0xD1 => {
                let popped = self.pop_stack();
                self.reg_d = (popped >> 8) as u8;
                self.reg_e = (popped & 0xFF) as u8;
            }
            0xE1 => {
                let popped = self.pop_stack();
                self.reg_h = (popped >> 8) as u8;
                self.reg_l = (popped & 0xFF) as u8;
            }
            0xF1 => {
                let popped = self.pop_stack();
                self.reg_a = (popped >> 8) as u8;
                self.reg_f = (popped & 0xF0) as u8;
            }
            _ => return false,
        }
        return true;
    }
}
