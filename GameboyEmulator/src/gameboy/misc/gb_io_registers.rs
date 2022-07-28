use super::super::GameBoy;

impl GameBoy {
    pub(in super::super) fn init_io_registers(&mut self) {
        let initial_io_state = [
            0xCF, 0x00, 0x7E, 0xFF, 0x00, 0x00, 0x00, 0xF8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xE1, 0x80, 0x3F, 0x00, 0xFF, 0xBF, 0xFF, 0x3F, 0x00, 0xFF, 0xBF, 0x7F, 0xFF,
            0x9F, 0xFF, 0xBF, 0xFF, 0xFF, 0x00, 0x00, 0xBF, 0x00, 0xFF, 0x80, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x50, 0xBD, 0x02, 0xF9, 0x29, 0xF3, 0x48, 0x6E,
            0x03, 0xB2, 0xDC, 0x37, 0x16, 0xF6, 0x9D, 0x9F, 0x00, 0x84, 0x00, 0x00, 0x00, 0x00,
            0xFF, 0xFC, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF,
        ];

        for x in 0..128 {
            self.io[x] = initial_io_state[x];
        }
    }

    pub(in super::super) fn handle_io_register_write(&mut self, address: u16, value: u8) -> bool {
        // TODO: actually implement registers
        match address {
            0xFF00 => self.io[0x00] = (value & 0b00110000) | (self.io[0x00] & 0b11001111),
            0xFF04 => self.timer_div = 0x0000,
            0xFF11 => {
                self.io[0x11] = value;
                self.apu_pulse1_length_timer = 64 - value & 0b00111111;
            }
            0xFF16 => {
                self.io[0x16] = value;
                self.apu_pulse2_length_timer = 64 - value & 0b00111111;
            }
            0xFF1B => {
                self.io[0x1B] = value;
                self.apu_wave_length_timer = 255 - value;
            }
            0xFF20 => {
                self.io[0x20] = value;
                self.apu_noise_length_timer = 64 - value & 0b00111111;
            }
            // this is just to make zombie mode for my own music engine lmao
            0xFF12 => {
                self.io[0x12] = value;
                if self.apu_pulse1_enabled && value & 0x0F == 0x08 {
                    self.apu_pulse1_curr_volume = (self.apu_pulse1_curr_volume + 1) & 0x0F;
                }
            }
            0xFF17 => {
                self.io[0x17] = value;
                if self.apu_pulse2_enabled && value & 0x0F == 0x08 {
                    self.apu_pulse2_curr_volume = (self.apu_pulse2_curr_volume + 1) & 0x0F;
                }
            }
            0xFF21 => {
                self.io[0x21] = value;
                if self.apu_noise_enabled && value & 0x0F == 0x08 {
                    self.apu_noise_curr_volume = (self.apu_noise_curr_volume + 1) & 0x0F;
                }
            }

            0xFF14 => {
                self.io[0x14] = value;
                // If channel enable flag is set
                if value & 0x80 > 0 {
                    self.apu_pulse1_enabled = true;
                    self.apu_pulse1_freq_counter = 0;
                    self.apu_pulse1_env_counter = 0;
                    self.apu_pulse1_curr_volume = self.io[0x12] >> 4;
                    self.apu_pulse1_sweep_shadow_freq =
                        (2048 - (self.io[0x13] as u16 | ((self.io[0x14] as u16) << 8)) % 2048) * 2;
                    self.apu_pulse1_sweep_timer = 0;
                    self.apu_pulse1_sweep_enable = self.io[0x10] & 0b01110111 != 0;
                    self.handle_sweep();
                }
            }
            0xFF19 => {
                self.io[0x19] = value;
                // If channel enable flag is set
                if value & 0x80 > 0 {
                    self.apu_pulse2_enabled = true;
                    self.apu_pulse2_freq_counter = 0;
                    self.apu_pulse2_env_counter = 0;
                    self.apu_pulse2_curr_volume = self.io[0x17] >> 4;
                }
            }
            0xFF1E => {
                self.io[0x1E] = value;
                // If channel enable flag is set
                if value & 0x80 > 0 {
                    self.apu_wave_enabled = true;
                    self.apu_wave_freq_counter = 0;
                    self.apu_wave_env_counter = 0;
                    self.apu_wave_duty_step = 0;
                }
            }
            0xFF23 => {
                self.io[0x23] = value;
                // If channel enable flag is set
                if value & 0x80 > 0 {
                    self.apu_noise_enabled = true;
                    self.apu_noise_freq_counter = 0;
                    self.apu_noise_env_counter = 0;
                    self.apu_noise_curr_volume = self.io[0x21] >> 4;
                    self.apu_noise_duty_step = 0b0111_1111_1111_1111;
                }
            }
            0xFF46 => {
                self.oam_dma_counter = 160;
                self.oam_dma_source = (value as u16) << 8;
            }
            0xFF50 => self.rom_chip_enabled = false,
            _ => self.io[(address & 0x7F) as usize] = value,
        }
        return true;
    }

    pub(in super::super) fn handle_io_register_read(&self, address: u16) -> u8 {
        let result = match address {
            0xFF00 => {
                let mut return_value = self.io[0x00] & 0xF0;
                if return_value & (1 << 4) == 0 {
                    return_value |= self.joypad_state >> 4;
                } else if return_value & (1 << 5) == 0 {
                    return_value |= self.joypad_state & 0x0F;
                } else {
                    return_value |= 0x0F;
                }
                return_value
            } // TODO: actually implement input
            _ => self.io[(address & 0x7F) as usize],
        };
        return result;
    }
}
