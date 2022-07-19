use super::GameBoy;

impl GameBoy {
    pub(in super::super) fn handle_prefixed_instructions(&mut self, opcode: u8) -> bool {
        // Just to be sure...
        if opcode != 0xCB {
            panic!()
        }

        // Get the next byte from the PC
        let opcode = self.fetch_next_byte_from_pc();

        // Get the register value to shift
        let input_value = self.load_value_from_register(opcode & 0x07);
        let mut value_to_store: u8 = 0x00;

        // ogh i love the match operator - perform the operation
        match opcode {
            0x00..=0x07 => value_to_store = self.rlc(input_value),
            0x08..=0x0F => value_to_store = self.rrc(input_value),
            0x10..=0x17 => value_to_store = self.rl(input_value),
            0x18..=0x1F => value_to_store = self.rr(input_value),
            0x20..=0x27 => value_to_store = self.sla(input_value),
            0x28..=0x2F => value_to_store = self.sra(input_value),
            0x30..=0x37 => value_to_store = self.swap(input_value),
            0x38..=0x3F => value_to_store = self.srl(input_value),
            0x40..=0x7F => self.bit(input_value, (opcode >> 3) & 0x07),
            0x80..=0xBF => value_to_store = self.res(input_value, (opcode >> 3) & 0x07),
            0xC0..=0xFF => value_to_store = self.set(input_value, (opcode >> 3) & 0x07),
        }

        // Ignore bit instruction, otherwise write the value to the register
        if (0x40..=0x7F).contains(&opcode) == false {
            self.store_value_to_register(opcode & 0x07, value_to_store);
        }

        return true;
    }
}
