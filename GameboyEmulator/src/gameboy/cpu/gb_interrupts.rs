use super::super::GameBoy;

pub enum InterruptMasks {
    Vblank = 1 << 0,
    Lcd = 1 << 1,
    Timer = 1 << 2,
    Serial = 1 << 3,
    Joypad = 1 << 4,
}

impl GameBoy {
    pub(super) fn handle_interrupts(&mut self) {
        // Update timer
        self.handle_timer();

        // If interrupts are not enabled, return
        if self.ime == 0 {
            return;
        }

        while self.io[0x0F] & self.ie > 0 {
            // For convenience, let's put all the active requested interrupts into one variable
            let requested_interrupts = self.io[0x0F] & self.ie;

            // If V-blank was requested
            if requested_interrupts & (InterruptMasks::Vblank as u8) > 0 {
                // Un-request V-blank
                self.io[0x0F] &= !(InterruptMasks::Vblank as u8);

                // Disable master interrupt flag
                self.ime = 0;

                // Call the interrupt handler
                self.push_stack(self.pc);
                self.jump_absolute(0x0040);
            }

            // The rest of the interrupts follow the same logic, so I will omit comments from here on
            if requested_interrupts & (InterruptMasks::Lcd as u8) > 0 {
                self.io[0x0F] &= !(InterruptMasks::Lcd as u8);
                self.ime = 0;
                self.push_stack(self.pc);
                self.jump_absolute(0x0048);
            }

            if requested_interrupts & (InterruptMasks::Timer as u8) > 0 {
                self.io[0x0F] &= !(InterruptMasks::Timer as u8);
                self.ime = 0;
                self.push_stack(self.pc);
                self.jump_absolute(0x0050);
            }

            if requested_interrupts & (InterruptMasks::Serial as u8) > 0 {
                self.io[0x0F] &= !(InterruptMasks::Serial as u8);
                self.ime = 0;
                self.push_stack(self.pc);
                self.jump_absolute(0x0058);
            }

            if requested_interrupts & (InterruptMasks::Joypad as u8) > 0 {
                self.io[0x0F] &= !(InterruptMasks::Joypad as u8);
                self.ime = 0;
                self.push_stack(self.pc);
                self.jump_absolute(0x0060);
            }
        }
    }

    // Should be called every CPU cycle
    fn handle_timer(&mut self) {
        // Increment internal counter
        self.cpu_cycle_counter += 1;

        // DIV register
        if self.cpu_cycle_counter % 256 == 0 {
            self.io[0x04] = self.io[0x04].wrapping_add(1);
        }

        // The rest of the function only happens if the timer is enabled, so return if it's not enabled
        if self.io[0x07] & 0b00000100 == 0 {
            return;
        }

        // Get timer period from TAC
        let timer_period = match self.io[0x07] & 0b00000011 {
            0 => 1024,
            1 => 16,
            2 => 64,
            3 => 256,
            _ => 0, // to stop rustc from crying, clearly if i and an integer with 0b00000011 the only legal values would be 0 1 2 and 3, but i guess not
        };

        // Update timer
        if self.cpu_cycle_counter % timer_period == 0 {
            self.io[0x05] = self.io[0x06].wrapping_add(1);
        }

        // Handle overflow
        if self.io[0x05] == 0x00 {
            // Request interrupt
            self.io[0x0F] |= InterruptMasks::Timer as u8;

            // Set Timer Counter to Timer Modulo
            self.io[0x05] = self.io[0x06];
        }
    }
}
