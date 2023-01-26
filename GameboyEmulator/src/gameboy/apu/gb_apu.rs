use rodio::buffer::SamplesBuffer;

use crate::gameboy::GameBoy;

const DUTY_CYCLES: [[u8; 8]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 1, 1],
    [0, 1, 1, 1, 1, 1, 1, 0],
];

impl GameBoy {
    // One cycle is one tick in the 1048576 Hz clock
    pub(in super::super) fn run_apu_cycle(&mut self) {
        // If sound is disabled, reset all registers and dont output any audio
        if self.io[0x26] & (1 << 7) == 0 {
            for addr in 0x10..=0x25 {
                self.io[addr] = 0x00;
            }
            return;
        }

        // Otherwise, update sound
        // Get data for channel 1

        // 512 Hz clock go!
        self.apu_clock_timer += 1;

        if self.apu_clock_timer == 4096 {
            // Reset timer and inc clock
            self.apu_clock_timer = 0;
            self.apu_clock += 1;

            self.handle512_channel_1();
            self.handle512_channel_2();
            self.handle512_channel_3();
            self.handle512_channel_4();
        }

        if self.apu_buffer_write_index % (1 << 6) == 0 {
            self.apu_buffer[self.apu_buffer_to_use][(self.apu_buffer_write_index >> 6) * 2] = 0;
            self.apu_buffer[self.apu_buffer_to_use][(self.apu_buffer_write_index >> 6) * 2 + 1] = 0;
        }

        self.handle_apu_channel_1();
        self.handle_apu_channel_2();
        self.handle_apu_channel_3();
        self.handle_apu_channel_4();

        self.apu_buffer_write_index += 1;
        if self.apu_buffer_write_index == 256 << 6 {
            self.apu_buffer_write_index = 0;

            let apu_source: SamplesBuffer<u16> =
                SamplesBuffer::new(2, 32768, self.apu_buffer[self.apu_buffer_to_use]);
            //match self
            //    .apu_stream_handle
            //    .play_raw(apu_source.convert_samples())
            //{
            //    Ok(_) => (),
            //    Err(e) => {
            //        println!("Audio error: {}", e);
            //    }
            //}
            self.apu_sink.append(apu_source);
            self.apu_buffer_to_use ^= 1;
        }
    }

    fn handle_apu_channel_1(&mut self) {
        if self.io[0x25] & 0b0001_0001 == 0 {
            self.apu_sound_output[0] = 0;
            return;
        }

        // Handle frequency of channel 1
        let freq = 2047 - (self.io[0x13] as u16 | ((self.io[0x14] as u16) << 8)) % 2048;
        let duty = (self.io[0x11] >> 6) as usize;
        if self.apu_pulse1_freq_counter < freq * 2 {
            self.apu_pulse1_freq_counter += 1;
        } else {
            self.apu_pulse1_freq_counter = 0;
            self.apu_pulse1_duty_step = (self.apu_pulse1_duty_step + 1) % 8;
        }
        // Update channel 1 sound output state
        if self.apu_pulse1_enabled {
            self.apu_sound_output[0] =
                DUTY_CYCLES[duty][self.apu_pulse1_duty_step] * self.apu_pulse1_curr_volume * 8;
        } else {
            self.apu_sound_output[0] = 0;
        }
        // Add channel 1 to apu buffer left
        if self.io[0x25] & 0b0001_0000 > 0 {
            self.apu_buffer[self.apu_buffer_to_use][(self.apu_buffer_write_index >> 6) * 2] +=
                ((self.apu_sound_output[0] as u16) * (16 * ((self.io[0x24] >> 4) & 0x07) as u16))
                    / 64;
        }
        // Add channel 1 to apu buffer right
        if self.io[0x25] & 0b0000_0001 > 0 {
            self.apu_buffer[self.apu_buffer_to_use][(self.apu_buffer_write_index >> 6) * 2 + 1] +=
                ((self.apu_sound_output[0] as u16) * (16 * ((self.io[0x24]) & 0x07) as u16)) / 64;
        }
    }

