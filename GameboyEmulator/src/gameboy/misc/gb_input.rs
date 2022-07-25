use crate::gameboy::GameBoy;
use minifb::{Key, Window};

impl GameBoy {
    pub(crate) fn update_input(&mut self, window: &Window) {
        let mut new_input = 0;
        for key in [
            Key::Right,
            Key::Left,
            Key::Up,
            Key::Down,
            Key::Enter,
            Key::RightShift,
            Key::X,
            Key::Z,
        ] {
            new_input <<= 1;
            if window.is_key_down(key) {
                new_input |= 0x01;
            }
        }
        self.joypad_state = !(new_input as u8);
        // todo: delete
        if self.joypad_state & 0b00001000 == 0 {
            self.debug_enabled = true;
        }
    }
}
