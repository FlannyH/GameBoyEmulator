use super::FlagMask;
use super::GameBoy;

impl GameBoy {
    // Add two 8 bit values and set the flags
    pub(in super::super) fn adc_8_8(&mut self, a: u8, b: u8) -> u8 {
        // Handle carry
        let mut carry = 0;
        if self.reg_f & FlagMask::CARRY as u8 > 0x00 {
            carry = 1;
        }

        // Reset the flag register
        self.reg_f = 0x00;

        //Do a 16 bit subtraction
        let a16 = (a as u16) + ((b as u16) + (carry as u16));

        // Carry flag
        if (a16 >= 0x100) {
            self.reg_f |= FlagMask::CARRY as u8;
        }

        // Half flag
        if a16 & 0xF0 != a as u16 & 0xF0 {
            self.reg_f |= FlagMask::HALF as u8;
        }

        // Add the low and high bits together
        let a = a.wrapping_add(b + carry);

        // Handle zero flag
        if a == 0x00 {
            self.reg_f |= FlagMask::ZERO as u8;
        }
        return a;
    }

    // Subtract two 8 bit values and set the flags
    pub(in super::super) fn sbc_8_8(&mut self, a: u8, b: u8) -> u8 {
        // Handle carry
        let mut borrow = 0;
        if self.reg_f & FlagMask::CARRY as u8 > 0x00 {
            borrow = 1;
        }

        // Reset the flag register and set the negative flag
        self.reg_f = 0x00 | FlagMask::NEG as u8;

        //Do a 16 bit subtraction
        let a16 = (0x100 + (a as u16)) - ((b as u16) + (borrow as u16));

        // Carry flag
        if (a16 < 0x100) {
            self.reg_f |= FlagMask::CARRY as u8;
        }

        // Half flag
        if ((0x10 + ((a as u16) & 0x0F)) - (((b as u16) + (borrow as u16)) & 0x0F)) < 0x10 {
            self.reg_f |= FlagMask::HALF as u8;
        }

        // Add the low and high bits together
        let a = a.wrapping_sub(b + borrow);

        // Handle zero flag
        if a == 0x00 {
            self.reg_f |= FlagMask::ZERO as u8;
        }
        return a;
    }

    pub(in super::super) fn add_8_8(&mut self, a: u8, b: u8) -> u8 {
        // Reset the flag register
        self.reg_f = 0x00;

        // Reuse ADC code
        self.adc_8_8(a, b)
    }

    pub(in super::super) fn sub_8_8(&mut self, a: u8, b: u8) -> u8 {
        // Reset the flag register
        self.reg_f = 0x00;

        // Reuse ADC code
        self.sbc_8_8(a, b)
    }

    pub(in super::super) fn add_16_s8(&mut self, a_h: u8, a_l: u8, b: u8) -> (u8, u8) {
        // Add the low bytes together
        let a_l = self.add_8_8(a_l, b);
        let mut a_h = a_h;

        // Correct the flags
        self.reg_f &= FlagMask::HALF as u8 | FlagMask::CARRY as u8;

        // Handle carry. This should not affect flags and inc or dec based on the sign of B
        if self.reg_f & FlagMask::CARRY as u8 > 0x00 {
            if b >= 0x80 {
                a_h -= 1;
            } else {
                a_h += 1;
            }
        }
        return (a_h, a_l);
    }

    pub(in super::super) fn add_16_16(&mut self, a_h: u8, a_l: u8, b_h: u8, b_l: u8) -> (u8, u8) {
        // Save the Z flag since it should not change after this instruction
        let temp_flag = self.reg_f & FlagMask::ZERO as u8;

        // Perform 16 bit addition
        let a_l = self.add_8_8(a_l, b_l);
        let a_h = self.adc_8_8(a_h, b_h);

        // Restore the Z flag
        self.reg_f = self.reg_f & (!(FlagMask::ZERO as u8)) | temp_flag;

        return (a_h, a_l);
    }

    pub(in super::super) fn and_8_8(&mut self, a: u8, b: u8) -> u8 {
        // Set flag initial state
        self.reg_f = FlagMask::HALF as u8;

        // Apply operation
        let a = a & b;

        // Handle zero flag
        if a == 0x00 {
            self.reg_f |= FlagMask::ZERO as u8;
        }
        return a;
    }

    pub(in super::super) fn xor_8_8(&mut self, a: u8, b: u8) -> u8 {
        // Set flag initial state
        self.reg_f = 0x00;

        // Apply operation
        let a = a ^ b;

        // Handle zero flag
        if a == 0x00 {
            self.reg_f |= FlagMask::ZERO as u8;
        }
        return a;
    }

    pub(in super::super) fn or_8_8(&mut self, a: u8, b: u8) -> u8 {
        // Set flag initial state
        self.reg_f = 0x00;

        // Apply operation
        let a = a | b;

        // Handle zero flag
        if a == 0x00 {
            self.reg_f |= FlagMask::ZERO as u8;
        }
        return a;
    }

    pub(in super::super) fn cp_8_8(&mut self, a: u8, b: u8) {
        // This instruction is a SUB instruction without changing the actual register, so A is not mutable
        let a = a;
        self.sub_8_8(a, b);
    }
}