    fn handle_apu_channel_2(&mut self) {
        if self.io[0x25] & 0b0010_0010 == 0 {
            self.apu_sound_output[1] = 0;
            return;
        }
        // Handle frequency of channel 2
        let freq = 2047 - (self.io[0x13 + 5] as u16 | ((self.io[0x14 + 5] as u16) << 8)) % 2048;
        let duty = (self.io[0x11 + 5] >> 6) as usize;

        if self.apu_pulse2_freq_counter < freq * 2 {
            self.apu_pulse2_freq_counter += 1;
        } else {
            self.apu_pulse2_freq_counter = 0;
            self.apu_pulse2_duty_step = (self.apu_pulse2_duty_step + 1) % 8;
        }
        // Update channel 2 sound output state
        if self.apu_pulse2_enabled {
            self.apu_sound_output[1] =
                DUTY_CYCLES[duty][self.apu_pulse2_duty_step] * self.apu_pulse2_curr_volume * 8;
        } else {
            self.apu_sound_output[1] = 0;
        }
        // Add channel 2 to apu buffer left
        if self.io[0x25] & 0b0010_0000 > 0 {
            self.apu_buffer[self.apu_buffer_to_use][(self.apu_buffer_write_index >> 6) * 2] +=
                ((self.apu_sound_output[1] as u16) * (16 * ((self.io[0x24] >> 4) & 0x07) as u16))
                    / 64;
        }
        // Add channel 2 to apu buffer right
        if self.io[0x25] & 0b0000_0010 > 0 {
            self.apu_buffer[self.apu_buffer_to_use][(self.apu_buffer_write_index >> 6) * 2 + 1] +=
                ((self.apu_sound_output[1] as u16) * (16 * ((self.io[0x24]) & 0x07) as u16)) / 64;
        }
    }

    fn handle_apu_channel_3(&mut self) {
        if self.io[0x25] & 0b0100_0100 == 0 {
            self.apu_sound_output[2] = 0;
            return;
        }

        if self.io[0x1A] == 0 {
            self.apu_sound_output[2] = 0;
            return;
        }

        // Handle frequency of channel 3
        let freq = 2047 - (self.io[0x1D] as u16 | ((self.io[0x1E] as u16) << 8)) % 2048;

        if self.apu_wave_freq_counter < freq {
            self.apu_wave_freq_counter += 1;
        } else {
            self.apu_wave_freq_counter = 0;
            self.apu_wave_duty_step = (self.apu_wave_duty_step + 1) % 32;
        }
        // Update channel 3 sound output state
        if self.apu_wave_enabled {
            let mut curr_sample_byte = self.io[0x30 + self.apu_wave_duty_step / 2];
            if self.apu_wave_duty_step % 2 == 0 {
                curr_sample_byte >>= 4;
            }
            self.apu_sound_output[2] = (0x0F - (curr_sample_byte & 0x0F))
                * [0, 8, 4, 2][(self.io[0x1C] as usize) >> 5 & 0b11];
        } else {
            self.apu_sound_output[2] = 0;
        }
        // Add channel 3 to apu buffer left
        if self.io[0x25] & 0b0100_0000 > 0 {
            self.apu_buffer[self.apu_buffer_to_use][(self.apu_buffer_write_index >> 6) * 2] +=
                ((self.apu_sound_output[2] as u16) * (16 * ((self.io[0x24] >> 4) & 0x07) as u16))
                    / 64;
        }
        // Add channel 3 to apu buffer right
        if self.io[0x25] & 0b0000_0100 > 0 {
            self.apu_buffer[self.apu_buffer_to_use][(self.apu_buffer_write_index >> 6) * 2 + 1] +=
                ((self.apu_sound_output[2] as u16) * (16 * ((self.io[0x24]) & 0x07) as u16)) / 64;
        }
    }

