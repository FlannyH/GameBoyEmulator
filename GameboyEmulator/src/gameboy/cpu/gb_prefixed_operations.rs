use super::super::FlagMask;
use super::super::GameBoy;

impl GameBoy {
    pub(in super::super) fn rlc(&mut self, value: u8) -> u8 {
        // Shift
        let result = value << 1 | value >> 7;

        // Fix flags
        self.reg_f = 0;
        if result == 0 {
            self.reg_f |= FlagMask::ZERO as u8;
        }
        if value & 0x80 > 0 {
            self.reg_f |= FlagMask::CARRY as u8;
        }
        return result;
    }
    pub(in super::super) fn rrc(&mut self, value: u8) -> u8 {
        // Shift
        let result = value >> 1 | value << 7;

        // Fix flags
        self.reg_f = 0;
        if result == 0 {
            self.reg_f |= FlagMask::ZERO as u8;
        }
        if value & 0x01 > 0 {
            self.reg_f |= FlagMask::CARRY as u8;
        }
        return result;
    }
    pub(in super::super) fn rl(&mut self, value: u8) -> u8 {
        // Shift
        let mut result = value << 1;
        if self.reg_f & FlagMask::CARRY as u8 > 0 {
            result |= 0x01;
        }

        // Fix flags
        self.reg_f = 0;
        if result == 0 {
            self.reg_f |= FlagMask::ZERO as u8;
        }
        if value & 0x80 > 0 {
            self.reg_f |= FlagMask::CARRY as u8;
        }
        return result;
    }
    pub(in super::super) fn rr(&mut self, value: u8) -> u8 {
        // Shift
        let mut result = value >> 1;
        if self.reg_f & FlagMask::CARRY as u8 > 0 {
            result |= 0x80;
        }

        // Fix flags
        self.reg_f = 0;
        if result == 0 {
            self.reg_f |= FlagMask::ZERO as u8;
        }
        if value & 0x01 > 0 {
            self.reg_f |= FlagMask::CARRY as u8;
        }
        return result;
    }
    pub(in super::super) fn sla(&mut self, value: u8) -> u8 {
        // Shift
        let result = value << 1;

        // Fix flags
        self.reg_f = 0;
        if result == 0 {
            self.reg_f |= FlagMask::ZERO as u8;
        }
        if value & 0x80 > 0 {
            self.reg_f |= FlagMask::CARRY as u8;
        }
        return result;
    }
    pub(in super::super) fn sra(&mut self, value: u8) -> u8 {
        // Shift
        let result = (value >> 1) | (value & 0x80);

        // Fix flags
        self.reg_f = 0;
        if result == 0 {
            self.reg_f |= FlagMask::ZERO as u8;
        }
        if value & 0x01 > 0 {
            self.reg_f |= FlagMask::CARRY as u8;
        }
        return result;
    }
    pub(in super::super) fn swap(&mut self, value: u8) -> u8 {
        // Swap nibbles
        let result = value >> 4 | value << 4;

        // Set flags
        self.reg_f = 0;
        if result == 0 {
            self.reg_f |= FlagMask::ZERO as u8;
        }
        return result;
    }
    pub(in super::super) fn srl(&mut self, value: u8) -> u8 {
        // Shift
        let result = value >> 1;

        // Fix flags
        self.reg_f = 0;
        if result == 0 {
            self.reg_f |= FlagMask::ZERO as u8;
        }
        if value & 0x01 > 0 {
            self.reg_f |= FlagMask::CARRY as u8;
        }
        return result;
    }
    pub(in super::super) fn bit(&mut self, value: u8, bit: u8) {
        // Clear all flags except the carry flag, we should keep that one the same. Also, set the half flag
        self.reg_f &= FlagMask::CARRY as u8;
        self.reg_f |= FlagMask::HALF as u8;

        // Check if the bit is zero, and set the zero flag accordingly
        if value & (1 << bit) == 0 {
            self.reg_f |= FlagMask::ZERO as u8;
        }
    }
    pub(in super::super) fn res(&mut self, value: u8, bit: u8) -> u8 {
        value & !(1 << bit)
    }
    pub(in super::super) fn set(&mut self, value: u8, bit: u8) -> u8 {
        value | (1 << bit)
    }
}
