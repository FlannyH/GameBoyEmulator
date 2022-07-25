use crate::gameboy::GameBoy;
use minifb::{Key, Window};

impl GameBoy {
    pub(crate) fn update_input(&mut self, window: &Window) {
        let mut new_input = 0;
        for key in [
            /* Down   */Key::Down,
            /* Up     */Key::Up,
            /* Left   */Key::Left,
            /* Right  */Key::Right,
            /* Start  */Key::Enter,
            /* Select */Key::RightShift,
            /* B      */Key::Z,
            /* A      */Key::X,
        ] {
            new_input <<= 1;
            if window.is_key_down(key) {
                new_input |= 0x01;
            }
        }
        self.joypad_state = !(new_input as u8);
    }
}