    fn handle_apu_channel_4(&mut self) {
        if self.io[0x25] & 0b1000_1000 == 0 {
            self.apu_sound_output[3] = 0;
            return;
        }
        // Handle frequency of channel 4
        let divisor_code = self.io[0x22] & 0b00000111;
        let clock_shift = self.io[0x22] >> 4;
        let freq = ([8, 16, 32, 48, 64, 80, 96, 112][divisor_code as usize] << (clock_shift)) >> 2;

        if self.apu_noise_freq_counter < (freq as u16) * 2 {
            self.apu_noise_freq_counter += 1;
        } else {
            self.apu_noise_freq_counter = 0;

            // Perform shifty magic
            let mut a = self.apu_noise_duty_step & 0b1;
            self.apu_noise_duty_step >>= 1;
            a ^= self.apu_noise_duty_step & 0b1;
            self.apu_noise_duty_step |= a << 14;
            if self.io[0x22] & (1 << 3) > 0 {
                self.apu_noise_duty_step = self.apu_noise_duty_step & !(1 << 6) | a << 6;
            }
        }
        // Update channel 4 sound output state
        if self.apu_noise_enabled {
            self.apu_sound_output[3] =
                (!self.apu_noise_duty_step & 0x01) as u8 * self.apu_noise_curr_volume * 8;
        } else {
            self.apu_sound_output[3] = 0;
        }
        // Add channel 4 to apu buffer left
        if self.io[0x25] & 0b1000_0000 > 0 {
            self.apu_buffer[self.apu_buffer_to_use][(self.apu_buffer_write_index >> 6) * 2] +=
                ((self.apu_sound_output[3] as u16) * (16 * ((self.io[0x24] >> 4) & 0x07) as u16))
                    / 64;
        }
        // Add channel 4 to apu buffer right
        if self.io[0x25] & 0b0000_1000 > 0 {
            self.apu_buffer[self.apu_buffer_to_use][(self.apu_buffer_write_index >> 6) * 2 + 1] +=
                ((self.apu_sound_output[3] as u16) * (16 * ((self.io[0x24]) & 0x07) as u16)) / 64;
        }
    }

    fn handle512_channel_1(&mut self) {
        // Get flags
        let length_enable_flag1: bool = self.io[0x14] & (1 << 6) > 0;
        // Handle length
        if [0, 2, 4, 6].contains(&(self.apu_clock % 8))
            && length_enable_flag1
            && self.apu_pulse1_length_timer > 0
        {
            self.apu_pulse1_length_timer -= 1;
            if self.apu_pulse1_length_timer == 0 {
                self.apu_pulse1_enabled = false;
            }
        }
        // Handle volume
        {
            let vol_env_reg = self.io[0x12];
            if self.apu_clock % 8 == 7 {
                self.apu_pulse1_env_counter = self.apu_pulse1_env_counter.overflowing_add(1).0;
                if self.apu_pulse1_env_counter == vol_env_reg & 0b00000111
                    && vol_env_reg & 0b00000111 != 0
                {
                    self.apu_pulse1_env_counter = 0;
                    if vol_env_reg & 0b00001000 == 0 {
                        // ha this is convenient, thanks rust
                        self.apu_pulse1_curr_volume = self.apu_pulse1_curr_volume.saturating_sub(1);
                    } else {
                        self.apu_pulse1_curr_volume += 1;
                        self.apu_pulse1_curr_volume = self.apu_pulse1_curr_volume.clamp(0, 15);
                    }
                }
            }
        };
        // Handle sweep
        self.handle_sweep();
    }

