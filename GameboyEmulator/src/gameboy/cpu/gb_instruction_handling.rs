use std::io::{self, Read};

use crate::gameboy::FlagMask;

use super::super::GameBoy;

impl GameBoy {
    pub(in crate) fn run_frame(&mut self) {
        loop {
            let prev = self.ppu_ly;
            self.process_next_instruction();
            //self.print_reg_state();
            self.run_ppu_cycle();
            self.run_ppu_cycle();

            if self.oam_dma_counter > 0 {
                self.oam[160 - self.oam_dma_counter as usize] = self.fetch_byte_from_memory(
                    self.oam_dma_source + (160 - self.oam_dma_counter as u16),
                );
                self.oam_dma_counter -= 1;
            }

            let mut _stdin = io::stdin();
            if prev != self.ppu_ly && self.ppu_ly % 144 == 0 {
                break;
            }
            if (self.io[0x40] & 0x80 == 0) && (self.timer_div < 3) {
                //break;
            }
            if self.debug_enabled && (self.new_instruction_tick) {
                if self.rom_chip_enabled == self.debug_bios {
                    println!("Opcode: ${:02X}, PC: ${:04X}", self.last_opcode, self.pc);
                    self.print_reg_state();
                    if self.debug_require_input {
                        let _ = _stdin.read(&mut [0u8]).unwrap();
                        let _ = _stdin.read(&mut [0u8]).unwrap();
                    }
                }
                //if self.rom_chip_enabled == self.debug_bios {
                //    break;
                //}
            }
        }
    }

    pub(in super::super) fn process_next_instruction(&mut self) {
        // Update timer
        self.handle_timer();

        // Wait for previous instruction to finish
        self.new_instruction_tick = false;
        if self.curr_cycles_to_wait > 1 {
            self.curr_cycles_to_wait -= 1;
            return;
        }
        if self.curr_cycles_to_wait > 0 {
            self.curr_cycles_to_wait -= 1;
            self.new_instruction_tick = true;
        }

        // Interrupts
        self.handle_interrupts();

        // If halted, stop here
        if self.is_halted {
            return;
        }

        // Read byte from PC
        let opcode = self.fetch_next_byte_from_pc();
        self.last_opcode = opcode;

        // Pass it to a bunch of functions, let them handle it. If none of them handle it, this is an invalid opcode, and we should hang.
        if self.handle_misc_instructions(opcode) {
            self.last_opcode_cycles = self.curr_cycles_to_wait;
            return;
        }
        if self.handle_load_instructions(opcode) {
            self.last_opcode_cycles = self.curr_cycles_to_wait;
            return;
        }
        if self.handle_arithmetic_instructions(opcode) {
            self.last_opcode_cycles = self.curr_cycles_to_wait;
            return;
        }
        if self.handle_branch_instructions(opcode) {
            self.last_opcode_cycles = self.curr_cycles_to_wait;
            return;
        }
        if self.handle_incdec_instructions(opcode) {
            self.last_opcode_cycles = self.curr_cycles_to_wait;
            return;
        }

        // If we get here, we have an instruction that we don't know how to process
        println!("--Opcode ${:02X} not implemented!--", opcode);
        self.print_reg_state();
        panic!();
    }

    pub(in super::super) fn handle_misc_instructions(&mut self, opcode: u8) -> bool {
        // These are all the special cases that don't fit neatly into one category
        match opcode {
            0x00 => return true, // NOP - no operation
            0x07 => {
                self.reg_a = self.rlc(self.reg_a);
                self.reg_f &= FlagMask::CARRY as u8;
            }
            0x0F => {
                self.reg_a = self.rrc(self.reg_a);
                self.reg_f &= FlagMask::CARRY as u8;
            }
            0x10 => {
                self.fetch_next_byte_from_pc();
            } // STOP
            0x17 => {
                self.reg_a = self.rl(self.reg_a);
                self.reg_f &= FlagMask::CARRY as u8;
            }
            0x1F => {
                self.reg_a = self.rr(self.reg_a);
                self.reg_f &= FlagMask::CARRY as u8;
            }
            0x76 => self.is_halted = true,
            0xCB => {
                self.handle_prefixed_instructions(opcode);
            } // CB - prefixed instructions mostly for bit shifting, setting, and clearing
            0xF3 => self.ime = 0,
            0xFB => self.ime = 1,
            _ => return false,
        }
        return true;
    }
}
