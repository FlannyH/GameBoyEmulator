use super::GameBoy;

impl GameBoy {
    pub(in crate) fn run_frame(&mut self) {
        for i in 0..96 {
            self.process_next_instruction();
            //self.print_reg_state();
        }
    }

    pub(in super::super) fn process_next_instruction(&mut self) {
        // Read byte from PC
        let opcode = self.fetch_next_byte_from_pc();
        //println!("Opcode: ${:02X}", opcode);

        // Pass it to a bunch of functions, let them handle it. If none of them handle it, this is an invalid opcode, and we should hang.
        if self.handle_misc_instructions(opcode) {
            return;
        }
        if self.handle_load_instructions(opcode) {
            return;
        }
        if self.handle_arithmetic_instructions(opcode) {
            return;
        }
        if self.handle_branch_instructions(opcode) {
            return;
        }
        if self.handle_incdec_instructions(opcode) {
            return;
        }

        // If we get here, we have an instruction that we don't know how to process
        println!("--Opcode ${:02X} not implemented!--", opcode);
        panic!();
    }

    pub(in super::super) fn handle_misc_instructions(&mut self, opcode: u8) -> bool {
        // These are all the special cases that don't fit neatly into one category
        match opcode {
            0x00 => return true, // NOP - no operation
            0x07 => self.reg_a = self.rlc(self.reg_a),
            0x0F => self.reg_a = self.rrc(self.reg_a),
            0x10 => todo!(), // STOP
            0x17 => self.reg_a = self.rl(self.reg_a),
            0x1F => self.reg_a = self.rr(self.reg_a),
            0x76 => todo!(), // HALT
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
}