    fn handle512_channel_2(&mut self) {
        // Get flags
        let length_enable_flag2: bool = self.io[0x19] & (1 << 6) > 0;
        // Handle length
        if [0, 2, 4, 6].contains(&(self.apu_clock % 8))
            && length_enable_flag2
            && self.apu_pulse2_length_timer > 0
        {
            self.apu_pulse2_length_timer -= 1;
            if self.apu_pulse2_length_timer == 0 {
                self.apu_pulse2_enabled = false;
            }
        }
        // Handle volume
        {
            let vol_env_reg = self.io[0x17];
            if self.apu_clock % 8 == 7 {
                self.apu_pulse2_env_counter += 1;
                if self.apu_pulse2_env_counter == vol_env_reg & 0b00000111
                    && vol_env_reg & 0b00000111 != 0
                {
                    self.apu_pulse2_env_counter = 0;
                    if vol_env_reg & 0b00001000 == 0 {
                        // ha this is convenient, thanks rust
                        self.apu_pulse2_curr_volume = self.apu_pulse2_curr_volume.saturating_sub(1);
                    } else {
                        self.apu_pulse2_curr_volume += 1;
                        self.apu_pulse2_curr_volume = self.apu_pulse2_curr_volume.clamp(0, 15);
                    }
                }
            }
        };
    }

    fn handle512_channel_3(&mut self) {
        // Get flags
        let length_enable_flag3: bool = self.io[0x14] & (1 << 6) > 0;
        // Handle length
        if [0, 2, 4, 6].contains(&(self.apu_clock % 8))
            && length_enable_flag3
            && self.apu_wave_length_timer > 0
        {
            self.apu_wave_length_timer -= 1;
            if self.apu_wave_length_timer == 0 {
                self.apu_wave_enabled = false;
            }
        }
    }

    fn handle512_channel_4(&mut self) {
        // Get flags
        let length_enable_flag4: bool = self.io[0x23] & (1 << 6) > 0;
        // Handle length
        if [0, 2, 4, 6].contains(&(self.apu_clock % 8))
            && length_enable_flag4
            && self.apu_noise_length_timer > 0
        {
            self.apu_noise_length_timer -= 1;
            if self.apu_noise_length_timer == 0 {
                self.apu_noise_enabled = false;
            }
        }
        // Handle volume
        {
            let vol_env_reg = self.io[0x21];
            if self.apu_clock % 8 == 7 {
                self.apu_noise_env_counter += 1;
                if self.apu_noise_env_counter == vol_env_reg & 0b00000111
                    && vol_env_reg & 0b00000111 != 0
                {
                    self.apu_noise_env_counter = 0;
                    if vol_env_reg & 0b00001000 == 0 {
                        // ha this is convenient, thanks rust
                        self.apu_noise_curr_volume = self.apu_noise_curr_volume.saturating_sub(1);
                    } else {
                        self.apu_noise_curr_volume += 1;
                        self.apu_noise_curr_volume = self.apu_noise_curr_volume.clamp(0, 15);
                    }
                }
            }
        };
    }

    pub(in super::super) fn handle_sweep(&mut self) {
        let freq = (self.io[0x13] as u16) | (((self.io[0x14] & 0b111) as u16) << 8);
        if [2, 6].contains(&(self.apu_clock % 8)) {
            // Tick timer
            if (self.io[0x10] & 0b01110000) >> 4 == 0 {
                return;
            }

            self.apu_pulse1_sweep_timer += 1;
            if self.apu_pulse1_sweep_timer < (self.io[0x10] & 0b01110000) >> 4 {
                return;
            }

            // If shift amount isn't 0 and enabled flag is set
            if self.io[0x10] & 0b00000111 != 0 && self.apu_pulse1_sweep_enable {
                self.apu_pulse1_sweep_timer = 0;
                // Shift
                let shift_amount = self.io[0x10] & 0b00000111;
                let mut new_frequency = freq >> shift_amount;

                // Negate flag - sum
                if self.io[0x10] & 0b00001000 > 0 {
                    new_frequency = freq.saturating_sub(new_frequency);
                } else {
                    new_frequency = freq.saturating_add(new_frequency);
                }

                // Overflow check
                if new_frequency > 2047 {
                    self.apu_pulse1_enabled = false;
                } else if shift_amount != 0 {
                    // Write frequency
                    self.io[0x13] = (new_frequency & 0xFF) as u8;
                    self.io[0x14] =
                        self.io[0x14] & 0b11111000 | ((new_frequency >> 8) as u8) & 0b00000111;
                    self.apu_pulse1_sweep_shadow_freq = new_frequency;
                }
            }
        }
    }
}
