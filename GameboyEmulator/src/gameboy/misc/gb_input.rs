use crate::gameboy::GameBoy;
use mini_gl_fb::{glutin::event::VirtualKeyCode, BasicInput};

impl GameBoy {
    pub(crate) fn update_input(&mut self, input: &BasicInput) {
        let mut new_input = 0;
        for key in [
            /* Down   */ VirtualKeyCode::Down,
            /* Up     */ VirtualKeyCode::Up,
            /* Left   */ VirtualKeyCode::Left,
            /* Right  */ VirtualKeyCode::Right,
            /* Start  */ VirtualKeyCode::Return,
            /* Select */ VirtualKeyCode::RShift,
            /* B      */ VirtualKeyCode::Z,
            /* A      */ VirtualKeyCode::X,
        ] {
            new_input <<= 1;
            if input.key_is_down(key) {
                new_input |= 0x01;
            }
        }
        self.joypad_state = !(new_input as u8);
    }
}
